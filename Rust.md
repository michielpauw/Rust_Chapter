# Rust

## Introduction

- Multi-paradigm...
- Focus on safety, speed, and concurrency...
- Accomplished by being memory safe without using garbage collection
  - Instead uses a _borrow checker_
- `.rs` extension
-

### History

- 2006: personal project by Mozilla employee Graydon Hoare
- Sponsored by Mozilla, announced in 2010
- First successfully compiled itself in 2011
- First stable release in 2015
- Current version (13 April): 1.51.0
- In 2020, most of Rust team at Mozilla laid off
- 2021:
  - Rust Foundation officially announced by five founding companies (AWS, Huawei, Google, Microsof, Mozilla)
  - Google announced support for Rust within Android Open Source Project (as an alternative to C/C++)

## Features

### Memory safety

Rust is designed to be memory safe. It does not permit:

- null pointers
- dangling pointers
- data races

The Rust core library provides an option type, to test whether a pointer has `Some` or `None` value.

### Memory management

- No automated garbage collection system
- Instead, memory and other resources are managed through Resource Acquisition is Initialization convention, with optional reference counting.

### Syntax

Superficially similar to C and C++:

- Curly brackets
- Control flow keywords like `if`, `else`, `while` and `for`

```rust
fn main() {
    println!("Hello, World!");
}
```

In a deeper sense closer to some functional programming languages:

- The `match` keyword is used for pattern matching
- Nearly every part of a function body is an expression
- A function need not end with a return expression

See this example of a recursive implementation of the factorial function:

```rust
fn factorial(i: u64) -> u64 {
    match i {
        0 => 1,
        n => n * factorial(n-1)
    }
}
```

Or this iterative implementation, using the `..=` operator to create an inclusive range:

```rust
fn factorial(i: u64) -> u64 {
    (2..=i).product()
}
```
