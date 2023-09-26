use crate::program::Program;
use colored::Colorize;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Error {
    Syntax,
    NotImplemented,
    FileNotFound,
    Type,
    UnknownToken,
    UnexpectedToken,
    Critical,
    TooCharacters,
}

fn error_to_string(error: &Error) -> &'static str {
    match error {
        Error::Syntax => "Syntax",
        Error::NotImplemented => "NotImplemented",
        Error::FileNotFound => "FileNotFound",
        Error::Type => "Type",
        Error::UnknownToken => "UnknownToken",
        Error::UnexpectedToken => "UnexpectedToken",
        Error::Critical => "Critical",
        Error::TooCharacters => "TooCharacters",
    }
}

fn get_code_number(kind: Error) -> u8 {
    match kind {
        Error::Syntax => 1,
        Error::NotImplemented => 2,
        Error::FileNotFound => 1,
        Error::Type => 1,
        Error::UnknownToken => 1,
        Error::UnexpectedToken => 1,
        Error::Critical => 2,
        Error::TooCharacters => 1,
    }
}

fn header(program: &Program, kind: &Error, is_warning: bool) -> String {
    let kind_colored = if is_warning {
        error_to_string(kind).yellow()
    } else {
        error_to_string(kind).red()
    };
    let location = format!(
        "In the file `{}`,\n  at block `{}`,\n    at line {}",
        program.get_filename(),
        program.get_name(),
        program.get_line()
    );
    format!(
        "{}\n     {} {}",
        location,
        kind_colored,
        if is_warning { "warning" } else { "error" }
    )
}

pub fn raise(program: &Program, kind: Error, message: String) {
    let header: String = header(program, &kind, false);
    let code = get_code_number(kind);
    eprintln!("{} [Code {}]\nDetails: {}", header, code, message);
    // std::process::exit();
}

pub fn warn(program: &Program, kind: Error, message: String) {
    let header: String = header(program, &kind, true);
    let code: u8 = get_code_number(kind);
    println!("{} [Code {}]\nDetails: {}", header, code, message);
}
