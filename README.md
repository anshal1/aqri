# Aqri

A small, simple compiled language built in Rust to understand how programming
languages work. The compiler pipeline is built step by step:

```
source (.aqri) → lexer → parser → typechecker → AST → assembly → assembler/linker → binary
```

Currently the **lexer**, **parser**, and a basic **AST** are implemented. The
next stage being added is **typechecking**.

## Compiler Flow

```
source (.aqri) → lexer → parser → typechecker → AST → assembly → assembler/linker → binary
```

**Current step:** We are currently working on the **typechecker** stage.

- ✅ Lexer — tokenizes source text
- ✅ Parser — builds the AST from tokens
- 🔨 Typechecker — validating types and variable declarations (in progress)
- ⬜ AST → Assembly — translating the AST into assembly code
- ⬜ Assembler/Linker — external tools that produce the final binary

## Project layout

```
src/
  main.rs            Entry point: reads input.aqri, runs lexer + parser, prints AST
  lexer.rs           Tokenizes source text into a stream of tokens
  parser.rs          Drives the statement parsers and builds the AST
  utils.rs           Small helpers (e.g. error formatting)
  statement_parser/
    let_parser.rs    Parses `let <name> <type> = <value>` statements
    print_parser.rs  Parses `prt(<name>)` print statements
```

## Running

```sh
cargo run
```

It reads `./input.aqri` and prints the debug representation of the parsed AST.

## Example (`input.aqri`)

```
let x int = 100
prt(x)
let z str = "test"
```

## Typechecker

The typechecker walks the AST and verifies that:

- Variable declarations are type-consistent (`let x int = 100` is fine,
  `let x int = "hi"` is an error).
- Variables used in expressions were previously declared (`prt(y)` before
  `let y ...` is an error).

## Design questions

The current code is intentionally minimal, but a few places feel rough and I'd
like your input on the right direction:

1. **AST shape.** The `AST` struct is a flat bag of `Option<String>` fields with
   one variant per statement kind. This does not scale to expressions, binary
   ops, or control flow. Would you prefer I refactor it into an explicit enum
   (e.g. `Statement::Let { ... }`, `Statement::Print { ... }`) now, before the
   code grows? Or keep the flat struct for now and revisit later?

2. **Error handling.** Currently parse errors are printed and parsing aborts.
   Would you like me to add error recovery (collect all errors and continue) or
   keep fail-fast for now?

**Your feedback on the best approach for each of these points would be greatly appreciated.**
