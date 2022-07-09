# move-readme

Generates a readme from a Move package.

## Setup

Install the CLI using Cargo:

```bash
cargo install move-readme

# On Sui
cargo install move-readme --features address20

# On Aptos
cargo install move-readme --features address32
```

### Usage

In a directory containing a `Move.toml`, run:

```bash
move-readme
```

This will write a `README.md` file to the current directory.

License: Apache-2.0
