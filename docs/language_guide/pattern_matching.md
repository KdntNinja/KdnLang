# Pattern Matching in KdnLang

Pattern matching is a powerful feature in KdnLang that allows you to match values against patterns and extract information from them.

## Basic Match Statement

The `match` statement in KdnLang is similar to Rust's pattern matching:

```kdn
let value = 3;

match value {
    1 => print("One"),
    2 => print("Two"),
    3 => print("Three"),
    _ => print("Other"),
}
```

The `_` pattern is a catchall that matches any value.

## Matching on Ranges

You can match on ranges of values:

```kdn
let age = 25;

match age {
    0..=12 => print("Child"),
    13..=19 => print("Teenager"),
    20..=65 => print("Adult"),
    _ => print("Senior"),
}
```

## Matching with Guards

You can add extra conditions to patterns using guards:

```kdn
let number = 15;

match number {
    n if n % 2 == 0 => print("Even"),
    n if n % 2 == 1 => print("Odd"),
    _ => print("Not a number"), // This case won't be reached for any number
}
```

## Destructuring Tuples

You can destructure tuples in a match statement:

```kdn
let point = (10, 20);

match point {
    (0, 0) => print("Origin"),
    (0, y) => print("On the y-axis at " + str(y)),
    (x, 0) => print("On the x-axis at " + str(x)),
    (x, y) => print("At point (" + str(x) + ", " + str(y) + ")"),
}
```

## Destructuring Structs

You can destructure structs to extract and match on their fields:

```kdn
struct Person {
    name: String,
    age: i32,
}

let person = Person { name: "John", age: 30 };

match person {
    Person { age: 0..=18, .. } => print("Minor"),
    Person { name, age } => print(name + " is " + str(age) + " years old"),
}
```

## Pattern Binding

You can bind variables to parts of patterns:

```kdn
let pair = (10, 20);

match pair {
    (x, y) if x == y => print("Equal coordinates"),
    (x, y) if x + y == 30 => print("Sum is 30"),
    (x, y) => print("Coordinates: " + str(x) + ", " + str(y)),
}
```

## Using Match Expressions for Values

A match statement can be used as an expression to return a value:

```kdn
let number = 5;

let message = match number {
    1 => "One",
    2 => "Two",
    3 => "Three",
    4 => "Four",
    5 => "Five",
    _ => "Other",
};

print(message);  // Prints "Five"
```

Pattern matching is a concise and powerful way to handle different cases in your code, especially when combined with KdnLang's type system.
