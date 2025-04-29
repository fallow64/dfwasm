use anyhow::{Result, anyhow};
use dfwasm_template::{Args, CodeBlock, Item, Template};
use wasmer::{Instance, Store, Value};

pub struct CompiledModuleTest {
    pub main_function_name: String,
    pub templates: Vec<Template>,
}

pub struct TestCase {
    pub module_name: String,
    pub function_name: String,
    pub inputs: Vec<String>,
}

pub fn parse_string_to_wasmer_value(str_value: &str, ty: wasmer::Type) -> Result<Value> {
    match ty {
        wasmer::Type::I32 => {
            Ok(Value::from(str_value.parse::<i32>().unwrap_or_else(|e| {
                panic!("Invalid i32: {str_value}. Error: {e}")
            })))
        }
        wasmer::Type::I64 => {
            Ok(Value::from(str_value.parse::<i64>().unwrap_or_else(|e| {
                panic!("Invalid i64: {str_value}. Error: {e}")
            })))
        }
        _ => Err(anyhow!("Unsupported type")),
    }
}

pub fn parse_string_to_df_value(str_value: &str, ty: wasmer::Type) -> Result<Item> {
    match ty {
        wasmer::Type::I32 => Ok(Item::num(format_df_number_i32(
            str_value.parse::<i32>().expect("Invalid i32"),
        ))),
        wasmer::Type::I64 => Ok(Item::num(format_df_number_i64(
            str_value.parse::<i64>().expect("Invalid i64"),
        ))),
        _ => Err(anyhow!("Unsupported type")),
    }
}

pub fn wasmer_value_to_df_value(value: &Value) -> Result<Item> {
    match value {
        Value::I32(i) => Ok(Item::num(format_df_number_i32(*i))),
        Value::I64(i) => Ok(Item::num(format_df_number_i64(*i))),
        _ => Err(anyhow!("Unsupported type")),
    }
}

pub fn clear_variables(template: &mut Template) {
    template.set_var(
        "PurgeVars",
        Args::with_tags(
            vec![Item::string("wasm.$")],
            vec![
                Item::Tag {
                    option: "Any part of name".to_string(),
                    tag: "Match Requirement".to_string(),
                    action: "PurgeVars".to_string(),
                    block: CodeBlock::SetVariable,
                },
                Item::Tag {
                    option: "False".to_string(),
                    tag: "Ignore Case".to_string(),
                    action: "PurgeVars".to_string(),
                    block: CodeBlock::SetVariable,
                },
            ],
        ),
    );
}

pub fn format_df_number_i64(value: i64) -> String {
    let abs = value.unsigned_abs();
    let int_part = abs / 1000;
    let frac_part = abs % 1000;

    let sign = if value < 0 { "-" } else { "" };

    // Format the fractional part as a 3-digit number, then trim trailing zeros
    let mut frac_str = format!("{frac_part:03}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }

    if frac_str.is_empty() {
        format!("{sign}{int_part}",)
    } else {
        format!("{sign}{int_part}.{frac_str}")
    }
}

/// Formats an i32 number to a string.
pub fn format_df_number_i32(value: i32) -> String {
    let value_bytes: [u8; 4] = value.to_le_bytes();
    let value_bytes: [u8; 8] = [
        value_bytes[0],
        value_bytes[1],
        value_bytes[2],
        value_bytes[3],
        0,
        0,
        0,
        0,
    ];

    let value = i64::from_le_bytes(value_bytes);

    let abs = value.unsigned_abs();
    let int_part = abs / 1000;
    let frac_part = abs % 1000;

    let sign = if value < 0 { "-" } else { "" };

    // Format the fractional part as a 3-digit number, then trim trailing zeros
    let mut frac_str = format!("{frac_part:03}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }

    if frac_str.is_empty() {
        format!("{sign}{int_part}",)
    } else {
        format!("{sign}{int_part}.{frac_str}")
    }
}

/// Returns `(inputs, expected results)`
pub fn get_wasmer_results(
    case: &TestCase,
    store: &mut Store,
    wasmer_instance: &mut Instance,
) -> Result<(Vec<Item>, Vec<Item>)> {
    // First, get the expected output(s)

    // Parse the module input types
    let export_type = wasmer_instance
        .module()
        .exports()
        .functions()
        .find(|f| f.name() == case.function_name)
        .ok_or_else(|| {
            anyhow!(
                "Function {:?} not found in module exports",
                case.function_name
            )
        })?;

    let function_type = export_type.ty();

    // Parse the inputs as the correct type
    let input_types = function_type.params();
    let inputs = case
        .inputs
        .iter()
        .zip(input_types)
        .map(|(str_value, ty)| parse_string_to_wasmer_value(str_value, *ty))
        .collect::<Result<Vec<_>>>()?;

    // Call the function, and now we have our expected output
    let result = wasmer_instance
        .exports
        .get_function(&case.function_name)?
        .call(store, &inputs)?;

    // Convert the result to a DF value
    let df_results = result
        .iter()
        .map(wasmer_value_to_df_value)
        .collect::<Result<Vec<_>>>()?;

    // Convert the inputs to DF values
    let df_inputs = case
        .inputs
        .iter()
        .zip(input_types)
        .map(|(str_value, ty)| parse_string_to_df_value(str_value, *ty))
        .collect::<Result<Vec<_>>>()?;

    Ok((df_inputs, df_results))
}

/// Parses the input file (xyz.test) and returns a vector of test cases.
pub fn parse_input_file(module_name: &str, contents: &str) -> Vec<TestCase> {
    let mut cases = Vec::new();
    for line in contents.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let function_name = parts[0].to_string();
        let inputs = parts[1..]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();

        cases.push(TestCase {
            module_name: module_name.to_string(),
            function_name,
            inputs,
        });
    }
    cases
}
