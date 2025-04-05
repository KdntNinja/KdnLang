# 🚀 KdnLang: A Simple Expression-Based Language

## 📖 Overview

KdnLang is a lightweight programming language designed for simplicity and readability. It currently supports:

- 🔢 **Variable declarations and assignments**
- 🔄 **For loops with range expressions**
- 🖨️ **Basic output via `print()`**
- ➗ **Mathematical expressions and operations**
- 📦 **Variable scoping within loops**

## 📝 Example Code

### 1️⃣ Variables & Expressions

```kdn
let num = 0;
num = num + 5;
print(num);
```

### 2️⃣ For Loops

```kdn
let sum = 0;
for i in 0..10 {
    sum = sum + i;
    print(sum);
}
```

### 3️⃣ Basic Arithmetic

```kdn
let x = 10;
let y = 5;
let result = x * y / 2 + 3;
print(result);
```

## 🔧 Implementation Details

### Project Architecture

The KdnLang interpreter is implemented in Rust with a modular architecture:

```files
KdnLang/
├── src/
│   ├── lexer.rs            # Token definitions using Logos
│   ├── error.rs            # Error handling with Miette
│   ├── grammar.pest        # Grammar definitions
│   ├── interpreter/        # Main language execution
│   │   ├── mod.rs          # Module exports
│   │   ├── execution.rs    # Primary interpretation logic
│   │   ├── context.rs      # Variable context handling
│   ├── parser/             # Expression parsing
│   │   ├── mod.rs          # Module exports  
│   │   ├── expression.rs   # Expression evaluation
│   ├── main.rs             # Program entry point
```

### How It Works

1. **Lexical Analysis**: Converts source code into tokens using the Logos crate
2. **Parsing**: Processes tokens into expressions and statements
3. **Execution**: Interprets the code with variable management and context handling
4. **Error Reporting**: Provides detailed, colorful error messages using Miette

## 🏗️ Installation & Usage

### Prerequisites

- Rust and Cargo (1.65 or later recommended)

### Building the Project

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/KdnLang
   cd KdnLang
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

### Usage

Run a KdnLang script:

```sh
cargo run -- --file your_script.kdn
```

Or if you've built the release version:

```sh
./target/release/kdnlang --file your_script.kdn
```

## 📌 Current Status & Roadmap

### Currently Implemented

- [x] Basic lexer and parser
- [x] Variable declarations and assignments
- [x] Mathematical expressions
- [x] For loops with ranges
- [x] Basic print statement
- [x] Error reporting with source highlights

### Upcoming Features

- [ ] Functions and return values
- [ ] Conditional statements (if/else)
- [ ] User input handling
- [ ] String manipulation
- [ ] Type annotations
- [ ] Extended standard library

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📜 License

MIT License © 2023 Kaiden Smith
