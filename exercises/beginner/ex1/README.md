# Command-line binary calculator

## Prerequisites
You need to know:
* how to create and run an app
* how to print
* variables
* types
* control flow (if/else etc.)

## Exercise

### Input
Program expects 3 inputs from `stdin`:
1. Operator - "+" | "-" | "*" | "/" | "%"
2. Left operand - any number
3. Right operand - any number (non zero if operator is '/')


### Output
Result of given calculation

## Hints
Use `io::read_to_string` function from Rust 1.65. Run `rustup update` just to be sure that you have it.

`io::read_to_string` can fail to read the input string, thus it doesn't return the `String` directly. It returns an _object_ of type `Result<String, io::Error>` instead. You can read it as "`Result` containing either a `String` or an `io::Error`". Don't worry about the details of `Result` type for now.

```rust
use std::io;

fn main() {
    let result: Result<String, io::Error> = io::read_to_string(io::stdin());

    // Since the `String` is inside the `Result` object, it needs to be unwrapped.
    // You can do that with an `expect` method.
    // If the `String` is not present inside the Result object, the application will crash
    // displaying provided message.
    let input: String = result.expect("Failed to read input");
}
```

This may look weird at first, but `Result` is a neat way to ensure that you don't forget to handle the error.

Here is a simplified version of the previous example.

```rust
use std::io;

fn main() {
    // we call `expect` directly on the returned value
    let input = io::read_to_string(io::stdin()).expect("Failed to read input");
}
```

Ok, but what if you wanted to read an `i32` instead of a `String`? You'd need to _parse_ the input string to `i32`.

```rust
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin())
        .expect("Failed to read input")
        .trim() // removes '\n' attached to the end of the string
        .parse::<i32>() // parses String to the type specified in the turbofish `::<>`
        .expect("Failed to parse input");
}
```