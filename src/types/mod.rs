use std::error::Error;
use std::fmt;

pub mod app_config;
pub mod note;

pub use app_config::AppConfig;
pub use note::Note;

#[derive(Debug)]
pub struct NoteError {
    pub message: &'static str,
}

impl std::fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoteError: {}", &self.message)
    }
}

impl Error for NoteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct CommandError {
    pub message: &'static str,
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NoteError: {}", &self.message)
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
