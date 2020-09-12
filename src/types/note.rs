use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub created: DateTime<Utc>,
    pub body: String,
}

impl Note {
    pub fn new(title: &str) -> Note {
        Note {
            title: String::from(title),
            created: Utc::now(),
            body: String::from(""),
        }
    }
}
