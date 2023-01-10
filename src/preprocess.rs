use std::{env, process};
// use crate::file_traitement::File;

fn help() {
    println!("fortran.rs, an open-source fortran interpreter.");
    println!("TODO");
}

fn exit(reason: &str) {
    println!("CriticalError: {}", reason);
    help();
    process::exit(2);
}

fn split_args(args: Vec<String>) {
    // do stuff with all args
}

pub fn process_args() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => exit("No Args..."),
        1 => exit("No filename provided"),
        _ => split_args(args),
    }
}