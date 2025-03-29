# KdnLang Syntax Guide

## Overview

KdnLang is a statically-typed programming language that combines Rust's performance and Python's simplicity. This guide provides an overview of its syntax and features.

---

## Hello, World

```kdn
fn main() {
    print("Hello, world!");
}
```

## Variables & User Input

```kdn
fn main() {
    let name: String = input("Enter your name: ");
    print("Hello, " + name + "!");

    let age: i32 = input("Enter your age: ").parse();
    print("You are " + str(age) + " years old.");
}
```

## Functions & Control Flow

```kdn
fn greet(name: String) -> String {
    return "Hello, " + name + "!";
}

fn main() {
    print(greet(input("What's your name? ")));
}
```

## Pattern Matching

```kdn
fn main() {
    let age: i32 = input("Enter your age: ").parse();

    match age {
        1..=12 => print("You're a kid."),
        13..=19 => print("You're a teen."),
        _ => print("You're an adult."),
    }
}
```

## Error Handling

```kdn
fn main() {
    try {
        let age: i32 = input("Enter age: ").parse();
        print("Your age is " + str(age));
    } except {
        print("Invalid input! Please enter a number.");
    }
}
```

## Structs & Methods

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

fn main() {
    let user: Person = Person::new(input("Enter name: "), input("Enter age: ").parse());
    user.greet();
}
```

## Async/Await

```kdn
async fn fetch_data() -> String {
    return "Data loaded".to_string();
}

fn main() {
    let result: String = await fetch_data();
    print(result);
}
```
