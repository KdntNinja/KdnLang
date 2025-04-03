# 🚀 KdnLang: A Statically-Typed, Pythonic Language with Rust-Like Syntax

## 📖 Overview

KdnLang is a modern programming language that combines **Rust's static typing** and **performance** with **Python's simplicity** and **ease of use**. It features:

- 🦀 **Rust-style strong typing** (`i32`, `f64`, `str`, etc.)
- 🐍 **Python-like user interaction** (`print()`, `input()`)
- 🛠️ **Garbage collection** (no manual memory management)
- 🏗️ **Pattern matching** (`match` like Rust)
- 🔄 **Pythonic `try-except` error handling**
- ⚡ **Async/await support** (Rust-style but simple like Python)

---

## 📝 Example Code

### 1️⃣ Hello, World

```kdn
print("Hello, world!");
```

### 2️⃣ Variables & User Input

```kdn
let name: str = input("Enter your name: ");
print("Hello, " + name + "!");

let age: i32 = input("Enter your age: ").parse();
print("You are " + str(age) + " years old.");
```

### 3️⃣ Functions & Control Flow

```kdn
fn greet(name: str) -> str {
    return "Hello, " + name + "!";
}

print(greet(input("What's your name? ")));
```

### 4️⃣ Pattern Matching

```kdn
let age: i32 = input("Enter your age: ").parse();

match age {
    1..=12 => print("You're a kid."),
    13..=19 => print("You're a teen."),
    _ => print("You're an adult."),
}
```

### 5️⃣ Error Handling (Pythonic `try-except`)

```kdn
try {
    let age: i32 = input("Enter age: ").parse();
    print("Your age is " + str(age));
} except {
    print("Invalid input! Please enter a number.");
}
```

### 6️⃣ For loops (Rust-style)

```kdn
for i in 0..11 {
   print(i);
}
```

### 7️⃣ Async/Await (Rust-Style)

```kdn
async fn fetch_data() -> String {
    return "Data loaded".to_string();
}

let result: String = await fetch_data();
print(result);
```

---

## 🔧 How It Works

### ✅ Compilation & Execution Steps

1. Lexer: Converts source code into tokens.
2. Parser: Builds an Abstract Syntax Tree (AST).
3. Type Checker: Ensures Rust-like static typing.
4. Interpreter: Executes the AST.

---

## 🏗️ Installation & Usage

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/KdntNinja/KdnLang
   cd KdnLang
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Add the binary to your PATH (optional):

   ```sh
   export PATH="$PATH:$(pwd)/target/release"
   ```

### Usage

Run a KdnLang script:

```sh
kdnlang --file script.kdn
```

---

## 📌 Roadmap

### Project Structure

```plaintext
KdnLang/
├── src/
│   ├── lexer/         # Tokenizer
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   ├── parser/        # AST generator
│   │   ├── mod.rs
│   │   ├── parser.rs
│   ├── typechecker/   # Static type checking
│   │   ├── mod.rs
│   │   ├── typechecker.rs
│   ├── interpreter/  # Executes AST
│   │   ├── mod.rs
│   │   ├── interpreter.rs
│   ├── main.rs         # Entry point
├── examples/           # Sample KdnLang scripts
├── tests/              # Unit tests
├── Cargo.toml          # Rust package configuration
```

### Development Milestones - TODO

#### Phase 1: Core Language Features

- [x] Design Syntax & Grammar
- [x] Implement Lexer (Tokenization)
- [x] Implement Parser (AST)
- [ ] Implement Type Checker
- [ ] Implement Interpreter
- [ ] Develop Standard Library

#### Phase 2: Performance & Tooling

- [ ] REPL for Interactive Coding
- [ ] Debugging & Error Messages
- [ ] IDE & LSP Support

#### Phase 3: Advanced Features

- [ ] Concurrency (Threads & Async)
- [ ] Foreign Function Interface (FFI)
- [ ] Package Manager

---

## 🤝 Contributing

Want to help? Open an issue or submit a pull request!

---

## 📜 License

MIT License © 2025 Kaiden Smith
