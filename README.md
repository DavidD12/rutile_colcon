# rutile_colcon

Simple generator for Rust ROS 2 packages with `colcon` and `r2r` integration.

## Description

`rutile_colcon` automatically creates the boilerplate for a Rust package that can be built with ROS 2 / `colcon`:

- initializes a Cargo package
- adds required Rust dependencies
- generates ROS 2 files (`package.xml`, `CMakeLists.txt`, `r2r_cargo.cmake`)
- generates a minimal Rust node binary

## Features

- simple CLI powered by `clap`
- generates a package ready for `colcon build`
- creates a minimal `<package>_node` Rust node
- configures a Cargo `colcon` build profile

## Requirements

- Rust + Cargo
- ROS 2 environment + `colcon`
- CMake (the generated `r2r_cargo.cmake` indicates a minimum of 3.21)
- standard system tools (`rm`)

## Installation

### From source

```bash
git clone https://github.com/DavidD12/rutile_colcon.git
cd rutile_colcon
cargo install --path .
```

### Run locally (without global install)

```bash
cargo run -- --package-name my_pkg --directory /tmp
```

## Usage

```bash
rutile_colcon --package-name <package_name> --directory <destination_directory> [--verbose <level>]
```

Options:

- `-p, --package-name`: name of the package to generate
- `-d, --directory`: destination parent directory
- `-v, --verbose`: verbosity level (default: `1`)

Example:

```bash
rutile_colcon -p demo_robot -d /tmp
```

This creates:

```text
/tmp/demo_robot/
```

## Generated files

For package `demo_robot`, the generated structure includes:

- `Cargo.toml` (Cargo init + `colcon` profile + `demo_robot_node` binary)
- `bin/demo_robot_node.rs` (minimal Rust node)
- `package.xml`
- `CMakeLists.txt`
- `r2r_cargo.cmake`
- `dummy.c`

## Technical notes

- The generator runs `cargo init --bin`.
- It adds these dependencies:
  - `r2r`
  - `rutile_r2r`
  - `tokio` with `features = ["full"]`
- The default `src/main.rs` is removed and replaced by a binary in `bin/`.

## License

Licensed under `LGPL-3.0-only`.
