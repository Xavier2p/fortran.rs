# `fortran.rs`

![GitHub top language](https://img.shields.io/github/languages/top/xavier2p/fortran.rs?style=for-the-badge&logo=rust&color=orange)

This is a Fortran interpreter, written in Rust.

Now, it supports only the Fortran90 version, but you can help me to add more versions !

> **Warning**  
> This project is work in progress, so it's not working today.

## How to install it

### From source

```bash
git clone https://github.com/xavier2p/fortran.rs.git && cd fortran.rs/
cargo install --path .
```

Other ways are possible, please check [install.md](../docs/install.md) for more information.

## How to use it

```bash
fortran-rs run <FILE>
```

All the options are available with the `--help` option.

```console
$ fortran-rs --help
An open-source Fortran interpreter.
Written in Rust.

Usage: fortran-rs [OPTIONS] <COMMAND>

Commands:
  run    Run the Fortran file passed as argument
  check  Check the syntax of the file passed as argument
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Print the comment during the execution of the program
  -h, --help     Print help
  -V, --version  Print version
```
