use std::fmt;

pub mod persistence;
pub mod editor;

#[derive(Debug)]
pub struct Note {
    pub id: Option<isize>,
    pub name: String,
    pub content: String
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = match self.id {
            Some(id) => write!(f, "{}. ", id),
            None     => write!(f, "[unsaved] "),
        };
        write!(f, "{}\n{}", self.name, self.content)
    }
}

impl Note {
    pub fn new(id: Option<isize>, name: &str, content: &str) -> Note {
        Note { id: id, name: name.to_string(), content: content.to_string() }
    }
}

