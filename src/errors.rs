//! # Errors
//!
//! This module contains the error handling system.
use colored::Colorize;

/// This enum contains the different types of errors.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ErrorKind {
    Syntax,
    NotImplemented,
    FileNotFound,
    Type,
    UnknownToken,
    UnexpectedToken,
    Critical,
}

/// This struct contains the errors.
#[allow(dead_code)]
pub struct Error {
    filename: String,
    function: String,
    line: usize,
    column: usize,
    value: String,
    kind: ErrorKind,
    code: i32,
}

/// This struct contains the errors.
impl Error {
    /// This function returns the code of the error.
    fn get_code_number(kind: ErrorKind) -> i32 {
        match kind {
            ErrorKind::Syntax => 1,
            ErrorKind::NotImplemented => 2,
            ErrorKind::FileNotFound => 1,
            ErrorKind::Type => 1,
            ErrorKind::UnknownToken => 1,
            ErrorKind::UnexpectedToken => 1,
            ErrorKind::Critical => 2,
        }
    }

    /// This function returns the error's level.
    fn get_error(&self, level: &str) -> String {
        match self.kind {
            ErrorKind::Syntax => "Syntax",
            ErrorKind::NotImplemented => "NotImplemented",
            ErrorKind::FileNotFound => "FileNotFound",
            ErrorKind::Type => "Type",
            ErrorKind::UnknownToken => "UnknownToken",
            ErrorKind::UnexpectedToken => "UnexpectedToken",
            ErrorKind::Critical => "Critical",
        }
        .to_string()
            + level
    }

    /// This function returns a new error.
    pub fn new(
        filename: String,
        function: String,
        line: usize,
        column: usize,
        value: String,
        kind: ErrorKind,
    ) -> Error {
        Error {
            filename,
            function,
            line,
            column,
            value,
            kind,
            code: Error::get_code_number(kind),
        }
    }

    /// This function returns a new warning.
    #[allow(dead_code)]
    pub fn warn(&self) {
        println!(
            "Warning handled on {}, in `{}` at line {}:{}.",
            self.filename.blue(),
            self.function.yellow(),
            self.line,
            self.column
        );

        let kind: String = self.get_error("Warning");

        println!("Warning: {}", kind.yellow());
        println!("        > {}", self.value.cyan());
    }

    /// This function returns a new error.
    #[allow(dead_code)]
    pub fn raise(&self) {
        eprintln!(
            "Error handled on {}, in `{}` at line {}:{}.",
            self.filename.blue(),
            self.function.yellow(),
            self.line,
            self.column
        );

        let kind: String = self.get_error("Error");

        eprintln!("Error: {}", kind.red());
        eprintln!("      > {}", self.value.magenta());
        eprintln!(
            "Exiting with code {}...",
            self.code.to_string().red().dimmed()
        );
        std::process::exit(self.code);
    }
}
