# 🚀 KdnLang: A Statically-Typed, Pythonic Language with Rust-Like Syntax

## 📖 Overview

KdnLang is a modern programming language that combines **Rust's static typing** and **performance** with **Python's simplicity** and **ease of use**. It features:

- 🦀 **Rust-style strong typing** (`i32`, `f64`, `String`, etc.)
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
let name: String = input("Enter your name: ");
print("Hello, " + name + "!");

let age: i32 = input("Enter your age: ").parse();
print("You are " + str(age) + " years old.");
```

### 3️⃣ Functions & Control Flow

```kdn
fn greet(name: String) -> String {
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

### 6️⃣ Structs & Methods (Rust-style OOP)

```kdn
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        return Person { name, age };
    }

    fn greet(self) {
        print("Hello, my name is " + self.name);
    }
}

let user: Person = Person::new(input("Enter name: "), input("Enter age: ").parse());
user.greet();
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
4. Interpreter/Compiler: Executes the AST or compiles it into machine code.

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
kdnlang run script.kdn
```

---

## 📌 Roadmap

### Project Structure

```plaintext
KdnLang/
├── src/
│   ├── lexer.rs        # Tokenizer
│   ├── parser.rs       # AST generator
│   ├── type_checker.rs # Static type checking
│   ├── interpreter.rs  # Executes AST
│   ├── compiler.rs     # Bytecode/Machine code generation (Future)
│   ├── repl.rs         # Interactive REPL
│   ├── stdlib/         # Standard library
│   ├── main.rs         # Entry point
├── examples/           # Sample KdnLang scripts
├── docs/               # Documentation
├── tests/              # Unit tests
├── README.md           # Project documentation
└── Cargo.toml          # Rust package configuration
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

- [ ] Bytecode Compilation
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
