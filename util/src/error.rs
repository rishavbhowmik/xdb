use std::option::Option;

pub enum ErrorType {
    Warning,
    Happens,
    Critical,
    Unexpected,
}

pub struct Error {
    error_type: ErrorType,
    code: String,
    description: String,
}

impl Error {
    pub fn new(error_type: ErrorType, code: &str, description: Option<String>) -> Self {
        Error {
            error_type: error_type,
            code: code.to_string(),
            description: description.unwrap_or("".to_string()),
        }
    }
}

// add fmt::Debug trait to Error struct
use std::fmt;
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ error_type: {}, code: {}, description: {} }}",
            match self.error_type {
                ErrorType::Warning => "Warning",
                ErrorType::Happens => "Happens",
                ErrorType::Critical => "Critical",
                ErrorType::Unexpected => "Unexpected",
            },
            self.code,
            self.description,
        )
    }
}
