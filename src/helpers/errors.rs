//! Errors and warnings management
//!
//! This module contains the error handling functions.
use crate::program::Program;
use colored::Colorize;

/// This enum contains the different types of errors.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    /// This error is raised when the syntax is invalid.
    Syntax,

    /// This error is raised when a feature is not implemented.
    NotImplemented,

    /// This error is raised when a file is not found.
    FileNotFound,

    /// This error is raised when a type is invalid.
    Type,

    /// This error is raised when a token is unknown.
    UnknownToken,

    /// This error is raised when a token is unexpected.
    UnexpectedToken,

    /// This error is raised when a critical error is encountered.
    Critical,

    /// This error is raised when a line contains more than 100 characters.
    TooCharactersOnLine,

    /// This error is raised when a line contains a forbidden character.
    ForbiddenCharacter,

    /// This error is raised when a line is not formatted correctly.
    WrongFormat,
}

/// This function returns the string corresponding to the error.
fn error_to_string(error: &Error) -> &'static str {
    match error {
        Error::Syntax => "Syntax",
        Error::NotImplemented => "NotImplemented",
        Error::FileNotFound => "FileNotFound",
        Error::Type => "Type",
        Error::UnknownToken => "UnknownToken",
        Error::UnexpectedToken => "UnexpectedToken",
        Error::Critical => "Critical",
        Error::TooCharactersOnLine => "TooCharactersOnLine",
        Error::ForbiddenCharacter => "ForbiddenCharacter",
        Error::WrongFormat => "WrongFormat",
    }
}

/// This function returns the string corresponding to the error.
fn to_stderr(program: &Program, kind: &Error, message: String, is_warning: bool) -> String {
    let location = format!(
        "{} {}:{}:{}",
        "-->".blue(),
        program.get_filename(),
        program.get_name(),
        program.get_line()
    );
    format!(
        "{}: {}: `{}`\n{}",
        if is_warning {
            "warning".yellow()
        } else {
            "error".red()
        },
        error_to_string(kind).cyan(),
        message,
        location,
    )
}

/// This function returns the string corresponding to the error.
pub fn raise(program: &Program, kind: Error, message: String) {
    let stderr: String = to_stderr(program, &kind, message, false);
    eprintln!("{}\n", stderr);
    std::process::exit(1);
}

/// This function returns the string corresponding to the error.
#[allow(dead_code)]
pub fn warn(program: &Program, kind: Error, message: String) {
    let stderr: String = to_stderr(program, &kind, message, true);
    println!("{}\n", stderr);
}
