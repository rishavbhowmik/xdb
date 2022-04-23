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
            error_type,
            code: code.to_string(),
            description: description.unwrap_or("".to_string()),
        }
    }
    pub fn code(&self) -> &str {
        &self.code
    }
}

// add fmt::Debug trait to Error struct
use std::fmt;
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_type = match self.error_type {
            ErrorType::Warning => "Warning",
            ErrorType::Happens => "Happens",
            ErrorType::Critical => "Critical",
            ErrorType::Unexpected => "Unexpected",
        };
        if self.description.len() > 0 {
            write!(
                f,
                "{}\nCode: {}\nDescription: {}",
                error_type, self.code, self.description
            )
        } else {
            write!(f, "{}\nCode: {}", error_type, self.code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_new() {
        let error = Error::new(
            ErrorType::Warning,
            "test_error_new",
            Some("test_error_new".to_string()),
        );
        assert_eq!(error.code, "test_error_new");
        assert_eq!(error.description, "test_error_new");
    }

    #[test]
    fn test_error_new_fmt() {
        let error_warning = Error::new(
            ErrorType::Warning,
            "test_error_new_fmt",
            Some("Test error new fmt".to_string()),
        );
        assert_eq!(
            format!("{:?}", error_warning),
            "Warning\nCode: test_error_new_fmt\nDescription: Test error new fmt"
        );
        let error_happens = Error::new(
            ErrorType::Happens,
            "test_error_new_fmt",
            Some("Test error new fmt".to_string()),
        );
        assert_eq!(
            format!("{:?}", error_happens),
            "Happens\nCode: test_error_new_fmt\nDescription: Test error new fmt"
        );
        let error_critical = Error::new(
            ErrorType::Critical,
            "test_error_new_fmt",
            Some("Test error new fmt".to_string()),
        );
        assert_eq!(
            format!("{:?}", error_critical),
            "Critical\nCode: test_error_new_fmt\nDescription: Test error new fmt"
        );
        let error_unexpected = Error::new(
            ErrorType::Unexpected,
            "test_error_new_fmt",
            Some("Test error new fmt".to_string()),
        );
        assert_eq!(
            format!("{:?}", error_unexpected),
            "Unexpected\nCode: test_error_new_fmt\nDescription: Test error new fmt"
        );
    }

    #[test]
    fn test_error_new_no_description() {
        let error = Error::new(ErrorType::Warning, "test_error_new_no_description", None);
        assert_eq!(error.code, "test_error_new_no_description");
        assert_eq!(error.description, "");
    }

    #[test]
    fn test_error_new_no_description_fmt() {
        let error_warning = Error::new(
            ErrorType::Warning,
            "test_error_new_no_description_fmt",
            None,
        );
        assert_eq!(
            format!("{:?}", error_warning),
            "Warning\nCode: test_error_new_no_description_fmt"
        );
        let error_happens = Error::new(
            ErrorType::Happens,
            "test_error_new_no_description_fmt",
            None,
        );
        assert_eq!(
            format!("{:?}", error_happens),
            "Happens\nCode: test_error_new_no_description_fmt"
        );
        let error_critical = Error::new(
            ErrorType::Critical,
            "test_error_new_no_description_fmt",
            None,
        );
        assert_eq!(
            format!("{:?}", error_critical),
            "Critical\nCode: test_error_new_no_description_fmt"
        );
        let error_unexpected = Error::new(
            ErrorType::Unexpected,
            "test_error_new_no_description_fmt",
            None,
        );
        assert_eq!(
            format!("{:?}", error_unexpected),
            "Unexpected\nCode: test_error_new_no_description_fmt"
        );
    }
}
