mod commands;
pub mod types;

use chrono::Local;
use commands::add_command;
use git2::Repository;
use regex::Regex;
use sanitize_filename as lib_sanitize;
use std::collections::HashMap;
use std::path::PathBuf;
use types::AppConfig;
use types::CommandError;

pub fn init(conf: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create root dir, if not exists:
    create_dir(&conf.root_dir)?;
    Ok(())
}

pub fn execute_command(
    command: &str,
    app_conf: &AppConfig,
    options: &HashMap<&str, &str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if command == "add" {
        return add_command(app_conf, options);
    }
    Err(Box::new(CommandError {
        message: "Unknown command",
    }))
}

pub fn create_note_dirname() -> String {
    let now = Local::now();
    now.format("%Y%m%d%H%M%S%.f").to_string()
}

pub fn clean_filename(name: &str) -> String {
    let re = Regex::new(r"[^\w_.-]").unwrap();
    let opts = lib_sanitize::Options {
        windows: true,
        truncate: false,
        replacement: "-",
    };
    let sanitized_title = lib_sanitize::sanitize_with_options(name, opts);
    let sanitized_title = re.replace_all(sanitized_title.as_str(), "-");
    String::from(sanitized_title.trim_matches('-'))
}

pub fn create_dir(path: &PathBuf) -> std::io::Result<()> {
    std::fs::DirBuilder::new().recursive(true).create(path)
}

pub fn open_git_repo(path: &PathBuf) -> Result<Repository, git2::Error> {
    // Try to open an existing repo first:
    if let Ok(repo) = Repository::open(path) {
        return Ok(repo);
    } else {
        // create a new one:
        let pathstr = String::from(path.to_str().unwrap());
        println!("Initializing GIT repository in {}", pathstr);
        return Repository::init(path);
    }
}

trait PathBufExt {
    fn from_parts(parts: Vec<String>) -> PathBuf;
}

impl PathBufExt for PathBuf {
    fn from_parts(parts: Vec<String>) -> PathBuf {
        let s = String::from(std::path::MAIN_SEPARATOR);
        PathBuf::from(parts.join(&s))
    }
}
