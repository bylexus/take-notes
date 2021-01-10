use clap::{App, Arg, SubCommand};
use lib::{execute_command, init, types::AppConfig};
use std::collections::HashMap;

const APP_IDENT: &str = "ch.alexi.take-notes";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app_config = AppConfig::new(".take-notes");

    let matches = App::new("TakeNotes")
        .version("0.0.1")
        .author("Alexander Schenkel <alex-tn@alexi.ch>")
        .about("Keep track of your notes - the simple way")
        .arg(
            Arg::with_name("dir")
                .help("Set another notes root dir")
                .short("d")
                .long("dir")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a new note, or commits an existing one")
                .arg_from_usage(
                    "<title_with_path> 'title or title with path, e.g. \'my/folder/Note Title\''. If title is not a .md filename, an arbitary filename is created.",
                ),
        )
        .get_matches();

    // Read user-delivered dir, if any:
    if let Some(user_dir) = matches.value_of("dir") {
        app_config.root_dir = std::path::PathBuf::from(user_dir);
    }

    // Init the app, make sure everything is OK befor proceeding:
    init(&app_config)?;

    // The add command:
    let mut command = "";
    let mut options = HashMap::new();
    if let Some(matches) = matches.subcommand_matches("add") {
        command = "add";
        options.insert("title", matches.value_of("title_with_path").unwrap());
    }

    println!(
        "Using Notes Data Dir: {}",
        String::from(app_config.root_dir.to_str().unwrap())
    );

    execute_command(command, &app_config, &options)
}
