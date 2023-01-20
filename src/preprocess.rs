use crate::errors::{Error, ErrorKind};
use clap::Parser;
use colored::Colorize;
use std::path::Path;

#[derive(Parser, Default, Debug, Clone)]
#[command(author = "Xavier2p", version, about)]
/// fortran-rs: An open-source Fortran interpreter, written in Rust
pub struct Args {
    /// Path to the file to threat
    path: String,

    #[arg(short, long)]
    /// Print the comment during the execution of the program
    verbose: bool,

    #[arg(long)]
    /// Threat `Warning` as `Error`
    werror: bool,
}

impl Args {
    pub fn get_path(&self) -> &String {
        return &self.path;
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_werror(&self) -> bool {
        self.werror
    }

    pub fn check(&self) -> Result<(), Error> {
        if self.path == "" || !Path::new(&self.path.as_str()).exists() {
            Err(Error::new(
                "none".to_string(),
                "none".to_string(),
                0,
                0,
                format!("No file exists with this name: `{}`", self.path),
                ErrorKind::FileNotFound,
            ))
        } else {
            Ok(())
        }
    }

    pub fn print(&self) {
        println!("{} Arguments:", "|".dimmed());
        println!("{} + {}", "|".dimmed(), "`PATH`".green());
        println!(
            "{} + {}",
            "|".dimmed(),
            if self.get_verbose() {
                "`VERBOSE`".green()
            } else {
                "`VERBOSE`".red()
            }
        );
        println!(
            "{} + {}",
            "|".dimmed(),
            if self.get_werror() {
                "`WERROR`".green()
            } else {
                "`WERROR`".red()
            }
        );
    }
}

pub fn process_args() -> Args {
    let args: Args = Args::parse();
    match args.check() {
        Ok(_) => {}
        Err(err) => err.raise(),
    }

    if args.get_verbose() {
        args.print();
    }

    return args;
}
