use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use lib::{execute_command, init, types::AppConfig};

const APP_IDENT: &str = "ch.alexi.take-notes";

fn main() {
    let mut app_config = AppConfig::new(APP_IDENT);

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
            SubCommand::with_name("create")
                .about("Creates a new note")
                .arg_from_usage(
                    "<title_with_path> 'title or title with path, e.g. \'my/folder/Note Title\''",
                ),
        )
        .get_matches();

    // Read user-delivered dir, if any:
    if let Some(user_dir) = matches.value_of("dir") {
        app_config.root_dir = std::path::PathBuf::from(user_dir);
    }

    // Init the app, make sure everything is OK befor proceeding:
    init(&app_config);

    // The create command:
    let mut command = "";
    let mut options = HashMap::new();
    if let Some(matches) = matches.subcommand_matches("create") {
        command = "create";
        options.insert("title", matches.value_of("title_with_path").unwrap());
    }

    println!(
        "Using Notes Data Dir: {}",
        app_config.root_dir.into_os_string().into_string().unwrap()
    );

    match execute_command(command, &options) {
        Ok(_) => (),
        Err(msg) => eprintln!("ERROR: {}",msg)
    }
}
