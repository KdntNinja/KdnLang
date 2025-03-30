# Getting Started with KdnLang

This guide will help you get started with KdnLang, including installation, writing your first program, and basic usage.

## Installation

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

## Your First KdnLang Program

Create a file called `hello.kdn` with the following content:

```kdn
fn main() {
    print("Hello, world!");
}
```

Run your program:

```sh
kdnlang run hello.kdn
```

## Basic Usage

### Running a Script

```sh
kdnlang run script.kdn
```

### Using the REPL (Interactive Mode)

```sh
kdnlang repl
```

### Compiling a Program (Future Feature)

```sh
kdnlang build script.kdn
```

## Next Steps

- Explore the [Syntax Guide](./syntax.md)
- Learn about the [Standard Library](./stdlib.md)
- Check out the [Language Guide](./language_guide/index.md) for in-depth documentation
