# Move Readme

Readme generator for Move packages.

_Documentation is currently extremely sparse but will be improved in the near future._

## Setup

Install the CLI using Cargo:

```bash
cargo install move-readme

# On Sui
cargo install move-readme --features address20

# On Aptos
cargo install move-readme --features address32
```

## Usage

In a directory containing a `Move.toml`, run:

```
move-readme
```

This will write a `README.md` file to the current directory.

## License

Move Readme is licensed under the Apache License, Version 2.0.
