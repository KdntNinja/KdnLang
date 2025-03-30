# Basic Types in KdnLang

KdnLang provides a variety of built-in types similar to Rust, with the simplicity of Python-like syntax.

## Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `i32` | 32-bit signed integer | `let age: i32 = 25;` |
| `f64` | 64-bit floating point | `let price: f64 = 19.99;` |
| `bool` | Boolean (true/false) | `let is_active: bool = true;` |
| `char` | Single Unicode character | `let grade: char = 'A';` |

## String Type

KdnLang provides a `str` type for text:

```kdn
let name: str = "KdnLang";
let greeting: str = "Hello, " + name + "!";
```

## Type Conversion

KdnLang provides simple methods for type conversion:

```kdn
let num_str: str = "42";
let num: i32 = num_str.parse();  // str to i32

let age: i32 = 25;
let age_str: str = age.to_string();  // i32 to str
```

## Type Inference

While KdnLang is statically typed, it supports type inference for local variables:

```kdn
// Type is inferred as str
let language = "KdnLang";

// Type is inferred as i32
let version = 1;
```

## Collections

### Arrays

Fixed-size collections of the same type:

```kdn
let numbers: [i32; 3] = [1, 2, 3];
let first = numbers[0];  // Accessing elements
```

### Lists

Dynamically sized collections:

```kdn
let fruits: List<str> = ["apple", "banana", "cherry"];
fruits.push("date");  // Adding elements
let fruit = fruits[1];  // Accessing elements
```

### Maps

Key-value collections:

```kdn
let scores: Map<str, i32> = {
    "Alice": 95,
    "Bob": 87,
    "Charlie": 92
};

let alice_score = scores["Alice"];  // Accessing values
scores["Dave"] = 88;  // Adding new entries
```

## Optional Types

KdnLang supports optional values similar to Rust's Option:

```kdn
let maybe_value: Option<i32> = Some(42);

match maybe_value {
    Some(value) => print("Got value: " + value.to_string()),
    None => print("No value present"),
}
```
