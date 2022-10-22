# Functions that return nothing
## C++
```c++
void f() {

}
```

## Rust
```rust
// `fn` is a keyword used to declare functions
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
Passing by value doesn't always mean passing _by copy_. In Rust, values of simple datatypes are copied, while values of complex datatypes are _moved_ (more on that in future chapters).

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
We call the process of creating a reference _borrowing_. You can have multiple immutable references to the object at the same time, that's why immutable references are often called _shared references_.

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

let mut a = 0;
f(&mut a); // notice `&mut`
```
Rust has a additional rule at play. You cannot create a mutable reference if there exists any other reference (immutable or mutable) to the variable. That's why mutable references are often called _exclusive references_.

```rust
let mut a = 0;
let r: &mut i32 = &mut a;
f(&a);
f(r);
// error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
//  --> <source>:6:7
//   |
// 5 |     let r = &mut a;
//   |             ------ mutable borrow occurs here
// 6 |     f(&a);
//   |       ^^ immutable borrow occurs here
// 7 |     f(&r);
//   |       -- mutable borrow later used here
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