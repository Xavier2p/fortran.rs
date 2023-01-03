use colored::Colorize;

pub struct Error {
    filename: String,
    function: String,
    line: usize,
    column: usize,
    value: String,
    code: i32,
}

impl Error {
    pub fn new(
        filename: String,
        function: String,
        line: usize,
        column: usize,
        value: String,
        code: i32,
    ) -> Error {
        Error {
            filename,
            function,
            line,
            column,
            value,
            code,
        }
    }

    pub fn raise(&self) {
        println!(
            "Error handled on {}, in `{}` at line {}:{}.",
            self.function.blue(), self.filename.yellow(), self.line, self.column
        );
        println!("Error: {}", self.value.red());
        println!("Exiting with code {}", self.code.to_string().red());
        std::process::exit(self.code);
    }
}
