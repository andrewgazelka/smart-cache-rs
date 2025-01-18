# smart-cache

[![Crates.io](https://img.shields.io/crates/v/smart-cache.svg)](https://crates.io/crates/smart-cache)
[![Documentation](https://docs.rs/smart-cache/badge.svg)](https://docs.rs/smart-cache)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md)
[![Issues](https://img.shields.io/github/issues/andrewgazelka/smart-cache)](https://github.com/andrewgazelka/smart-cache/issues)

A Rust library for smart function caching with automatic invalidation. Cache expensive function calls with a simple attribute macro.

This is a Rust implementation of the smart caching concept, inspired by the Python version at [smart-cache](https://github.com/andrewgazelka/smart-cache). While the Python version pioneered the concept of AST-based cache invalidation, this Rust implementation takes advantage of Rust's zero-cost abstractions and powerful macro system to provide similar functionality with near-native performance.

## Features

- Simple `#[cached]` attribute macro for function caching
- Persistent caching using [redb](https://github.com/cberner/redb)
- Zero-copy serialization with [rkyv](https://github.com/rkyv/rkyv) for maximum performance
- Automatic cache invalidation based on compile-time function AST analysis
- Thread-safe
- Support for complex function arguments
- Near in-memory performance with memory-mapped I/O

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
smart-cache = "0.1"
```

## Key Features

### Persistent Caching
Unlike in-memory caches, smart-cache persists results between program runs using redb's memory-mapped storage. Once a function result is cached, it remains available even after restarting your application, with near in-memory access speeds thanks to zero-copy reads.

### Smart Invalidation
The library analyzes the Abstract Syntax Tree (AST) of your cached functions at compile-time, generating a SHA-256 hash. If you modify the function's implementation, smart-cache automatically detects the change and invalidates the cache. This ensures you never get stale results when the function logic changes, with zero runtime overhead for invalidation checks.

### High Performance
The combination of rkyv's zero-copy serialization and redb's memory-mapped storage provides exceptional performance:
- Near-native speed for cache hits through memory mapping
- Zero-copy deserialization of cached values
- Efficient concurrent access with MVCC
- Optimized for modern SSD storage

## Example

```rust
use smart_cache_macro::cached;

// First run of your program:
#[cached]
fn expensive_computation(x: String, y: i32) -> String {
    println!("Computing...");  // We'll see when the function actually runs
    std::thread::sleep(std::time::Duration::from_secs(3));
    format!("example computation {}_{}", x, y)
}

fn main() {
    // First call: takes 3 seconds, prints "Computing..."
    let result1 = expensive_computation("hello".to_string(), 2);
    println!("{}", result1); // "example computation hello_2"

    // Second call: instant, no "Computing..." message
    let result2 = expensive_computation("hello".to_string(), 2);
    println!("{}", result2); // "example computation hello_2"
}

// If you restart your program, the cache persists:
fn main() {
    // Still instant, no "Computing..." message, uses cached result from previous run
    let result = expensive_computation("hello".to_string(), 2);
    println!("{}", result); // "example computation hello_2"
}

// If you modify the function, the cache invalidates automatically:
#[cached]
fn expensive_computation(x: String, y: i32) -> String {
    println!("Computing...");
    std::thread::sleep(std::time::Duration::from_secs(3));
    format!("new computation {}_{}", x, y)  // Changed the string
}

fn main() {
    // Cache was invalidated due to function change
    // Takes 3 seconds and prints "Computing..." again
    let result = expensive_computation("hello".to_string(), 2);
    println!("{}", result); // "new computation hello_2"
}
```

## How it Works

The `#[cached]` attribute macro automatically:
1. Uses [rkyv](https://github.com/rkyv/rkyv) for zero-copy serialization of function arguments and results, providing extremely efficient conversion to and from bytes
2. Generates a compile-time SHA-256 hash of the function's AST, which is used as part of the cache key to ensure cache invalidation when the function changes
3. Combines the function hash with serialized arguments to create a unique cache key
4. Stores results in a persistent [redb](https://github.com/cberner/redb) database, which offers several advantages over SQLite:
   - Memory-mapped file I/O for near in-memory performance
   - Zero-copy reads
   - ACID compliance with MVCC (Multi-Version Concurrency Control)
   - Optimized for SSD storage
5. Handles concurrent access safely through redb's built-in thread-safe transaction system

The combination of rkyv's zero-copy serialization and redb's memory-mapped storage means that cache hits are extremely fast, often approaching the speed of direct memory access. The use of compile-time function hashing ensures that cache invalidation has zero runtime overhead.

## License

Licensed under either of:
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
