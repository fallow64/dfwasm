use anyhow::{Result, anyhow};
use dfwasm_template::{Args, CodeBlock, Item, Template};
use wasmi::{Instance, Store, Val, core::ValType};

pub struct CompiledModuleTest {
    pub main_function_name: String,
    pub templates: Vec<Template>,
}

pub struct TestCase {
    pub module_name: String,
    pub function_name: String,
    pub inputs: Vec<String>,
}

pub fn parse_string_to_wasm_value(str_value: &str, ty: ValType) -> Result<Val> {
    match ty {
        ValType::I32 => Ok(Val::I32(
            str_value
                .parse::<i32>()
                .map_err(|_| anyhow!("Invalid i32"))?,
        )),
        ValType::I64 => Ok(Val::I64(
            str_value
                .parse::<i64>()
                .map_err(|_| anyhow!("Invalid i64"))?,
        )),
        _ => Err(anyhow!("Unsupported type")),
    }
}

pub fn parse_string_to_df_value(str_value: &str, ty: ValType) -> Result<Item> {
    match ty {
        ValType::I32 => Ok(Item::num(format_df_number_i32(
            str_value.parse::<i32>().expect("Invalid i32"),
        ))),
        ValType::I64 => Ok(Item::num(format_df_number_i64(
            str_value.parse::<i64>().expect("Invalid i64"),
        ))),
        _ => Err(anyhow!("Unsupported type")),
    }
}

pub fn wasm_value_to_df_value(value: &Val) -> Result<Item> {
    match value {
        Val::I32(i) => Ok(Item::num(format_df_number_i32(*i))),
        Val::I64(i) => Ok(Item::num(format_df_number_i64(*i))),
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
pub fn get_wasm_results(
    case: &TestCase,
    store: &mut Store<u32>,
    instance: &mut Instance,
) -> Result<(Vec<Item>, Vec<Item>)> {
    // Parse the module input types so we can convert our string test cases to the correct types
    let export_type = instance
        .get_export(&store, &case.function_name)
        .unwrap_or_else(|| panic!("Function {} not found in module", case.function_name))
        .ty(&store);
    let function_type = export_type
        .func()
        .unwrap_or_else(|| panic!("Export {} is not a function", case.function_name));

    let input_types = function_type.params();
    let result_types = function_type.results();

    // Parse the string inputs as WASM values
    let inputs = case
        .inputs
        .iter()
        .zip(input_types)
        .map(|(str_value, ty)| parse_string_to_wasm_value(str_value, *ty))
        .collect::<Result<Vec<_>>>()?;

    // Call the function, and now we have our expected output
    let mut results = vec![Val::I32(0); result_types.len()];
    instance
        .get_func(&store, &case.function_name)
        .expect("Function not found")
        .call(store, &inputs, results.as_mut())?;

    // Convert the result to a DF value
    let df_results = results
        .iter()
        .map(wasm_value_to_df_value)
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
