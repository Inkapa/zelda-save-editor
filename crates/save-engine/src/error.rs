use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum SaveError {
    UnknownFormat,
    MissingField(&'static str),
    IndexOutOfRange { index: usize, max: usize },
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::UnknownFormat => write!(f, "unrecognized save file format"),
            SaveError::MissingField(name) => {
                write!(f, "expected field not found in hash table: {name}")
            }
            SaveError::IndexOutOfRange { index, max } => {
                write!(f, "index {index} out of range (max {max})")
            }
        }
    }
}

impl std::error::Error for SaveError {}
