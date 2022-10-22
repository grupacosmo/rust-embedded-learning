# Constants
## C++
```c++
const int x = 0;
```

## Rust
```rust
// `let` is a keyword used to declare variables.
let x: i32 = 0;
```

# Variables
## C++
```c++
int x = 0;
```

## Rust
```rust
// `mut` means mutable - liable to change
let mut x: i32 = 0;
```

# Arrays
## C++
```c++
int a[5] = { 1, 2, 3, 4, 5 };
```

## Rust
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
As opposed to C++, Rust arrays are "proper" types, which allows them to be easily passed and returned from functions. In C++, you would need to pass a pointer, or use `std::array`.

# Type inference
## C++
```c++
auto x = 0;
```

## Rust
Rust infers types by default.
```rust
let x = 0;
```