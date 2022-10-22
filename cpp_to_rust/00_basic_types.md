# Boolean
| C++           | Rust          |
| ------------- | ------------- |
| bool          | bool            |

# Integers
| C++           | Rust          |
| ------------- | ------------- |
| char          | i8            |
| unsigned char | u8            |
| short         | i16           |
| unsigned short| u16           |
| int           | i32           |
| unsigned int  | u32           |
| long          | i64           |
| unsigned long | u64           |
| ssize_t       | isize         |
| size_t        | usize         |

Note: The actual mapping may be different depending on the CPU architecture.

# Floating point numbers
| C++           | Rust          |
| ------------- | ------------- |
| float         | f32           |
| double        | f64           |

# Strings
| C++             | Rust          |
| --------------- | ------------- |
| char            | u8            |
| char8_t         | char          |
| const char8_t*  | &str          |
| std::u8string   | String        |

Note: `char8_t` and `std::u8string` in C++ exist for UTF-8 support. Rust supports UTF-8 by default.