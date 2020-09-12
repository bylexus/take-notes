mod commands;
pub mod types;

use commands::create_command;
use std::collections::HashMap;
use types::AppConfig;
use regex::Regex;
use sanitize_filename as lib_sanitize;

pub fn run() -> Result<(), String> {
    return Ok(());
}

pub fn init(conf: &AppConfig) {
    let err = format!(
        "Error: could not create root notes directory {}",
        conf.root_dir.display()
    );
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&conf.root_dir)
        .expect(&err);
}

pub fn execute_command(command: &str, options: &HashMap<&str, &str>) -> Result<(), String> {
    if command == "create" {
        create_command(options)
    } else {
        Err(format!("Unknown command: {}", command))
    }
}

pub fn clean_filename(name : &str) -> String {
    let re = Regex::new(r"[^\w_.-]").unwrap();
    let opts = lib_sanitize::Options {
        windows: true,
        truncate: false,
        replacement: "-"
    };
    let sanitized_title = lib_sanitize::sanitize_with_options(name, opts);
    let sanitized_title = re.replace_all(sanitized_title.as_str(), "-");
    String::from(sanitized_title.trim_matches('-'))
}

