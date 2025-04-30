# C-Compiler
Compiler for C programming language to Assembly file using Rust

# Repository structure 
    c-compiler/
    ├── Cargo.toml
    ├── README.md
    ├── examples/
    │   └── hello.c
    ├── output/
    │   └── hello.asm
    ├── tests/
    │   └── integration.rs
    └── src/
        ├── main.rs
        ├── logic/
        │   ├── lexer.rs
        │   ├── parser.rs
        │   ├── semantic.rs
        │   ├── codegen.rs
        │   ├── ir.rs
        │   ├── symbol_table.rs
        │   └── utils.rs
        └── data_structures/
            ├── ast.rs
            ├── objects.rs
            └── keywords.rs
