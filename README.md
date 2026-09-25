# Expense CLI

A simple command-line expense tracker written in Rust.

This project was built as a Rust learning project to practice:

* Structs
* Vectors
* Pattern matching
* Command-line arguments
* File I/O
* Reading and writing data
* Error handling

## Features

* Add expenses
* List expenses
* Store expenses in a local file
* Load expenses when the application starts
* Simple command-line interface

## Requirements

* Rust
* Cargo

Check your installation:

```bash
rustc --version
cargo --version
```

## Installation

Clone the repository:

```bash
git clone https://github.com/thinkphp/expense-cli.git
cd expense-cli
```

Build the project:

```bash
cargo build --release
```

The executable will be available at:

```text
target/release/expense-cli
```

## Usage

### Add an expense

```bash
cargo run -- add 25.50 food
```

Example:

```text
Added expense!
```

Another example:

```bash
cargo run -- add 100 rent
cargo run -- add 15 coffee
```

### List expenses

```bash
cargo run -- list
```

Example output:

```text
1. 25.50 - food
2. 100.00 - rent
3. 15.00 - coffee
```

## Data Storage

Expenses are stored locally in:

```text
expenses.txt
```

Example:

```text
25.5|food
100|rent
15|coffee
```

The application loads the existing expenses when it starts and saves new expenses to the file.

## Project Structure

```text
expense-cli/
├── src/
│   └── main.rs
├── expenses.txt
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Roadmap

Possible future improvements:

* [ ] Delete an expense
* [ ] Edit an expense
* [ ] Calculate total expenses
* [ ] Filter by category
* [ ] Add dates
* [ ] Add monthly summaries
* [ ] Export to CSV
* [ ] Store data as JSON
* [ ] Improve error handling
* [ ] Add automated tests
* [ ] Add proper CLI argument parsing

## Learning Goals

The main goal of this project is to practice Rust by building a small, useful CLI application.

The project intentionally starts simple and will be expanded as new Rust concepts are learned.

## License

This project is open source and available under the MIT License.

