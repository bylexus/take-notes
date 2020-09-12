use crate::clean_filename;
use crate::types::Note;
use std::collections::HashMap;

pub fn create_command(options: &HashMap<&str, &str>) -> Result<(), String> {
    // unwrap is safe here: the main program already sets it for sure:
    let title = options.get("title").unwrap().trim();
    if title.len() == 0 {
        return Err(String::from(
            "Title is empty. Please provide a valid title.",
        ));
    }

    // Split title into path and title parts:
    let parts: Vec<&str> = title.split('/').collect();
    let (title, parts) = parts.split_last().unwrap();
    let title = clean_filename(title);
    let parts: Vec<String> = parts
        .into_iter()
        .map(|val| clean_filename(val))
        .filter(|val| val.len() > 0)
        .collect();

    println!("Title: {}", title);
    println!("Path: {:#?}", parts);

    let note = Note::new(&title);
    println!("Note: {:#?}", note);

    Ok(())
}
