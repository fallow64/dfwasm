use dfwasm_template::{Args, Item, Template};

// Hack to allow us to extend the Template struct
pub trait TemplateExt {
    fn push_op_stack(&mut self, var_to_push: Item) -> &mut Self;
    fn pop_op_stack(&mut self, var_to_pop: Item) -> &mut Self;
    fn compare_and_push(&mut self, action: impl Into<String>, a: Item, b: Item) -> &mut Self;
}

// Implement the trait for the foreign type
impl TemplateExt for Template {
    /// Pushes a value to the operand stack.
    fn push_op_stack(&mut self, var_to_push: Item) -> &mut Self {
        self.set_var(
            "AppendValue",
            Args::with(vec![Item::var(DF_VAR_OP_STACK), var_to_push]),
        )
    }

    /// Pops a value from the operand stack and sets it to the given variable.
    fn pop_op_stack(&mut self, var_to_pop: Item) -> &mut Self {
        self.set_var(
            "PopListValue",
            Args::with(vec![var_to_pop, Item::var(DF_VAR_OP_STACK)]),
        )
    }

    /// Pushes a one or zero depending on the result of the comparison.
    fn compare_and_push(&mut self, action: impl Into<String>, a: Item, b: Item) -> &mut Self {
        self.if_var(action, Args::with(vec![a, b]))
            .open_bracket()
            .push_op_stack(num(format_df_number_usize(1)))
            .close_bracket()
            .else_block()
            .open_bracket()
            .push_op_stack(num(format_df_number_usize(0)))
            .close_bracket()
    }
}

/// Formats an i64 number to a string.
///
/// DiamondFire numbers are weird in that they used fixed-point arithmetic with 3 decimal places (i.e. i64/1000).
/// This function formats the number as a string with the decimal point in the right place.
///
/// - A value of 1 would be formatted as "0.001".
/// - A value of 1000 would be formatted as "1".
/// - A value of 820 would be formatted as "0.82".
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

