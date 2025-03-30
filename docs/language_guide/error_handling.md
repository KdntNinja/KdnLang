# Error Handling in KdnLang

KdnLang combines Rust's robust error handling with Python's simplicity through its `try-except` mechanism.

## Basic Error Handling

Use the `try-except` block to catch and handle errors:

```kdn
try {
    let age: i32 = input("Enter your age: ").parse();
    print("Your age is " + str(age));
} except {
    print("Invalid input! Please enter a number.");
}
```

## Catching Specific Error Types

You can catch specific types of errors:

```kdn
try {
    let file = File::open("config.txt");
    let content = file.read_to_string();
    print(content);
} except FileNotFoundError {
    print("The file could not be found.");
} except IoError {
    print("An I/O error occurred.");
} except {
    print("An unknown error occurred.");
}
```

## Getting Error Information

You can capture the error object to access its details:

```kdn
try {
    let result = some_function_that_might_fail();
    print("Result: " + str(result));
} except e {
    print("Error occurred: " + str(e));
    print("Error type: " + str(e.type()));
}
```

## The `finally` Block

Use a `finally` block to execute code regardless of whether an error occurred:

```kdn
try {
    let file = File::open("data.txt");
    let content = file.read_to_string();
    process_data(content);
} except {
    print("Error processing file");
} finally {
    clean_up_resources();
}
```

## Propagating Errors

Functions can propagate errors to their callers using the `?` operator:

```kdn
fn read_config() -> Result<String, Error> {
    let file = File::open("config.txt")?;
    let content = file.read_to_string()?;
    return Ok(content);
}

fn main() {
    try {
        let config = read_config()?;
        print("Config loaded: " + config);
    } except {
        print("Could not load configuration.");
    }
}
```

## Creating Custom Errors

You can define your own error types:

```kdn
struct ValidationError {
    field: String,
    message: String,
}

impl Error for ValidationError {
    fn description(self) -> String {
        return "Validation error in field '" + self.field + "': " + self.message;
    }
}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        return Err(ValidationError {
            field: "age",
            message: "Age cannot be negative"
        });
    }
    if age > 150 {
        return Err(ValidationError {
            field: "age",
            message: "Age seems unrealistically high"
        });
    }
    return Ok(age);
}
```

KdnLang's error handling system ensures your programs can gracefully handle unexpected situations while keeping the code readable and maintainable.
