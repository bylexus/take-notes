use crate::types::NoteError;
use std::fs::File;
use std::io::Write;
use crate::PathBufExt;
use crate::{clean_filename, create_note_dirname};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

const NOTE_FILENAME: &str = "note.md";

#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub created: DateTime<Utc>,
    pub path: PathBuf,
}

impl Note {
    pub fn new(title: &str, path: &PathBuf, root_dir: &PathBuf) -> Note {
        Note {
            title: String::from(title),
            created: Utc::now(),
            path: PathBuf::from(path),
        }
    }

    pub fn from_path(title: &str, root_dir: &PathBuf) -> std::io::Result<Note> {
        let parts: Vec<&str> = title.split('/').collect();
        let (title, parts) = parts.split_last().unwrap();
        let foldername = clean_filename(title);
        let parts: Vec<String> = parts
            .into_iter()
            .map(|val| clean_filename(val))
            .filter(|val| val.len() > 0)
            .collect();
        let mut path_buf = PathBuf::from_parts(parts);
        path_buf.push(foldername);

        match Note::load(&path_buf, root_dir) {
            Ok(note) => Ok(note),
            Err(_) => {
                let note = Note::new(*title, &path_buf, root_dir);
                note.create_note_file(root_dir)?;
                Ok(note)
            }
        }
    }

    pub fn load(path: &PathBuf, root_dir: &PathBuf) -> Result<Note, NoteError> {
        if Note::exists(path, root_dir) {
            Ok(Note::new("Foo", path, root_dir))
        } else {
            Err(NoteError {
                message: "Note does not exist",
            })
        }
    }

    pub fn exists(path: &PathBuf, root_dir: &PathBuf) -> bool {
        let full_path = root_dir.join(path);
        if !full_path.is_dir() {
            return false;
        }
        let note_file = full_path.join(NOTE_FILENAME);
        note_file.is_file()
    }

    pub fn create_note_file(&self, root_dir: &PathBuf) -> std::io::Result<()> {
        let full_path = root_dir.join(&self.path);
        let note_file = full_path.join(NOTE_FILENAME);
        if !full_path.is_dir() {
            std::fs::create_dir_all(full_path)?;
        }
        if !note_file.is_file() {
            let mut file = File::create(&note_file)?;
            let content = format!("title: {title}
created: {created}
---

# {title}

            ",
            title=self.title,
            created=self.created.to_rfc3339()
        );
            file.write_all(content.as_bytes())?;
        }

        Ok(())
    }
}
