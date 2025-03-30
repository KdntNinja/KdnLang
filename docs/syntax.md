# KdnLang Syntax Guide

## Overview

KdnLang is a statically-typed, compiled programming language that combines Rust's performance and type safety with Python's readability. This guide provides an overview of its syntax and features.

---

## Hello, World

```kdn
fn main() -> none {
    print("Hello, world!");
}
```

## Variables & Type Annotations

All variables in KdnLang require explicit type annotations for strong type safety:

```kdn
fn main() -> none {
    // String type
    let name: str = "KdnLang";
    
    // Integer type
    let version: i32 = 1;
    
    // Floating point type
    let pi: f64 = 3.14159;
    
    // Boolean type
    let is_compiled: bool = true;
    
    print("Hello from " + name + " version " + version.to_string() + "!");
}
```

## Functions & Type Signatures

Functions must declare parameter types and return types:

```kdn
// Function with typed parameters and return value
fn greet(name: str) -> str {
    return "Hello, " + name + "!";
}

fn main() -> none {
    let message: str = greet("KdnLang");
    print(message);
}
```

## Pattern Matching

```kdn
fn main() -> none {
    let age: i32 = 15;

    match age {
        0..=12 => print("You're a kid."),
        13..=19 => print("You're a teen."),
        _ => print("You're an adult."),
    }
}
```

## Error Handling

```kdn
fn main() -> none {
    try {
        let result: i32 = parse_number("abc");
        print("Number: " + result.to_string());
    } except {
        print("Invalid input! Please enter a number.");
    }
}

fn parse_number(input: str) -> i32 {
    // This would check if input is a valid number
    // and throw an exception if not
    return input.parse();
}
```

## Structs & Methods

```kdn
struct Person {
    name: str,
    age: i32,
}

impl Person {
    fn new(name: str, age: i32) -> Person {
        return Person { name, age };
    }

    fn greet(self) -> none {
        print("Hello, my name is " + self.name);
    }
    
    fn get_age(self) -> i32 {
        return self.age;
    }
}

fn main() -> none {
    let user: Person = Person::new("John", 30);
    user.greet();
    
    let age: i32 = user.get_age();
    print("Age: " + age.to_string());
}
```

## Async/Await

```kdn
async fn fetch_data() -> str {
    // Simulate network request
    return "Data loaded".to_string();
}

fn main() -> none {
    let result: str = await fetch_data();
    print(result);
}
```

## Compilation Process

KdnLang is a compiled language. Source code is processed through the following steps:

1. **Lexical Analysis**: The source code is tokenized
2. **Parsing**: Tokens are converted into an Abstract Syntax Tree (AST)
3. **Type Checking**: The AST is checked for type consistency
4. **Code Generation**: The AST is transformed into executable machine code
5. **Optimization**: (Optional) The code is optimized based on the selected optimization level

To compile a KdnLang program:

```sh
kdnlang compile input.kdn [output] [-O0|-O1|-O2|-O3]
```

Where:

- `input.kdn` is the source file
- `[output]` is the optional output file name (defaults to the input filename without extension)
- `-O0` to `-O3` are optimization levels (default is `-O0`, no optimizations)

You can also use the simplified command format:

```sh
kdnlang input.kdn [output] [-O0|-O1|-O2|-O3]
```

Or compile and immediately run:

```sh
kdnlang run input.kdn [-O0|-O1|-O2|-O3]
```
