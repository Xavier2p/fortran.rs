//! CLI interface and arguments parsing
//! This module is used to parse the arguments passed to the program.
use clap::{Parser, Subcommand};
use std::path::Path;

/// This struct is used to parse the arguments passed to the program.
#[derive(Parser, Clone)]
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
pub enum Commands {
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
        #[arg(long, short = 'W')]
        werror: bool,

        /// Print the comment during the execution of the program
        #[arg(short, long)]
        verbose: bool,
    },
}

impl Cli {
    /// This function returns the path to the file to interpret.
    pub fn get_path(&self) -> String {
        match &self.command {
            Commands::Run { file, .. } => file.to_string(),
            Commands::Check { file, .. } => file.to_string(),
        }
    }

    /// This function returns the value of the `verbose` argument.
    #[allow(dead_code)]
    pub fn get_verbose(&self) -> bool {
        self.verbose
            || match &self.command {
                Commands::Run { verbose, .. } => *verbose,
                Commands::Check { verbose, .. } => *verbose,
            }
    }

    /// This function returns the value of the `werror` argument.
    pub fn get_werror(&self) -> bool {
        match &self.command {
            Commands::Run { .. } => false,
            Commands::Check { werror, .. } => *werror,
        }
    }

    /// This function returns the called command.
    pub fn get_command(&self) -> &Commands {
        &self.command
    }

    /// This function prints the `Cli` struct for debug purposes.
    /// Not available in release mode.
    pub fn debug(&self) {
        println!("Cli {{");
        println!("    command: {:?}", self.get_command());
        println!("    path: {}", self.get_path());
        println!("    werror: {}", self.get_werror());
        println!("    verbose: {}", self.verbose);
        println!("}}");
        println!("---- ---- ---- ---- ----");
    }
}

/// This function checks if the file exists.
fn check_path(input: &str) -> Result<String, String> {
    if !Path::new(input).exists() {
        Err(format!("No file exists with this name: `{}`", input))
    } else {
        Ok(input.to_string())
    }
}
