//! This module is used to parse the arguments passed to the program.
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::Path;

/// This struct is used to parse the arguments passed to the program.
#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
#[command(subcommand_required = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Print the comment during the execution of the program
    #[arg(short, long)]
    verbose: bool,
}

/// This struct is used to parse the arguments passed to the program.
#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// Run the Fortran file passed as argument
    #[command(arg_required_else_help = true)]
    Run {
        /// Path to the file to interpret
        #[arg(value_parser = check_path)]
        file: String,

        /// Print the comment during the execution of the program
        #[arg(short, long)]
        verbose: bool,
    },

    /// Check the syntax of the file passed as argument
    #[command(arg_required_else_help = true)]
    Check {
        /// Path to the file to interpret
        #[arg(value_parser = check_path)]
        file: String,

        /// Threat `Warning` as `Error`
        #[arg(long)]
        werror: bool,

        /// Print the comment during the execution of the program
        #[arg(short, long)]
        verbose: bool,
    },
}

/// This function returns the path to the file to interpret.
pub fn get_path(args: &Cli) -> String {
    match &args.command {
        Commands::Run { file, .. } => file.to_string(),
        Commands::Check { file, .. } => file.to_string(),
    }
}

/// This function returns the value of the `verbose` argument.
pub fn get_verbose(args: &Cli) -> bool {
    args.verbose
        || match &args.command {
            Commands::Run { verbose, .. } => *verbose,
            Commands::Check { verbose, .. } => *verbose,
        }
}

/// This function returns the value of the `werror` argument.
pub fn get_werror(args: &Cli) -> bool {
    match &args.command {
        Commands::Run { .. } => false,
        Commands::Check { werror, .. } => *werror,
    }
}

/// This function prints the arguments passed to the program.
pub fn print(args: &Cli) {
    println!("{} Arguments:", "|".dimmed());
    println!("{} + {}", "|".dimmed(), "`PATH`".green());
    println!(
        "{} + {}",
        "|".dimmed(),
        if get_verbose(args) {
            "`VERBOSE`".green()
        } else {
            "`VERBOSE`".red()
        }
    );
    println!(
        "{} + {}",
        "|".dimmed(),
        if get_werror(args) {
            "`WERROR`".green()
        } else {
            "`WERROR`".red()
        }
    );
}

/// This function parses the arguments passed to the program.
pub fn process_args() -> Cli {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Run { .. } => {
            println!("{}: `{}`...", "Running".green(), get_path(&args));
        }
        Commands::Check { .. } => {
            println!("{}: `{}`...", "Checking".green(), get_path(&args));
        }
    }

    if get_verbose(&args) {
        print(&args);
    }

    args
}

/// This function checks if the file exists.
fn check_path(input: &str) -> Result<String, String> {
    if !Path::new(input).exists() {
        Err(format!("No file exists with this name: `{}`", input))
    } else {
        Ok(input.to_string())
    }
}
