# Static polymoprhism
## C++
```c++
// compiles for every T that has a `size` method.
template <typename T>
size_t add_sizes_of(const T& x, const T& y) {
    return x.size() + y.size();
}
```

## Rust
Rust doesn't have function templates, but it has a similar feature called generics. The difference between the two is that with generics we always need to define all the required operations that `T` needs to support in order to work with the generic function.

Let's see what happens if we don't specify required operations:
```rust
fn add_sizes_of<T>(x: &T, y: &T) -> usize {
    x.size() + y.size()
}
// error[E0599]: no method named `size` found for type parameter `T` in the current scope
//  --> <source>:2:7
//   |
// 1 | fn add_sizes_of<T>(x: T, y: T) -> usize {
//   |                 - method `size` not found for this type parameter
// 2 |     x.size() + y.size()
//   |       ^^^^ method not found in `T`
//   |
//   = help: items from traits can only be used if the type parameter is bounded by the trait
```
We have to bound T with a _trait_. Let's define it:

```rust
trait Size {
    fn size(&self) -> usize;
}
```
Now that we have our trait, we can use it to bound T in `add_sizes_of` function:
```rust
fn add_sizes_of<T: Size>(x: &T, y: &T) -> usize {
    x.size() + y.size()
}
```
or with alternative syntax:
```rust
fn add_sizes_of<T>(x: &T, y: &T) -> T
where
    T: Size
{
    x.size() + y.size()
}
```

`add_sizes_of` function will only accept implementors of the Size trait. Let's define a `Widget` type that implements `Size`:
```rust
struct Widget {
    text: String
}

impl Widget {
    fn new(text: String) -> Self {
        Self { text }
    }
}

impl Size for Widget {
    fn size(&self) -> usize {
        self.text.len()
    }
}

let a = Widget::new(String::from("c++"));
let b = Widget::new(String::from("rust"));
add_sizes_of(&a, &b); // 7
```

# Dynamic polymorphism
## C++
```c++
class Draw {
   public:
    virtual ~Draw() = default;
    virtual void draw() const = 0;
};

struct Widget : public Draw {
    std::string text;

   public:
    void draw() const override {
        std::cout << text;
    }
};
```
Whenever you declare a method as `virtual`, C++ automatically creates a pointer to _virtual table_ of function pointers inside the class. This has some memory costs for every object instance you create, even if you don't use it polymorphically.
```c++
std::cout << sizeof(std::string); // prints 32
std::cout << sizeof(Widget); // prints 40
// vtable ptr size is 8 bytes
```

## Rust
Rust doens't have inheritance, but we can use traits to accomplish dynamic dispatch too.
```rust
trait Draw {
    fn draw(&self);
}

struct Widget {
   text: String
}

impl Draw for Widget {
    fn draw(&self) {
        println!("{}", self.text);
    }
}
```
Rust doesn't generate virtual tables for the object until it takes part in dynamic polymorphism.
```rust
println!("{}", std::mem::size_of::<String>()); // prints 24
println!("{}", std::mem::size_of::<Widget>()); // prints 24
```

To achieve dynamic polymorphism over a trait in Rust and generate a virtual table you need to create a _trait object_.
```rust
let w: Box<dyn Draw> = Box::new(Widget::new());
```
```
   &dyn Draw       ╭──────> Widget        ╭─> vtable of Draw for Widget
 ┏━━━━━━━━━━━━━┓   │       ┏━━━━━━━━━┓    │        ┏━━━━━━━━━┓
 ┃ data        ┠───╯       ┃ text    ┃    │        ┃ draw()  ┃
 ┣━━━━━━━━━━━━━┫           ┗━━━━━━━━━┛    │        ┣━━━━━━━━━┫
 ┃ vtable ptr  ┠─────╮                    │        ┃ drop()  ┃
 ┗━━━━━━━━━━━━━┛     │                    │        ┣━━━━━━━━━┫
                     ╰────────────────────╯        ┃ size    ┃
                                                   ╏         ╏
```
Credits to [vptr crate](https://docs.rs/vptr/latest/vptr/) for this beautiful diagram.