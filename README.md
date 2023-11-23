
# urlencode - A command line tool for manipulating URL-encoded query strings

[![Lint](https://github.com/waldirborbajr/urlencode/actions/workflows/linter.yaml/badge.svg)](https://github.com/waldirborbajr/urlencode/actions/workflows/linter.yaml)
[![CI](https://github.com/waldirborbajr/urlencode/actions/workflows/ci.yaml/badge.svg)](https://github.com/waldirborbajr/urlencode/actions/workflows/ci.yaml)
[![Release](https://github.com/waldirborbajr/urlencode/actions/workflows/release.yaml/badge.svg)](https://github.com/waldirborbajr/urlencode/actions/workflows/release.yaml)
[![Security audit](https://github.com/waldirborbajr/urlencode/actions/workflows/audit.yaml/badge.svg)](https://github.com/waldirborbajr/urlencode/actions/workflows/audit.yaml)
[![Release to crates.io](https://github.com/waldirborbajr/urlencode/actions/workflows/crates.yaml/badge.svg)](https://github.com/waldirborbajr/urlencode/actions/workflows/crates.yaml)

## Overview

A simple command line tool written in Rust programming language. It serializes to and [not yet] deserializes, from the application/x-www-form-urlencoded format.

## Usage

To use urlencode, follow these steps:

1. Navigate to the root directory of your Git repository in the terminal.

2. Run the following command:

```bash
urlencode
```

This command will execute the tool and perform the following operations:

1. Add all files recursively to the Git repository.
2. Commit all changes with a randomly generated commit message.
3. Push the changes to the remote repository (origin main branch).

## Installation

### Pre-Built Binary

Each release comes with pre-built binaries of several platforms. Grab it from [Github Releases](https://github.com/waldirborbajr/urlencode/releases).

### From source

Make sure you have Rust installed, then:

To build and install this, you'll need Rust and Cargo installed on your system. If you haven't already, you can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can proceed with the following steps:

1. Clone the repository:

```bash
git clone https://github.com/waldirborbajr/urlencode.git
```

2. Navigate to the project directory:

```bash
cd urlencode
```

3. Build the project using Cargo:

```bash
cargo build --release
```

4. Install the binary:

```bash
cargo install --path .
```

## Dependencies

- `clap`

- `urlencoding`

## Contributing to urlencode

If you are interested in contributing to urlencode, we would love to have your help! You can start by checking out the [ open issues ](https://github.com/waldirborbajr/urlencode/issues) on our GitHub repository to see if there is anything you can help with. You can also suggest new features or improvements by opening a new issue.

To contribute code to urlencode, you will need to fork the repository and create a new branch for your changes. Once you have made your changes, you can submit a pull request for them to be reviewed and merged into the main codebase.

## License

This project is released under the MIT License - see the [LICENSE](LICENSE) file for details.
