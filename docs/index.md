# Welcome to fortran.rs

An interpreter for the Fortran programming language written in Rust.

This is the documentation for the fortran.rs project.

## Installation

### From Source

Clone the repository and build the project.

```bash
git clone https://github.com/Xavier2p/fortran.rs.git
cd fortran.rs
cargo build --release
./target/release/fortran-rs <file>
```

## Commands

* `-v` - Verbose, print the comments
* `-Werror` - Treat warnings as errors
* `-h` - Print help


## Project layout

```text
fortran-rs
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── docs
│   ├── index.md
│   └── cours_fortran90.pdf
├── src
│   ├── lexer.rs
│   ├── token.rs
│   ├── parser.rs
│   ├── runtime.rs
│   ├── variables.rs
│   ├── errors.rs
│   ├── file.rs
│   └── main.rs
└── tests
    ├── hello-world.f90
    └── simple-code.f90
```

