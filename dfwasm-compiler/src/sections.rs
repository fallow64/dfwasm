use std::rc::Rc;

use dfwasm_template::{Args, Block, Template};
use wasmparser::{
    Data, DataKind, ElementItems, ElementKind, Encoding, ExternalKind, Payload, TableInit, TypeRef,
};

use crate::{DFWasmError, DFWasmResult, df_helper::DF_VAR_EXPORTS};

use super::{
    DFWasmCompiler,
    compiler::ControlStackEntry,
    df_helper::{
        DF_FUNC_BATCH_DATA_SECTION, DF_VAR_STORE_FUNCS, DF_VAR_STORE_GLOBALS, DF_VAR_STORE_TABLES,
        format_df_number_u64, format_df_number_usize, generate_function_name, generate_table_name,
        num, string, var,
    },
    operators::compile_operator,
};

/// Compile a section of the WASM module.
///
/// WASM modules are divided into sections, each containing different types of data.
/// For example, the type section contains function signatures, the import section
/// contains imported functions, and the code section contains the actual
/// implementation of the functions.
pub fn compile_section(
    compiler: &mut DFWasmCompiler,
    module_template: &mut Template,
    section: Payload<'_>,
) -> DFWasmResult<()> {
    match section {
        Payload::Version { num, encoding, .. } => {
            // Check the version and encoding
            if num != 1 {
                return Err(DFWasmError::UnsupportedVersion(num));
            }
            if encoding == Encoding::Component {
                return Err(DFWasmError::UnsupportedComponents);
            }
        }
        Payload::TypeSection(section) => {
            // The TypeSection contains all the function signatures
            for ty in section {
                let ty = ty?;
                compiler.function_signatures.push(Rc::new(ty));
            }
        }
        Payload::ImportSection(section) => {
            // Get the imports for the module

            for import in section {
                let import = import?;

                match import.ty {
                    TypeRef::Func(type_idx) => {
                        // Get the function type
                        let func_type = compiler.function_signatures[type_idx as usize].clone();
                        compiler.function_to_type_signature.push(func_type);
                        compiler.function_counter += 1;

                        // todo: better way of handling imports
                        // Add the function to the module template
                        module_template.set_var(
                            "AppendValue",
                            Args::with(vec![
                                var(DF_VAR_STORE_FUNCS),
                                string(format!("{}/{}", import.module, import.name)),
                            ]),
                        );
                    }
                    TypeRef::Table(..) => todo!("import table"),
                    TypeRef::Memory(memory_type) => {
                        // Check if a memory is already defined
                        if compiler.memory_type.is_some() {
                            return Err(DFWasmError::MultipleMemories);
                        }
                        // Set the memory type
                        compiler.memory_type = Some(memory_type);
                    }
                    TypeRef::Global(..) => todo!("import global"),
                    TypeRef::Tag(..) => todo!("import tag (?)"),
                }
            }
        }
        Payload::FunctionSection(section) => {
            // Correlate each function index to its type index
            for ty_index in section {
                let ty_index = ty_index?;

                // get the function's actual type
                let func_type = compiler.function_signatures[ty_index as usize].clone();

                // store the function type
                compiler.function_to_type_signature.push(func_type);
            }
        }
        Payload::TableSection(section) => {
            // Add the tables to the module template

            for table in section {
                let table = table?;

                if table.ty.initial > 10000 {
                    return Err(DFWasmError::MaximumTableSize);
                }

                let init_value = match table.init {
                    TableInit::RefNull => num(-1),
                    TableInit::Expr(const_expr) => compiler.eval_const_expr(const_expr)?,
                };
                compiler.table_to_init_expr.push(init_value.clone());

                let table_idx = compiler.table_counter;
                compiler.table_counter += 1;

                let table_name = generate_table_name(table_idx);

                // The table store contains "pointers" to each table
                // i.e. it contains strings like "$table_0", "$table_1", etc.
                // And the variable $table_0 contains the actual table.
                module_template.set_var(
                    "AppendValue",
                    Args::with(vec![var(DF_VAR_STORE_TABLES), string(table_name.clone())]),
                );

                // Initialize the table with it's initial value
                module_template
                    .repeat("Multiple", Args::with(vec![num(table.ty.initial)]))
                    .open_bracket_repeat()
                    .set_var("AppendValue", Args::with(vec![var(table_name), init_value]))
                    .close_bracket_repeat();
            }
        }
        Payload::MemorySection(section) => {
            // Store the memory and its type
            // There can only be *one* memory per module, and memories can be defined by
            // 1. (memory $m0 initialSize maxSize?)
            // 2. (memory $m0 (import "mem" "env") initialSize maxSize?)
            // Option #1 is in MemorySection, and option #2 is in ImportSection.
            for memory in section {
                let memory = memory?;

                if compiler.memory_type.is_some() {
                    return Err(DFWasmError::MultipleMemories);
                }

                compiler.memory_type = Some(memory);
            }
        }
        Payload::GlobalSection(section) => {
            // Get all globals, their types, and whether or not they're mutable
            for global in section {
                let global = global?;
                // todo: should we handle immutable/mutability here?

                // Get the initial value
                let init = compiler.eval_const_expr(global.init_expr)?;

                // Register it in the globals store of the module template
                module_template.set_var(
                    "AppendValue",
                    Args::with(vec![var(DF_VAR_STORE_GLOBALS), init]),
                );
            }
        }
        Payload::ExportSection(section) => {
            // Create the exports dictionary
            module_template.set_var("CreateDict", Args::with(vec![var(DF_VAR_EXPORTS)]));

            for export in section {
                let export = export?;

                match export.kind {
                    ExternalKind::Func => {
                        // Get the function index
                        let func_index = export.index as usize;

                        // Add the function to the module template
                        module_template.set_var(
                            "SetDictValue",
                            Args::with(vec![
                                var(DF_VAR_EXPORTS),
                                string(export.name),
                                num(func_index),
                            ]),
                        );
                    }
                    ExternalKind::Table
                    | ExternalKind::Memory
                    | ExternalKind::Global
                    | ExternalKind::Tag => {}
                }
            }
        }
        Payload::StartSection { func, .. } => {
            if compiler.start_method.is_some() {
                return Err(DFWasmError::MultipleStartMethods);
            }
            compiler.start_method = Some(func as usize);
        }
        Payload::ElementSection(section) => {
            // Elements are used to initialize tables
            for element in section {
                let element = element?;

                let (table_index, offset_into_table) = match element.kind {
                    ElementKind::Active {
                        table_index,
                        offset_expr,
                    } => (
                        table_index.unwrap_or(0) as usize,
                        compiler.eval_const_expr_as_offset(&offset_expr)?,
                    ),
                    ElementKind::Passive | ElementKind::Declared => {
                        // todo: what is this?
                        return Err(DFWasmError::NotYetImplemented("passive/declared elements"));
                    }
                };

                let table_name: String = generate_table_name(table_index);
                match element.items {
                    ElementItems::Functions(function_section) => {
                        for (table_element_index, function_idx) in
                            (offset_into_table..).zip(function_section)
                        {
                            let function_idx = function_idx?;

                            // Set the table element to the function index
                            module_template.set_var(
                                "SetListValue",
                                Args::with(vec![
                                    var(table_name.clone()),
                                    num(table_element_index + 1),
                                    num(function_idx),
                                ]),
                            );
                        }
                    }
                    ElementItems::Expressions(..) => {
                        return Err(DFWasmError::NotYetImplemented(
                            "expression elements for tables",
                        ));
                    }
                }
            }
        }
        Payload::DataCountSection { .. } => {}
        Payload::DataSection(section) => {
            // The data section contains predefined data to be stored in memory

            for data_definition in section {
                let data_definition = data_definition?;

                compile_data_initialization(compiler, module_template, data_definition)?;
            }
        }
        Payload::CodeSectionStart { .. } => {}
        Payload::CodeSectionEntry(function_body) => {
            let func_id = compiler.function_counter;
            compiler.function_counter += 1;

            let func_name = generate_function_name(
                func_id,
                compiler.options.module_name.as_ref().map(String::as_str),
            );

            // Append the function to the module template
            module_template.blocks.push(Block::SetVariable {
                args: Args::with(vec![var(DF_VAR_STORE_FUNCS), string(func_name.clone())]),
                action: "AppendValue".to_string(),
            });

            // Create the function template
            compiler
                .templates
                .push(Template::start_function(func_name.clone()));

            // Push to the control stack
            compiler
                .control_stack
                .push(ControlStackEntry::FunctionStart(
                    // this is not the function index but rather the index of the template
                    compiler.templates.len() - 1,
                ));

            let operators_reader = function_body.get_operators_reader()?;
            for op in operators_reader.into_iter_with_offsets() {
                let (op, location) = op?;

                // Compile the operator.
                compile_operator(compiler, op, location)?;
            }
        }

        // Sections for WebAssembly components
        // This is not within the scope of DFWasm
        Payload::ModuleSection { .. }
        | Payload::InstanceSection(..)
        | Payload::CoreTypeSection(..)
        | Payload::ComponentSection { .. }
        | Payload::ComponentInstanceSection(..)
        | Payload::ComponentAliasSection(..)
        | Payload::ComponentTypeSection(..)
        | Payload::ComponentCanonicalSection(..)
        | Payload::ComponentStartSection { .. }
        | Payload::ComponentImportSection(..)
        | Payload::ComponentExportSection(..) => {
            return Err(DFWasmError::UnsupportedComponents);
        }
        Payload::TagSection(..) => {
            return Err(DFWasmError::UnsupportedTags);
        }

        Payload::CustomSection(..) => {
            // As of right now, no custom sections are defined.
            // Maybe this could be used to somehow use DiamondFire blocks directly?
        }
        Payload::UnknownSection { id, .. } => {
            return Err(DFWasmError::UnknownSection(id));
        }
        Payload::End(_) => {}
        _ => return Err(DFWasmError::UnknownPayload),
    }

    Ok(())
}

