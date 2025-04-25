use std::{rc::Rc, vec};

use dfwasm_template::{Args, Block, Item, Template, split_templates};
use wasmparser::{ConstExpr, MemoryType, Operator, Parser, RecGroup};

use crate::{
    DFWasmError, DFWasmResult,
    df_helper::{DF_VAR_MEM_SIZE, DF_VAR_MODULE_INIT_FUNC, format_df_number_u64, string, var},
};

use super::{
    df_helper::{DF_FUNC_CALL_FUNC, format_df_number_i64, num},
    sections::compile_section,
};

/// An entry in the control stack when compiling.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ControlStackEntry {
    FunctionStart(usize),
    Block(usize),
    Loop(usize),
    If(usize),
    Else(usize),
}

/// The options for [`DFWasmCompiler`].
#[derive(Default, Debug, Clone)]
pub struct DFWasmCompilerOptions {
    /// The module name. Useful for when compiling multiple modules.
    pub module_name: Option<String>,
    /// Whether or not to include debugger function calls.
    pub debugger: bool,
    /// Whether or not to call the debugger on nops.
    pub skip_nop_debugger: bool,
    /// Whether or not to limit the size of the templates (highly recommended).
    /// Use this to set your plot size.
    pub max_template_size: Option<usize>,
    /// Whether or not to batch the data section memory setters into the $batchDataSection function.
    /// May be useful for large data sections.
    pub batch_data: bool,
    /// The number of bytes to batch into the $batchDataSection function.
    /// By default, this is 26 bytes, and is also the maximum value.
    ///
    /// Due to how DiamondFire works, a higher number may lead to an issue where you
    /// can't break the template due to the maximum template data size limit.
    ///
    /// Experimentally, for massive plots and very large data sections, 8 seems to be the maximum.
    /// Basic and large plots could get away with the default.
    ///
    /// If you don't care about deleting the templates, the maximum will also work fine.
    /// It will just inconvenience you if you delete them without /plot clear.
    pub batch_data_size: Option<usize>,
    /// Whether or not to only include the module init function in the template.
    /// Useful for debugging data section initialization.
    pub only_include_module_init: bool,
}

/// The WASM to DiamondFire compiler.
pub struct DFWasmCompiler<'a> {
    /// The binary WASM file to compile
    pub wasm: &'a [u8],
    /// The options to use when compiling
    pub options: DFWasmCompilerOptions,

    /// The exported templates (also used as a working area for the compiler)
    pub(crate) templates: Vec<Template>,

    /// The currently defined memory type, if any.
    pub(crate) memory_type: Option<MemoryType>,

    /// The index of the start method, which is called when the module is loaded.
    pub(crate) start_method: Option<usize>,

    /// The stack of control flow operators (ifs, loops, blocks, etc)
    pub(crate) control_stack: Vec<ControlStackEntry>,

    /// A map front table index to the initialization expression for that table.
    pub(crate) table_to_init_expr: Vec<Item>,

    /// The current function being compiled. Also increased on imports.
    pub(crate) function_counter: usize,

    /// The current table being compiled. Also increased on imports.
    pub(crate) table_counter: usize,

    /// A map from function index to function signature.
    pub(crate) function_to_type_signature: Vec<Rc<RecGroup>>,

    /// The defined function signatures,
    pub(crate) function_signatures: Vec<Rc<RecGroup>>,
}

impl<'a> DFWasmCompiler<'a> {
    /// Compiles a WASM file into DiamondFire templates.
    pub fn compile_wasm(
        wasm: &'a [u8],
        options: DFWasmCompilerOptions,
    ) -> DFWasmResult<Vec<Template>> {
        let this = Self {
            wasm,
            options,
            templates: Vec::new(),
            memory_type: None,
            start_method: None,
            control_stack: Vec::new(),
            table_to_init_expr: Vec::new(),
            function_counter: 0,
            table_counter: 0,
            function_to_type_signature: Vec::new(),
            function_signatures: Vec::new(),
        };
        this.compile()
    }

