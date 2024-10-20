# Rust Semaphore Library

A lightweight, thread-safe implementation of a counting semaphore in Rust.

## Features

- Simple and efficient semaphore implementation
- Thread-safe using `std::sync` primitives
- Easy-to-use API with `new`, `acquire`, and `release` methods
- Includes basic test coverage

## Usage

```rust
let sem = Semaphore::new(2);
sem.acquire();
// Critical section
sem.release();
```

Perfect for managing access to limited resources in concurrent Rust applications.