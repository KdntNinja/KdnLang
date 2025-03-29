# Plan for Building KdnLang

## 1. Project Structure

- **src/**: Core implementation of the language.
  - `lexer.rs`: Tokenizes the input source code.
  - `parser.rs`: Parses tokens into an Abstract Syntax Tree (AST).
  - `type_checker.rs`: Ensures type correctness.
  - `interpreter.rs`: Executes the AST directly.
  - `compiler.rs`: Compiles the AST into machine code (optional).
  - `repl.rs`: Implements the Read-Eval-Print Loop for interactive use.
  - `main.rs`: Entry point for the language runtime.
  - `stdlib/`: Standard library modules.
- **examples/**: Example KdnLang scripts.
- **docs/**: Documentation for the language.
- **tests/**: Unit and integration tests.

---

## 2. Development Phases

### Phase 1: Core Components

1. **Lexer**
   - Tokenize the input source code into meaningful symbols (keywords, identifiers, literals, etc.).
   - Use a library like `logos` for efficient tokenization.

2. **Parser**
   - Build an Abstract Syntax Tree (AST) from the tokens.
   - Implement recursive descent parsing for simplicity.

3. **Type Checker**
   - Implement static type checking to ensure type safety.
   - Support basic types like `i32`, `f64`, `String`, and user-defined types.

4. **Interpreter**
   - Traverse the AST and execute the code directly.
   - Implement basic control flow, function calls, and variable management.

### Phase 2: Advanced Features

1. **Pattern Matching**
   - Add support for `match` expressions similar to Rust.

2. **Error Handling**
   - Implement Pythonic `try-except` blocks for error handling.

3. **Async/Await**
   - Add support for asynchronous programming with `async` and `await` keywords.

4. **Structs and Methods**
   - Implement Rust-style structs and methods for object-oriented programming.

### Phase 3: Standard Library

1. **Basic Utilities**
   - Implement common functions like `print`, `input`, and string manipulation.

2. **Collections**
   - Add support for lists, maps, and sets.

3. **File I/O**
   - Provide APIs for reading and writing files.

### Phase 4: Compiler (Optional)

1. **Code Generation**
   - Compile the AST into intermediate representation (IR).
   - Use LLVM or a similar backend for machine code generation.

2. **Optimizations**
   - Implement optimizations like constant folding and dead code elimination.

---

## 3. Tools and Dependencies

- **Rust**: Programming language for implementing KdnLang.
- **Logos**: Lexer library for tokenization.
- **LLVM**: Backend for code generation (optional).
- **Cargo**: Build system and package manager.

---

## 4. Milestones

1. **MVP (Minimum Viable Product)**
   - Lexer, parser, type checker, and interpreter.
   - Basic REPL for interactive use.

2. **Beta Release**
   - Add pattern matching, error handling, and async/await.
   - Provide a basic standard library.

3. **Stable Release**
   - Complete standard library.
   - Add compiler support (optional).

---

## 5. Testing and Documentation

- Write unit tests for each module.
- Create integration tests for end-to-end scenarios.
- Document the language syntax, features, and usage in the `docs/` folder.

---

## 6. Community and Contributions

- Set up a GitHub repository for the project.
- Provide contribution guidelines and a code of conduct.
- Encourage community feedback and contributions.
