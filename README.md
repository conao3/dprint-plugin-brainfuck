# dprint-plugin-brainfuck

A [dprint](https://dprint.dev/) plugin for formatting [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) code.

## Overview

This plugin integrates with dprint to provide consistent formatting for Brainfuck source files. It compiles to WebAssembly for fast, cross-platform execution.

## Installation

Add the plugin to your `dprint.json` configuration file:

```json
{
  "plugins": [
    "https://plugins.dprint.dev/brainfuck-x.x.x.wasm"
  ]
}
```

Replace `x.x.x` with the desired version number.

## Configuration

Configure the plugin in your `dprint.json`:

```json
{
  "brainfuck": {
    "lineWidth": 120
  }
}
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `lineWidth` | number | 120 | Maximum line width before wrapping |

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- [dprint CLI](https://dprint.dev/install/)

### Building

```sh
cargo build --release
```

### Testing

```sh
cargo test
```

### Running Locally

Use dprint to test the plugin during development:

```sh
dprint fmt
```

## License

Apache License 2.0. See [LICENSE](LICENSE) for details.
