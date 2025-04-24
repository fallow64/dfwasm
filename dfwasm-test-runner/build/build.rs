use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

mod numbers;
mod test_arithmetic;
mod test_memory;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    I32,
    I64,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
        }
    }
}

pub fn clear_directory(direction: &str, subdirectory: &str) {
    let test_dir = Path::new("test_files").join(direction).join(subdirectory);

    if test_dir.exists() {
        // delete all .wat, .wasm, and .test files in the directory
        for entry in fs::read_dir(&test_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path
                .extension()
                .map_or(false, |ext| ext == "wat" || ext == "wasm" || ext == "test")
            {
                fs::remove_file(path).unwrap();
            }
        }
    }
}

pub fn write_test(
    directory: &str,
    subdirectory: &str,
    base_name: &str,
    wat_file_contents: &str,
    test_file_contents: &str,
) {
    // create the directory if it doesn't exist
    let test_dir = Path::new("test_files").join(directory).join(subdirectory);
    if !test_dir.exists() {
        fs::create_dir_all(&test_dir).unwrap();
    }

    // create the wat file
    let wat_file_path = test_dir.join(format!("{}.wat", base_name));
    let mut wat_file = File::create(&wat_file_path).unwrap();
    writeln!(wat_file, "{}", wat_file_contents).unwrap();

    // create the cases file
    let test_file_path = test_dir.join(format!("{}.test", base_name));
    let mut test_file = File::create(&test_file_path).unwrap();
    writeln!(test_file, "{}", test_file_contents).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=build");

    test_arithmetic::build();
    test_memory::build();
}
