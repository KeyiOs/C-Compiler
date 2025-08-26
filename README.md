# C-Compiler

A compiler for the C programming language that outputs an Assembly (.asm) file, implemented entirely in Rust.

This compiler uses a **standalone preprocessor** that handles `#include` directives, macro expansion, and conditional compilation (`#if`, `#ifdef`, etc.). This design allows for clear separation of preprocessing and compilation, improving modularity and easier debugging.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Table of Contents

- [C-Compiler](#c-compiler)
- [License](#license)
- [Project Overview](#project-overview)
- [Repository Structure](#repository-structure)
  - [logic 📁](#logic-📁)
  - [data_structures 📁](#data_structures-📁)
- [Supported Symbols](#supported-symbols)
- [Reserved Keywords](#reserved-keywords)
- [Valid Escape Sequences](#valid-escape-sequences)
  - [Numeric Escape Sequences](#numeric-escape-sequences)
  - [Limitations](#limitations)
- [Functions and Implementation](#functions-and-implementation)
  - [Token](#token)
    - [Structure](#structure)
    - [Types](#types)
    - [Functions](#functions)

## Project Overview

Stages of the compiler processing: 
- **Preprocessing**: Handles macros and #include directives, producing expanded source code.
- **Lexical Analysis (Lexer)**: Converts the preprocessed source code into a stream of tokens.
- **Parsing**: Builds an Abstract Syntax Tree (AST) from the token stream.
- **Semantic Analysis**: Validates the syntax and semantics of the program.
- **Intermediate Representation (IR)**: Converts the AST to an intermediate form.
- **Code Generation**: Generates assembly code based on the IR.

## Repository structure 

    c-compiler/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── examples/
    │   ├── oddEven.c
    │   ├── oddEven2.c
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
        │   ├── sym_table.rs
        │   └── utils.rs
        └── data_structures/
            ├── ast.rs
            ├── objects.rs
            └── keywords.rs

#### <u>logic</u> 📁

Contains the core logic of the compiler:
- **`lexer.rs`**: Handles lexical analysis by breaking down the input C code into tokens.
- **`parser.rs`**: Converts tokens into an Abstract Syntax Tree (AST).
- **`semantic.rs`**: Performs semantic analysis, checking for errors like undeclared variables, type mismatches, etc.
- **`codegen.rs`**: Converts the AST into assembly code.
- **`ir.rs`**: Defines the Intermediate Representation (IR) used for simplifying optimizations and code generation.
- **`symbol_table.rs`**: Manages variable declarations and their types.
- **`utils.rs`**: Contains utility functions used across the compiler.

#### <u>data_structures</u> 📁

Contains data structures used by the compiler:
- **`ast.rs`**: Defines the structure of the Abstract Syntax Tree (AST).
- **`token.rs`**: Defines the token data structure, token types, and functions for working with tokens.
- **`keywords.rs`**: List of symbols and reserved keywords in the language.

## Supported Symbols

The compiler supports the following symbols as defined in the standard C23 version (October 31, 2024) of the C language:

| Symbol | Description           | Symbol | Description           |
|--------|-----------------------|--------|-----------------------|
| `&`    | Ampersand             | `*`    | Asterisk              |
| `\`    | Backslash             | `^`    | Caret                 |
| `:`    | Colon                 | `,`    | Comma                 |
| `.`    | Dot                   | `"`    | Double Quote          |
| `=`    | Equal                 | `!`    | Exclamation           |
| `>`    | Greater Than          | `<`    | Less Than             |
| `-`    | Minus                 | `(`    | Parenthesis Left      |
| `)`    | Parenthesis Right     | `%`    | Percent               |
| `\|`   | Pipe                  | `+`    | Plus                  |
| `?`    | Question              | `;`    | Semicolon             |
| `'`    | Single Quote          | `/`    | Slash                 |
| `[`    | Square Bracket Left   | `]`    | Square Bracket Right  |
| `~`    | Tilde                 | `{`    | Curly Bracket Left    |
| `}`    | Curly Bracket Right   |        |                       |

The compiler also processes the following multi-character symbols:

| Symbol  | Description            | Symbol  | Description              |
|---------|------------------------|---------|--------------------------|
| `&&`    | Double Ampersand       | `--`    | Double Minus             |
| `\|\|`  | Double Pipe            | `++`    | Double Plus              |
| `->`    | Pointer                | `>>`    | Double Greater Than      |
| `<<`    | Double Less Than       | `<=`    | Less Than Equal          |
| `>=`    | Greater Than Equal     | `==`    | Double Equal             |
| `!=`    | Exclamation Equal      | `+=`    | Plus Equal               |
| `-=`    | Minus Equal            | `*=`    | Asterisk Equal           |
| `/=`    | Slash Equal            | `%=`    | Percent Equal            |
| `<<=`   | Left Shift Equal       | `>>=`   | Right Shift Equal        |
| `&=`    | Ampersand Equal        | `^=`    | Caret Equal              |
| `\|=`   | Pipe Equal             |         |                          |

## Reserved Keywords

List of all reserved keyword identifiers:

```
• Auto  
• Bool, Break  
• Case, Char, Const, Constexpr, Continue  
• Default, Do, Double  
• Else, Enum, Extern  
• False, Float, For  
• Goto  
• If, Inline, Int  
• Long  
• Nullptr  
• Register, Restrict, Return  
• Short, Signed, Sizeof, Static, StaticAssert, Struct, Switch  
• ThreadLocal, True, Typedef, Typeof, TypeofUnqual  
• Union, Unsigned  
• Void, Volatile  
• While  
```

Compiler currently does not support these keywords:

```
_Alignas, _Alignof, _Atomic, _BitInt, _Bool, _Complex, _Decimal, _Generic, _Imaginary, _Noreturn, _Static_assert, _Thread_local
```

## Valid Escape Sequences

The compiler also supports the following escape sequences as defined in the standard C23:

| Escape Sequence | Description         |
|-----------------|---------------------|
| `\\`            | Backslash (`\`)     |
| `\'`            | Single quote (`'`)  |
| `\"`            | Double quote (`"`)  |
| `\a`            | Audible bell        |
| `\b`            | Backspace           |
| `\f`            | Form feed (new page)|
| `\n`            | New line (LF)       |
| `\r`            | Carriage return (CR)|
| `\t`            | Horizontal tab      |
| `\v`            | Vertical tab        |

### Numeric Escape Sequences

In addition to the basic escapes, the compiler also supports:

- **Hexadecimal escapes**:  
  `\xhh` — Represents a character by its hexadecimal value. Example: `\x41` → `'A'`.

- **Octal escapes**:  
  `\ooo` — Represents a character by its octal value. Example: `\101` → `'A'`.

- **Universal character names (UCNs)**:  
  These are used to represent Unicode characters:
  - `\uXXXX` — 4-digit hexadecimal Unicode (e.g., `\u03A9` → `'Ω'`)
  - `\UXXXXXXXX` — 8-digit hexadecimal Unicode

These sequences allow embedding special characters, Unicode symbols, and binary values directly into strings and character constants.

#### <u>Limitations</u>

- **Multi-character literals are not supported.**  
  Character constants must contain exactly **one character** after escape processing.  
  Constructs like `'abcd'` or `'\"abc'` are invalid and will result in a compilation error.  
  This restriction ensures consistent behavior and improves portability.

# Functions and Implementation

This section showcases internal components of the compiler, including code snippets and the core data structures.

### Navigation

- [Token](#token)
  - [Structure](#structure)
  - [Types](#types)
  - [Functions](#functions)

## Token

### Structure

```rust
pub struct Token {
    pub token_type: Option<TokenType>,
    pub line: usize,
    pub next: Option<Rc<RefCell<Token>>>,
    pub prev: Option<Rc<RefCell<Token>>>,
}
```

Represents a single token in the compiler’s lexer phase.

- `token_type`: An optional variant of `TokenType` representing the kind of token.
- `line`: The line number in the source code where this token appears.
- `next` / `prev`: Optional references to adjacent tokens, enabling both-way traversal.

### Types

```rust
pub enum TokenType {
    Keyword(String),
    Operator(String),
    Literal(String),
    Identifier(String),
}
```

Defines the different kinds of tokens the compiler recognizes:

- `Keyword(String)`: Reserved words in the language (e.g. `fn`, `if`, `let`).
- `Operator(String)`: Symbols representing operations (e.g. `+`, `-`, `==`).
- `Literal(String)`: Literal values such as numbers or strings (e.g. `"42"`, `"hello"`).
- `Identifier(String)`: Variable names, function names, etc. (e.g. `my_var`, `main`).

### Functions

- <u>Token::Init()</u>

  Initializes a new, empty `Token` instance.  
  The `token_type` is set to `None`, and `line` is set to a default (e.g., `0`).  
  Used for bootstrapping token chains or creating placeholder tokens.

  ```rust
  mut Token = Token::init();
  ```

- <u>Token::Set()</u>

  Assigns a specific token type and line number to an existing `Token`.

  ```rust
  Token = Token::set(Token, TokenType::<Variant>(<StringValue>), <Line>);
  ```

  ##### Parameters:
  - `token`: The target `Token` instance (usually mutable).
  - `TokenType::<Variant>`: The token classification to apply. Supported variants include:
    - `Keyword("if".to_string())`
    - `Operator("+".to_string())`
    - `Literal("42".to_string())`
    - `Identifier("my_var".to_string())`
  - `line`: Line number where the token occurs in source code.

  ##### Example:

  ```rust
  token = Token::set(token, TokenType::Keyword("if".to_string()), 1);
  token = Token::set(token, TokenType::Identifier(buffer.clone()), 1);
  token = Token::set(token, TokenType::Literal(value.to_string()), 1);
  ```
