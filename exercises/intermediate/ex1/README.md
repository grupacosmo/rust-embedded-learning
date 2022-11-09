# Matrix abstraction

## Prerequisited
You've read chapters 1-5

## Exercise
Use `struct`s and `impl` blocks to write an abstraction for a dynamically sized matrix.

```rust
struct Matrix {
    data: Vec<f32>,
    width: usize,
    height: usize
}

impl Matrix {
    // constructor that fills matrix with zeros
    pub fn zeros(width: usize, height: usize) -> Self { ... }
    // get reference to single value of matrix
    pub fn get(&self, i: usize, j: usize) -> &f32 { ... }
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut f32 { ... }
    // get a slice of a row at index i
    pub fn row(&self, i: usize) -> &[f32]
    pub fn row_mut(&self, i: usize) -> &mut [f32]
    // multiply matrices
    pub fn add(&self, other: &Self) -> Self { ... }
    pub fn mul(&self, other: &Self) -> Self { ... }
}
```

## Hint
Size of `Vec` should be `width * height`. You can calculate index with `i + width * j`.