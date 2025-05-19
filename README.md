# pinocchio-init

`pinocchio-init` is a CLI tool for initializing new Solana programs using Pinocchio.

## Templates

There are two templates available:

1. basic - A basic program with a single `lib.rs` file that logs "Hello, Solana!".

2. full - A complete counter program with structured state, instructions, and tests.

Select the template that suits your use case and customize it as needed.

## Installation

You can install `pinocchio-init` using Cargo:

```sh
cargo install pinocchio-init
```

## Usage

```sh
pinocchio-init <PROGRAM_NAME> [--template=basic|full]
```

- `<PROGRAM_NAME>`: The name of your new program.
- `--template`: (Optional) Choose `basic` or `full` template. Defaults to `basic`.

Example:

```sh
pinocchio-init my-awesome-program --template=full
```

## License

MIT