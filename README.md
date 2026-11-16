# neon-willow

![CI](https://img.shields.io/github/actions/workflow/status/jcormierjdk/neon-willow/ci.yml?branch=main&label=CI) ![Rust](https://img.shields.io/badge/rust-1.70+-DEA584) ![License](https://img.shields.io/github/license/jcormierjdk/neon-willow?color=blue) ![Version](https://img.shields.io/badge/version-0.12.8-green)

A memory-safe Rust CLI for batch text processing. neon-willow reads input files, applies line-by-line transformations (trim, enumerate, format), and outputs results. Built with Clap derive macros for ergonomic CLI arguments, Rayon for parallel line processing, and Serde for TOML/JSON config. Compiles to a single static binary with no runtime dependencies.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Running Tests](#running-tests)
- [Contributing](#contributing)
- [Changelog](#changelog)
- [License](#license)

## Features

- Rayon-powered parallel line processing for large files
- 100% safe Rust -- no unsafe blocks
- Verbose mode with timing (Instant::now/elapsed) and line count
- Serde Serialize/Deserialize for structured configuration (Config struct)
- Integration tests with assert_cmd and predicates

## Prerequisites

- Rust 1.70 or higher (stable toolchain)
- Cargo (included with Rust)

## Installation

```bash
cargo install neon_willow
# or from source
git clone https://github.com/jcormierjdk/neon-willow.git
cd neon-willow && cargo build --release
```

## Usage

```bash
# Process a file with default settings
neon_willow --input file.txt --output result.txt

# With verbose output and line numbering
neon_willow -v --input file.txt
# => Processed 42 lines
# => Done in 1.23ms

# Process from stdin
cat file.txt | neon_willow --output -
```

## Project Structure

```
neon-willow/
├── src/main.rs
├── tests/integration_test.rs
├── Cargo.toml
├── README.md  LICENSE  CHANGELOG.md  CONTRIBUTING.md  CODE_OF_CONDUCT.md
├── .github/
│   ├── workflows/ci.yml
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
└── .gitignore
```

## Running Tests

```bash
cargo test
# with output
cargo test -- --nocapture
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

Quick start:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and run tests (see [Running Tests](#running-tests) above)
4. Commit with a clear message: `git commit -am 'Add: my feature'`
5. Push to your fork: `git push origin feature/my-feature`
6. Open a Pull Request

Please ensure all tests pass before submitting a pull request.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes and version history.

## License

This project is licensed under the Apache-2.0 -- see the [LICENSE](LICENSE) file for details.

Copyright (c) 2026 omar dubois.

## Acknowledgements

- Built with Rust, Clap, Rayon, and Serde
- CI/CD powered by [GitHub Actions](https://github.com/features/actions)
- Badges by [shields.io](https://shields.io)
- Inspired by best practices from the open-source community
