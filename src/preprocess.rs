use clap::Parser;
use colored::Colorize;
use std::{path::Path, process};

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
    // pub fn get_path(&self) -> &Path {
    // return Path::new(&self.path.as_str());
    // }

    pub fn get_path_str(&self) -> &String {
        return &self.path;
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    pub fn get_werror(&self) -> bool {
        self.werror
    }

    pub fn check(&self) {
        if self.path == "" {
            exit();
        }

        if !Path::new(&self.path.as_str()).exists() {
            exit();
        }
    }

    pub fn print(&self) {
        println!("Arguments:");
        println!(" + {}", "`PATH`".green());
        println!(
            " + {}",
            if self.get_verbose() {
                "`VERBOSE`".green()
            } else {
                "`VERBOSE`".red()
            }
        );
        println!(
            " + {}",
            if self.get_werror() {
                "`WERROR`".green()
            } else {
                "`WERROR`".red()
            }
        );
    }
}

fn exit() {
    println!(
        "{} The following required arguments were not provided:\n  {}",
        "error:".red(),
        "<PATH>".green()
    );
    println!("\n{} fortran-rs <PATH>", "Usage:".underline());
    println!("\nFor more information try '--help'");
    process::exit(2);
}

pub fn process_args() -> Args {
    let args: Args = Args::parse();
    args.check();

    if args.get_verbose() {
        args.print();
    }

    return args;
}
