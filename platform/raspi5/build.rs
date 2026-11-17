use std::{env, fs, path::PathBuf};

fn main() {
    let source = "../../thistle/src/native/abi.ts";
    println!("cargo:rerun-if-changed={source}");
    let abi = fs::read_to_string(source).expect("initialise pinned Thistle submodule");
    let mut constants = String::new();
    for name in ["TETO_HOST_WRITE", "TETO_BATCH_BUDGET", "TETO_BATCH_HOST",
                 "TETO_BATCH_EXITED", "TETO_STARTUP_MAGIC"] {
        let prefix = format!("export const {name}:");
        let line = abi.lines().find(|line| line.starts_with(&prefix)).expect("ABI constant");
        let value = line.split_once('=').unwrap().1.trim().trim_end_matches(';');
        let number = if let Some(hex) = value.strip_prefix("0x") {
            u32::from_str_radix(hex, 16).unwrap()
        } else {
            value.parse::<u32>().unwrap()
        };
        constants.push_str(&format!("pub const {name}: u32 = {number};\n"));
    }
    fs::write(PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("abi.rs"), constants).unwrap();
}
