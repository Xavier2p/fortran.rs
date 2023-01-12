use clap::Parser;
use colored::Colorize;
use std::process;

#[derive(Parser, Default, Debug)]
#[command(author = "Xavier2p", version, about)]
/// fortran-rs: An open-source Fortran interpreter, written in Rust
struct Args {
    /// Path to the file to threat
    path: String,

    #[arg(short, long)]
    /// Print the comment during the execution of the program
    verbose: bool,
    
    #[arg(long)]
    /// Threat `Warning` as `Error`
    werror: bool,
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

pub fn process_args() {
    let args: Args = Args::parse();
    println!("{:#?}", args);
    if args.path == "" {
        exit();
    }
}
