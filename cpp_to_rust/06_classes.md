# Structs
## C++
```c++
struct Widget {
    int x;
    int y;
};
```

## Rust
```rust
struct Widget {
    // `pub` means public
    pub x: i32,
    pub y: i32
}
```

# Classess
## C++
```c++
class Widget {
    int x;
    int y;
};
```

## Rust
Rust doesn't have a `class` keyword, but you can use structs as classes. Fields in structs are private by default.
```rust
struct Widget {
    x: i32,
    y: i32
}
```

# Static methods
## C++
```c++
class Widget {
   public:
    static void f() {
        
    }
};
```

## Rust
```rust
struct Widget;

// rust has an `impl` blocks, where you can implement methods for structs
impl Widget {
    // methods are static by default
    pub fn f() {

    }
}
```

# Constructors
## C++
```c++
class Widget {
    int x;

   public:
    Widget(int y) : x(y) {

    }
};

const auto w = Widget { 0 };
```

## Rust
Rust doesn't have constructors, but static methods are often used to achieve the same thing.
```rust
struct Widget {
    x: i32
}

impl Widget {
    // You can name the "constructor" anything you like,
    // `new` is a common convention
    pub fn new(y: i32) -> Widget {
        // Creates an object of Widget with `x` equal `y`
        Widget { x: y }
    }
}

let w = Widget::new(0);
```

# Const methods
## C++
```c++
class Widget {
    int x;

   public:
    void f() const {
        // cannot change x
    }
};
```

## Rust
```rust
struct Widget {
    x: i32
}

impl Widget {
    // `&self` is a reference to the object implementing this method
    pub fn f(&self) {
        // cannot change self.x
    }
}
```

# Non-const methods
## C++
```c++
class Widget {
    int x;

   public:
    void f() {
        x = 5;
    }
};
```

## Rust
```rust
struct Widget {
    x: i32
}

impl Widget {
    pub fn f(&mut self) {
        *self.x = 5;
    }
}
```

# Inheritance
## C++
```c++
struct Base {
    int x;
};

struct Widget : Base {
    int y;
};
```

## Rust
Rust doesn't support inheritance. The best you can do is composition.
```rust
struct Base {
    pub x: i32
}

struct Widget {
    pub base: Base,
    pub y: i32
}
```
