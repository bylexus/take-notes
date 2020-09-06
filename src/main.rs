use clap::{App, Arg, SubCommand};

const APP_IDENT: &str = "ch.alexi.take-notes";

struct AppConfig {
    root_dir: std::path::PathBuf,
}

fn init(conf: &AppConfig) {
    let err = format!(
        "Error: could not create root notes directory {}",
        conf.root_dir.display()
    );
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&conf.root_dir)
        .expect(&err);
}

fn main() {
    let mut root_dir = std::path::PathBuf::from(match dirs::data_dir() {
        Some(dir) => dir,
        None => std::path::PathBuf::default(),
    });
    root_dir.push(APP_IDENT);

    let mut app_config = AppConfig { root_dir };

    let matches = App::new("TakeNotes")
        .version("0.0.1")
        .author("Alexander Schenkel <alex-tn@alexi.ch>")
        .about("Keep track of your notes - the simple way")
        .arg(
            Arg::with_name("dir")
                .help("Set another notes root dir")
                .short("d")
                .long("dir")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Creates a new note")
                .arg_from_usage("<title_with_path> 'title or title with path'"),
        )
        .get_matches();

    // Read user-delivered dir, if any:
    if let Some(user_dir) = matches.value_of("dir") {
        app_config.root_dir = std::path::PathBuf::from(user_dir);
    }

    // The create command:
    if let Some(matches) = matches.subcommand_matches("create") {
        let title = matches.value_of("title_with_path").unwrap();
        println!("Given title: {}", title);
    }

    // Init the app, make sure everything is OK befor proceeding:
    init(&app_config);

    println!(
        "Notes Data Dir: {}",
        app_config.root_dir.into_os_string().into_string().unwrap()
    );
}
