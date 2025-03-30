# KdnLang Standard Library

The KdnLang standard library provides a set of built-in functions and types that are available to all KdnLang programs without requiring imports.

## Basic Input/Output

### print()

Outputs a value to the console.

```kdn
print("Hello, world!");  // Prints a string
print(42);              // Prints a number
print(true);            // Prints a boolean
```

### input()

Reads a line of text from the console.

```kdn
let name: str = input("Enter your name: ");
```

## Type Conversion

### parse()

Converts a string to another type.

```kdn
let age_str: str = input("Enter your age: ");
let age: i32 = age_str.parse();
```

### to_string()

Converts a value to a string.

```kdn
let age: i32 = 30;
let age_str: str = age.to_string();
```

## String Manipulation

### concat()

Concatenates two strings.

```kdn
let full_name: str = concat("John", " Doe");
```

You can also use the `+` operator:

```kdn
let full_name: str = "John" + " Doe";
```

### length()

Returns the length of a string.

```kdn
let name: str = "KdnLang";
let len: i32 = length(name);  // Returns 7
```

## Collections

### Lists

Create and manipulate lists:

```kdn
let numbers: List<i32> = [1, 2, 3, 4, 5];
print(numbers[0]);  // Prints 1
numbers.push(6);    // Adds 6 to the end of the list
```

## Future Additions

The standard library is actively being developed. Future additions will include:

- File I/O functions
- More advanced collection types (maps, sets)
- Mathematical functions
- Network operations
- More string manipulation functions