/// Formats a u64 number to a string. See [`format_df_number_i64`] for details.
pub fn format_df_number_u64(value: u64) -> String {
    let int_part = value / 1000;
    let frac_part = value % 1000;

    // Format the fractional part as a 3-digit number, then trim trailing zeros
    let mut frac_str = format!("{frac_part:03}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }

    if frac_str.is_empty() {
        format!("{int_part}")
    } else {
        format!("{int_part}.{frac_str}")
    }
}

/// Formats a u32 number to a string. See [`format_df_number_i64`] for details.
pub fn format_df_number_u32(value: u32) -> String {
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

/// Formats a usize number to a string. See [`format_df_number_i64`] for details.
pub fn format_df_number_usize(value: usize) -> String {
    let int_part = value / 1000;
    let frac_part = value % 1000;

    // Format the fractional part as a 3-digit number, then trim trailing zeros
    let mut frac_str = format!("{frac_part:03}");
    while frac_str.ends_with('0') {
        frac_str.pop();
    }

    if frac_str.is_empty() {
        format!("{int_part}")
    } else {
        format!("{int_part}.{frac_str}")
    }
}

/// Generates a unique function name based on the given function index.
pub fn generate_function_name(index: usize, module_name: Option<&str>) -> String {
    match module_name {
        Some(name) => format!("wasm.{name}.func_{index}"),
        None => format!("wasm.module.func_{index}"),
    }
}

/// Generates a unique variable name based on the given table index.
pub fn generate_table_name(index: usize) -> String {
    format!("wasm.$table_{index}")
}

/// Generates a unique block name based on the given block index.
pub fn generate_block_name(index: usize, module_name: Option<&str>) -> String {
    match module_name {
        Some(name) => format!("wasm.{name}.block_{index}"),
        None => format!("wasm.module.block_{index}"),
    }
}

/// Generates a unique loop name based on the given loop index.
pub fn generate_loop_name(index: usize, module_name: Option<&str>) -> String {
    match module_name {
        Some(name) => format!("wasm.{name}.loop_{index}"),
        None => format!("wasm.module.loop_{index}"),
    }
}

/// Generates a unique conditional name based on the given conditional index.
pub fn generate_conditional_name(index: usize, module_name: Option<&str>) -> String {
    match module_name {
        Some(name) => format!("wasm.{name}.if_{index}"),
        None => format!("wasm.module.if_{index}"),
    }
}

/// Creates a variable item with the given name.
pub fn var(name: impl Into<String>) -> Item {
    Item::var(name)
}

/// Creates a number item with the number formatted as a string.
///
/// Does not handle DF number conversion. See [`format_df_number_i64`] for details.
pub fn num(name: impl ToString) -> Item {
    Item::num(name)
}

/// Creates a string item with the given value.
pub fn string(value: impl Into<String>) -> Item {
    Item::string(value)
}

/// The DF variable representing the operand stack. This is a list, so technically the max stack size is 10K.
pub const DF_VAR_OP_STACK: &str = "wasm.$op_stack";
/// The DF variable representing the current memory size.
pub const DF_VAR_MEM_SIZE: &str = "wasm.$memSize";

/// The DF variable representing the function store (i.e. a list of all of the compiled function names or imported functions).
pub const DF_VAR_STORE_FUNCS: &str = "wasm.$store_funcs";
/// The DF variable representing the tables store (i.e. a list of all of references to all of the tables).
pub const DF_VAR_STORE_TABLES: &str = "wasm.$store_tables";
/// The DF variable representing the globals store (i.e. a list of all of the globals).
pub const DF_VAR_STORE_GLOBALS: &str = "wasm.$store_globals";

/// This function is called to call a WASM function. It takes in the function index and number of arguments.
pub const DF_FUNC_CALL_FUNC: &str = "wasm.internal.call_func";
/// This function is called to grow a table. It has parameters for the table index and the number of elements to grow,
/// and pops off the stack the number of new elements.
pub const DF_FUNC_GROW_TABLE: &str = "wasm.internal.grow_table";
/// This function is called to fill a table. It has parameters for the table index,
/// and pops off the stack the number of elements to fill and the value to fill with.
pub const DF_FUNC_FILL_TABLE: &str = "wasm.internal.fill_table";
/// This function is called to load from memory. It has parameters for number of bytes and the offset,
/// and pops off the stack the address to load from. It pushes the value to the stack.
pub const DF_FUNC_MEM_LOAD: &str = "wasm.internal.mem_load";
/// This function is called to store memory. It has parameters for number of bytes and the offset,
/// and pops off the stack the value and the address to store to.
pub const DF_FUNC_MEM_STORE: &str = "wasm.internal.mem_store";
/// This function is called to sign extend a number. It has parameters for the   and to_bits,
/// both divided by 1000. It pops off the stack the number to sign extend, and pushes the value to the stack.
pub const DF_FUNC_SIGN_EXTEND: &str = "wasm.internal.sign_extend";
/// This function is called when initializing a data section. It does not operate on the stack.
/// It has one parameter for the memory address (including offset), and 26 bytes of data.
pub const DF_FUNC_BATCH_DATA_SECTION: &str = "wasm.internal.batch_data_section";

pub const DF_FUNC_TRAP: &str = "wasm.internal.trap";

pub const DF_VAR_BRANCH_COUNTER: &str = "wasm.$branch_counter";
pub const DF_FUNC_CONTROL_LOOP_CHECK: &str = "wasm.internal.control_loop_check";

pub const DF_VAR_CURRENT_STACK_FRAME: &str = "wasm.$current_stack_frame";
pub const DF_VAR_IMPORTS: &str = "wasm.$imports";
pub const DF_VAR_EXPORTS: &str = "wasm.$exports";
pub const DF_VAR_EXPORT_SIGNATURES: &str = "wasm.$export_signatures";
pub const DF_FUNC_HOOK_INSTRUCTION: &str = "wasm.internal.hook_instruction";
pub const DF_FUNC_I64_MUL: &str = "wasm.internal.i64_mul";

pub fn get_local_name(local_index: u32) -> String {
    format!(
        "wasm.$frame%var({})_local_{}",
        DF_VAR_CURRENT_STACK_FRAME, local_index
    )
}
