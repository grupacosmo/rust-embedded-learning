# std::cout
## C++
```c++
int x = 1;
const char* s = "string";
std::cout << x << " " << s << std::endl;
```

## Rust

```rust
let x: i32 = 1;
let s: &str = "string";
// `{}` is a placeholder for argument formatting
println!("{} {}", x, s);
```

# std::cin
## C++
```c++
std::string s;
std::cin >> s;
```

## Rust
```rust
let mut s = String::new();
std::io::stdin()
    .read_line(&mut s)
    .expect("Failed to read user's input");
```