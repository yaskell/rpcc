## About

Toy compiler for a subset of C written in Rust.

Relies (for now) on a compiler driver (GCC) for invoking the preprocessor,
assembler and linker.

**Current Features**:

- Bare minimum: Can compile a main function that returns an integer.

## Get started

Dependencies are managed using `flake.nix`.

Run `cargo build --release` to build project, executable will be found in
`target/release`. Run `crust --help` for information on usage.
