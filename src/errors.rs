use colored::Colorize;


#[allow(dead_code)]
pub enum ErrorKind {
    Syntax,
    NotImplemented,
    FileNotFound,
    Type,
}



pub struct Error {
    filename: String,
    function: String,
    line: usize,
    column: usize,
    value: String,
    kind: ErrorKind,
    code: i32,
}

impl Error {
    pub fn new(
        filename: String,
        function: String,
        line: usize,
        column: usize,
        value: String,
        kind: ErrorKind,
    ) -> Error {
        let code = match kind {
            ErrorKind::Syntax => 1,
            ErrorKind::NotImplemented => 2,
            ErrorKind::FileNotFound => 1,
            ErrorKind::Type => 1,
        };
        Error {
            filename,
            function,
            line,
            column,
            value,
            kind,
            code,
        }
    }

    #[allow(dead_code)]
    pub fn warn(&self) {
        println!(
            "Warning handled on {}, in `{}` at line {}:{}.",
            self.function.blue(),
            self.filename.yellow(),
            self.line,
            self.column
        );

        let kind = match self.kind {
            ErrorKind::Syntax => "SyntaxWarning",
            ErrorKind::NotImplemented => "NotImplementedWarning",
            ErrorKind::FileNotFound => "FileNotFoundWarning",
            ErrorKind::Type => "TypeWarning",
        };

        println!("Warning: {}", kind.yellow());
        println!("        > {}", self.value.cyan());
    }

    pub fn raise(&self) {
        println!(
            "Error handled on {}, in `{}` at line {}:{}.",
            self.function.blue(),
            self.filename.yellow(),
            self.line,
            self.column
        );

        let kind = match self.kind {
            ErrorKind::Syntax => "SyntaxError",
            ErrorKind::NotImplemented => "NotImplementedError",
            ErrorKind::FileNotFound => "FileNotFoundError",
            ErrorKind::Type => "TypeError",
        };

        println!("Error: {}", kind.red());
        println!("      > {}", self.value.magenta());
        println!("Exiting with code {}", self.code.to_string().red());
        std::process::exit(self.code);
    }
}
