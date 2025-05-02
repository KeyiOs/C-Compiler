# C-Compiler
Compiler for C programming language to Assembly file written in Rust

## Project Overview
Stages of the compiler processing: 
- **Lexical Analysis (Lexer)**: Tokenizes the input C source code.
- **Parsing**: Builds an Abstract Syntax Tree (AST) from the tokenized input.
- **Semantic Analysis**: Validates the syntax and semantics of the program.
- **Intermediate Representation (IR)**: Converts the AST to an intermediate form.
- **Code Generation**: Generates assembly code based on the IR.

## Repository structure 
    c-compiler/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── examples/
    │   ├── primeNumbers.c
    │   └── reverseString.c
    ├── output/
    ├── tests/
    └── src/
        ├── main.rs
        ├── logic/
        │   ├── lexer.rs
        │   ├── parser.rs
        │   ├── semantic.rs
        │   ├── codegen.rs
        │   ├── ir.rs
        │   ├── mod.rs
        │   ├── sym_table.rs
        │   └── utils.rs
        └── data_structures/
            ├── ast.rs
            ├── objects.rs
            ├── mod.rs
            └── keywords.rs

#### `logic 📁`
Contains the core logic of the compiler:
- **`lexer.rs`**: Handles lexical analysis by breaking down the input C code into tokens.
- **`parser.rs`**: Converts tokens into an Abstract Syntax Tree (AST).
- **`semantic.rs`**: Performs semantic analysis, checking for errors like undeclared variables, type mismatches, etc.
- **`codegen.rs`**: Converts the AST into assembly code.
- **`ir.rs`**: Defines the Intermediate Representation (IR) used for simplifying optimizations and code generation.
- **`symbol_table.rs`**: Manages variable declarations and their types.
- **`utils.rs`**: Contains utility functions used across the compiler.

#### `data_structures 📁`
Contains data structures used by the compiler:
- **`ast.rs`**: Defines the structure of the Abstract Syntax Tree (AST).
- **`objects.rs`**: Contains data structures for representing tokens (keywords, operators, identifiers, etc.).
- **`keywords.rs`**: Handles keyword parsing and definitions.

## Supported Symbols
The compiler supports the following symbols as defined in the standard C23 version (October 31, 2024) of the C language:
```
| Symbol | Description           | Symbol | Description           |
|--------|-----------------------|--------|-----------------------|
| `&`    | Ampersand             | `+`    | Plus                  |
| `*`    | Asterisk              | `?`    | Question              |
| `\`    | Backslash             | `;`    | Semicolon             |
| `^`    | Caret                 | `'`    | Single Quote          |
| `:`    | Colon                 | `/`    | Slash                 |
| `,`    | Comma                 | `[`    | Square Bracket Left   |
| `.`    | Dot                   | `]`    | Square Bracket Right  |
| `"`    | Double Quote          | `~`    | Tilde                 |
| `=`    | Equal                 | `_`    | Underscore            |
| `!`    | Exclamation           | `{`    | Curly Bracket Left    |
| `>`    | Greater Than          | `}`    | Curly Bracket Right   |
| `#`    | Hash                  | `<`    | Less Than             |
| `-`    | Minus                 | `|`    | Pipe                  |
| `(`    | Parenthesis Left      | `)`    | Parenthesis Right     |
| `%`    | Percent               |        |                       |
```

## Valid Escape Sequences
The compiler also supports the following escape sequences as defined in the standard C23:

| Escape Sequence | Description                 |
|-----------------|-----------------------------|
| `\?`            | Question mark               |
| `\a`            | Audible bell                |
| `\b`            | Backspace                   |
| `\f`            | New page                    |
| `\n`            | New line                    |
| `\r`            | Carriage return             |
| `\t`            | Horizontal tab              |
| `\v`            | Vertical tab                |

<u>**Limitations**</u>

This compiler does **not** support *numeric escape sequences* and *Unicode*. Future updates may address these limitations to enhance compatibility with the C23 standard.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.