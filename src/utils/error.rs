use std::error;
use std::fmt;

#[derive(Debug)]
pub struct CLIError {
    error_message: String,
    error_code: String,
}

impl CLIError {
    pub fn new(error_code: String, error_message: String) -> Box<Self> {
        Box::new(Self {
            error_code,
            error_message,
        })
    }
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}: {})",
            self.error_code, self.error_message
        )
    }
}

impl error::Error for CLIError {}