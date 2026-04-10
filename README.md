# pswd

A minimal, fast CLI password generator written in Rust. Generated passwords are automatically copied to the clipboard.

## Usage

```
pswd [OPTIONS] [LENGTH]
```

### Arguments

| Argument | Default | Description                      |
| -------- | ------- | -------------------------------- |
| `LENGTH` | `32`    | Length of the generated password |

### Options

| Flag                      | Description                                              |
| ------------------------- | -------------------------------------------------------- |
| `-e`, `--exclude-special` | Use only alphanumeric characters (no special characters) |
| `-h`, `--help`            | Print help                                               |

## Examples

```bash
# Generate a 32-character password (default)
pswd

# Generate a 16-character password
pswd 16

# Generate a 64-character password without special characters
pswd 64 --exclude-special
```

## Character Sets

| Mode                | Characters                                         |
| ------------------- | -------------------------------------------------- |
| Default             | `A-Z`, `a-z`, `0-9`, `!@#$%^&*()_+-=[]{}\|;:,.<>?` |
| `--exclude-special` | `A-Z`, `a-z`, `0-9`                                |

## Installation

### From crates.io

```bash
cargo install pswd
```

### From source

```bash
git clone https://github.com/masiama/pswd
cd pswd
cargo install --path .
```

## License

MIT
