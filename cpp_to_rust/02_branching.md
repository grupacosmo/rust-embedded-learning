# If statements
## C++
```c++
if (x > 0) {

} else if (x == 0) {

} else {

}
```

## Rust
```rust
if x > 0 {

} else if x == 0 {

} else {

}
```
In Rust, `if`s are not statements, but expressions, which allows them to yield a value. This is not possible in C++.
```rust
let x = if x > 0 {
    1
} else if x == 0 {
    2
} else {
    3
}
```

# switch statements
## C++
```c++
int result;
switch (x) {
    case 0:
        result = f();
        break;
    case 1:
    case 2:
        result = g();
        break;
    default:
        result = h();
}
```

## Rust
Rust doesn't have `switch` statements, but it has `match` expressions.
```rust
let result = match x {
    0 => f(),
    1 | 2 => g(),
    _ => h()
}
