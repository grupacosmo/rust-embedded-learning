# Infinite loops
## C++
```c++
while (true) {

}
```

## Rust
```rust
loop {

}
```

# Breaking loops
## C++
```c++
while (true) {
    if (x > 0) {
        break;
    }
}
```

## Rust
```rust
loop {
    if x > 0 {
        break;
    }
}
```

# While loops
## C++
```c++
while (x > 0) {
    
}
```

## Rust
```rust
while x > 0 {

}
```

# For loops
## C++
```c++
for (int i = 0; i < 10; ++i) {
    
}

for (int i = 10; i >= 0; i -= 2) {
    
}
```

## Rust
```rust
for i in 0..10 {

}

// Rust's ranges have many more methods such as `rev` and `step_by`
for i in (0..10).rev().step_by(2) {

}
```