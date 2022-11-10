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
Use `io::stdin` function to obtain an object that represents the standard input stream.

```rust
use std::io;

fn main() {
    let stdin = io::stdin();
}
```

Then, use `read_line` to read a line from `stdin`.


```rust
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
}
```

Now, Rust will start to complain a little bit.
```
warning: unused `Result` that must be used
 --> src\main.rs:6:5
  |
6 |     stdin.read_line(&mut input);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this `Result` may be an `Err` variant, which should be handled
```
 
Rust tells us that `read_line` returned a value of type `Result` that may be an `Err` variant. `Result`s in Rust are used to report errors. To make the compiler happy, we will have to _handle_ the error somehow. For this exercise, we can just use the `expect` method to crash the aplication if an error occured.

```rust
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let result = stdin.read_line(&mut input);
    result.expect("Failed to read line");
}
```

Here is a more condensed version of the previous example:

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}
```

Ok, but what if you wanted to read an `i32` instead of a `String`? You'd need to _parse_ the input string to `i32`:

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input
        .trim() // removes '\n' attached to the end of the string
        .parse::<i32>() // parses String to the type specified inside the turbofish `::<>`
        .expect("Failed to parse input to i32");
}
```

Don't worry if you don't fully grasp how this works right now. All of the functionalities used here are well explained in the book.
