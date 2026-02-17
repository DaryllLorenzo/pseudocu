# pseudocu Interpreter

A pseudocode interpreter written in Rust. This project implements an interpreter for a pseudocode-like language designed to be intuitive and educational.

## Overview

pseudocu is an interpreter that allows writing and executing code in a pseudocode-like syntax. The language is designed to be easy to read and write, making it ideal for learning programming concepts, algorithm design, and quick scripting.

The interpreter is being developed in Rust, focusing on simplicity, clarity, and educational value.

## Quick Start

```bash
# Build the project
cargo build --release

# Run a program
cargo run --release <file.pc>
```

### Example

Create a file `example.pc`:

```
d = 5
a = 4
result = d + a
```

Run it:

```bash
cargo run --release example.pc
```

Output:
```
a = 4
d = 5
result = 9
```

## Language Features

### Variables and Assignments

Variables are identified by names starting with a letter or underscore, followed by letters, digits, or underscores.

```
x = 10
my_var = 42
_result = x + my_var
```

### Arithmetic Operators

| Operator | Description |
|----------|-------------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Integer Division |

### Comparison Operators

| Operator | Description | Returns |
|----------|-------------|---------|
| `==` | Equal | `1` if true, `0` if false |
| `!=` | Not equal | `1` if true, `0` if false |
| `>` | Greater than | `1` if true, `0` if false |
| `<` | Less than | `1` if true, `0` if false |
| `>=` | Greater than or equal | `1` if true, `0` if false |
| `<=` | Less than or equal | `1` if true, `0` if false |

### Operator Precedence

From highest to lowest:

1. Multiplication/Division (`*`, `/`)
2. Addition/Subtraction (`+`, `-`)
3. Comparison operators (`==`, `!=`, `>`, `<`, `>=`, `<=`)

## Language Specification

The complete language grammar is defined in Backus-Naur Form (BNF) in the file:

```
BNF.md
```

This file contains the formal specification of the pseudocu language syntax and structure.

**Formatting note in BNF.md:** In the BNF specification, the **pseudocu language symbols** (terminals) are shown in **bold**, while the _Backus-Naur metasymbols_ (such as `::=`, `|`, `< >`) and production rule names appear in normal text.

## Project Structure

```
pseudocu/
├── src/
│   ├── lexer.rs       # Tokenizer
│   ├── parser.rs      # Recursive descent parser
│   ├── ast.rs         # Abstract Syntax Tree definitions
│   ├── interpreter.rs # Expression evaluator and variable state
│   ├── main.rs        # CLI entry point
│   └── lib.rs         # Library exports
├── tests/
│   ├── lexer.rs       # Lexer tests
│   ├── parser.rs      # Parser tests
│   └── interpreter.rs # Interpreter tests
└── BNF.md             # Language grammar specification
```

## Running Tests

```bash
cargo test
```

## Roadmap

- [x] Repository initialization
- [x] Basic BNF grammar definition
- [x] Lexer implementation
- [x] Parser implementation
- [x] AST definition
- [x] Interpreter core
- [ ] Semicolon support for statement termination
- [ ] Control flow (if/else, loops)
- [ ] Functions
- [ ] Arrays/Lists
- [ ] String support
