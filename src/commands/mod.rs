use crate::open_git_repo;
use crate::types::AppConfig;
use crate::types::Note;
use crate::types::NoteError;
use std::collections::HashMap;
use std::error;

pub fn add_command(
    app_conf: &AppConfig,
    options: &HashMap<&str, &str>,
) -> Result<(), Box<dyn error::Error>> {
    // ensure git repo:
    // TODO: we work with the repo later
    // let repo = open_git_repo(&app_conf.root_dir)?;

    // unwrap is safe here: the main program already sets it for sure:
    let title = options.get("title").unwrap().trim();
    if title.len() == 0 {
        return Err(Box::new(NoteError {
            message: "Title is empty. Please provide a valid title.",
        }));
    }

    let note = Note::from_path(title, &app_conf.root_dir);
    println!("Note: {:#?}", note);

    Ok(())
}