    /// The main compile function. Goes through each section of the WASM file and compiles it into a template.
    fn compile(mut self) -> DFWasmResult<Vec<Template>> {
        // Create a new binary WASM parser (0 represents source offset)
        let parser = Parser::new(0);

        // Module template is what is called to initialize the module
        let module_template_name = match &self.options.module_name {
            Some(name) => format!("{}_module_init", name),
            None => "module_init".to_string(),
        };
        let mut module_template = Template::start_function(module_template_name.clone());
        module_template.set_var(
            "=",
            Args::with(vec![
                var(DF_VAR_MODULE_INIT_FUNC),
                string(module_template_name),
            ]),
        );

        // Iterate over every section in the WASM file
        for section in parser.parse_all(self.wasm) {
            let section = section?;
            compile_section(&mut self, &mut module_template, section)?;
        }

        // Add the start method to the module template if it exists
        if let Some(start_method) = self.start_method {
            module_template.blocks.push(Block::CallFunction {
                args: Args::with(vec![num(start_method), num(0), num(0)]),
                func: DF_FUNC_CALL_FUNC.to_string(),
            })
        }

        // Set the starting memory size
        if let Some(memory_type) = self.memory_type {
            module_template.set_var(
                "=",
                Args::with(vec![
                    var(DF_VAR_MEM_SIZE),
                    num(format_df_number_u64(memory_type.initial)),
                ]),
            );
        }

        if self.options.only_include_module_init {
            self.templates.clear();
        }

        self.templates.push(module_template);

        // Split the templates if they are too large
        if let Some(max_template_size) = self.options.max_template_size {
            Ok(split_templates(self.templates, max_template_size))
        } else {
            Ok(self.templates)
        }
    }

    /// Get the current working template index.
    pub(crate) fn get_current_template_idx(&self) -> Option<usize> {
        self.control_stack.last().map(|entry| match entry {
            ControlStackEntry::FunctionStart(template_idx)
            | ControlStackEntry::Block(template_idx)
            | ControlStackEntry::Loop(template_idx)
            | ControlStackEntry::If(template_idx)
            | ControlStackEntry::Else(template_idx) => *template_idx,
        })
    }

    // Get the current working template.
    pub(crate) fn get_current_template(&mut self) -> &mut Template {
        let template_idx = self.get_current_template_idx().expect("No template");

        self.templates
            .get_mut(template_idx)
            .expect("Template not found")
    }

    /// Evaluates a constant expression and returns the result as an Item.
    /// As opposed to [`eval_const_expr_as_offset`], this is used for things like
    /// table and global initializers.
    pub(crate) fn eval_const_expr(&self, expr: ConstExpr) -> DFWasmResult<Item> {
        let mut reader = expr.get_operators_reader();
        let op = reader.read()?;

        let result = match op {
            Operator::I32Const { value } => num(format_df_number_i64(value as i64)),
            Operator::I64Const { value } => num(format_df_number_i64(value)),
            Operator::F32Const { value: _ } => todo!("float constant expressions"),
            Operator::F64Const { value: _ } => todo!("float constant expressions"),
            Operator::RefNull { hty: _ } => num(-1),
            Operator::RefFunc { function_index } => num(function_index),
            Operator::GlobalGet { global_index: _ } => todo!("global get within a const expr"),
            _ => return Err(DFWasmError::UnsupportedConstExpr),
        };

        if reader.eof() {
            Ok(result)
        } else {
            match reader.read()? {
                Operator::End => Ok(result),
                // todo: should this be a separate error?
                _ => Err(DFWasmError::UnsupportedConstExpr),
            }
        }
    }

    /// Evaluates a constant expression as an offset.
    /// This is used when a compile-time known offset is needed, such as memory data section offsets,
    /// or element section offsets.
    pub(crate) fn eval_const_expr_as_offset(&self, expr: &ConstExpr) -> DFWasmResult<usize> {
        let mut reader = expr.get_operators_reader();
        let op = reader.read()?;

        let result = match op {
            Operator::I32Const { value } => usize::from_le_bytes((value as i64).to_le_bytes()),
            Operator::I64Const { value } => usize::from_le_bytes(value.to_le_bytes()),
            Operator::GlobalGet { global_index: _ } => todo!("global get within a const expr"),
            _ => return Err(DFWasmError::UnsupportedConstExpr),
        };

        if reader.eof() {
            Ok(result)
        } else {
            match reader.read()? {
                Operator::End => Ok(result),
                // todo: should this be a separate error?
                _ => Err(DFWasmError::UnsupportedConstExpr),
            }
        }
    }

    /// Get the argument count of a function type.
    pub(crate) fn arg_count_of_type(signature: &RecGroup) -> Option<usize> {
        Some(signature.types().next()?.unwrap_func().params().len())
    }

    /// Get the result count of a function type.
    /// In WASM, a function can have multiple results.
    pub(crate) fn result_count_of_type(signature: &RecGroup) -> Option<usize> {
        Some(signature.types().next()?.unwrap_func().results().len())
    }
}