fn compile_data_initialization(
    compiler: &mut DFWasmCompiler,
    module_template: &mut Template,
    data_definition: Data<'_>,
) -> DFWasmResult<()> {
    let offset_expr = match data_definition.kind {
        DataKind::Passive => return Err(DFWasmError::NotYetImplemented("passive data sections")),
        DataKind::Active {
            memory_index: _,
            offset_expr,
        } => offset_expr,
    };

    let offset = compiler.eval_const_expr_as_offset(&offset_expr)?;

    match compiler.options.batch_data_size {
        Some(batch_size) => {
            let mut initial_memory_address = offset;
            let mut buffer = Vec::new();

            for byte in data_definition.data {
                buffer.push(*byte);
                if buffer.len() == batch_size {
                    let mut args = vec![num(format_df_number_usize(initial_memory_address))];
                    args.extend(
                        buffer
                            .iter()
                            .map(|byte| num(format_df_number_u64((*byte).into())))
                            .collect::<Vec<_>>(),
                    );

                    // Call the batch function
                    module_template.call_function(DF_FUNC_BATCH_DATA_SECTION, Args::with(args));

                    // Reset the buffer and bump the address
                    buffer.clear();
                    initial_memory_address += batch_size;
                }
            }

            // Clear out the buffer
            if !buffer.is_empty() {
                let mut args = vec![num(format_df_number_usize(initial_memory_address))];

                args.extend(
                    buffer
                        .iter()
                        .map(|byte| num(format_df_number_u64((*byte).into())))
                        .collect::<Vec<_>>(),
                );

                // Call the batch function
                module_template.call_function(DF_FUNC_BATCH_DATA_SECTION, Args::with(args));

                // Reset the buffer and bump the address
                buffer.clear();
            }
        }
        None => {
            // Append the data, one block for one byte
            for (i, byte) in data_definition.data.iter().enumerate() {
                let memory_address = format_df_number_usize(offset + i);
                module_template.set_var(
                    "=",
                    Args::with(vec![
                        var(format!("$mem_{memory_address}")),
                        num(format_df_number_u64((*byte).into())),
                    ]),
                );
            }
        }
    }

    Ok(())
}
