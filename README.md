## About

Toy compiler for a subset of C written in Rust.

Relies (for now) on a compiler driver (GCC) for invoking the preprocessor,
assembler and linker.

**Current Features**:

- Bare minimum: Can compile a main function that returns an integer into x64
  assembly.
- Unary operators: Supports the negation (`-`) and bitwise complement (`~`)
  operators.

## Getting started

Dependencies are managed using `flake.nix`.

Run `cargo build --release` to build project, executable will be found in
`target/release`. Run `crust --help` for information on usage.

Use `./run.sh` to run the sample program and automatically print the exit code.
