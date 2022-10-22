# Functions that return nothing
## C++
```c++
void f() {

}
```

## Rust
```rust
// `fn` is a keyword used for declaring functions
fn f() {

}
```

# Returning values
## C++
```c++
int f() {
    const auto a = 1;
    const auto b = 2;
    return a + b;
}
```

## Rust
```rust
fn f() -> i32 {
    let a = 1;
    let b = 2;
    // lack of `;` indicates that the value is returned
    a + b
}
```

# Passing parameters by value
## C++
```c++
int f(int a, int b) {
    return a + b;
}
```

## Rust
```rust
fn f(a: i32, b: i32) -> i32 {
    a + b
}
```

# Passing parameters by const reference
## C++
```c++
int f(const int& a) {
    // cannot change a
}

const int a = 0;
f(a);
```

## Rust
```rust
fn f(a: &i32) {
    // cannot change a
}

let a = 0;
f(&a); // notice `&`
```

# Passing parameters by non-const reference
## C++
```c++
int f(int& a) {
    a = 5;
}

int a = 0;
f(a);
```

## Rust
```rust
fn f(a: &mut i32) {
    // references in rust need to be dereferenced with `*` operator
    *a = 5;
}

let a = 0;
f(&mut a); // notice `&mut`
```

# Overloading
## C++
```c++
void f(int a) {

}

void f(float a) {
    
}
```

## Rust
Rust doesn't support function overloading, but one can accomplish similar effect with _traits_ (more on that later on).
```rust
fn f_i32(a: i32) {

}

fn f_f64(a: f64) {
    
}
```