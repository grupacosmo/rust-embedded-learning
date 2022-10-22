# Structs
## C++
```c++
struct A {
    int x;
    int y;
};
```

## Rust
```rust
struct A {
    // `pub` means public
    pub x: i32,
    pub y: i32
}
```

# Classess
## C++
```c++
class A {
    int x;
    int y;
};
```

## Rust
Rust doesn't have a `class` keyword, but you can use structs as classes. Fields in structs are private by default.
```rust
struct A {
    x: i32,
    y: i32
}
```

# Const methods
## C++
```c++
class A {
    int x;

   public:
    void f() const {
        // cannot change x
    }
};
```

## Rust
```rust
struct A {
    x: i32
}

// rust has `impl` blocks where you can implement methods for structs
impl A {
    // `&self` is a reference to the object implementing the method
    pub fn f(&self) {
        // cannot change self.x
    }
}
```

# Non-const methods
## C++
```c++
class A {
    int x;

   public:
    void f() {
        x = 5;
    }
};
```

## Rust
```rust
struct A {
    x: i32
}

impl A {
    pub fn f(&mut self) {
        *self.x = 5;
    }
}
```