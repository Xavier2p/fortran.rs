# Alternative Installation methods

> **Warning**  
> All of this options are not available yet.

## From crates.io

```bash
cargo install fortran-rs
```

## With Docker

```bash
docker run --rm -it --name fortran-rs --volume ./:/app/src xavier2p/fortran-rs:latest run <FILE>
```

## With Nix Flake

```bash
nix run github:xavier2p/fortran.rs
```
