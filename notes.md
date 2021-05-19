# Rust

## Introduction

Systems Programming Language, pursuing _safety, concurrency, speed_ \

- Safety: guaranteed at compile time
- Concurrency: fearless, because things are safe
- Speed: zero cost abstractions

Best of both worlds: safety like high level scripting languages (Ruby, Python), speed like system languages like C, C++.

### History

- 2006 started as a personal project by Mozilla developer
- Mozilla starts sponsoring Rust officially in 2009
- v1.0 was released in 2015
- 2017: Firefox Quantum core rewritten in Rust (less buggy, twice as fast)

## Cargo

Cargo is a package manager: yes, an actual package manager for a systems programming language! But also:

- Build system
- Test runner
- Docs generator

## Installation

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## New project

`cargo new hello`
This creates the following:

- `src` directory with `main.rs`, a Rust source file.
- `Cargo.toml`: a config file, `toml` stands for _Tom's Obvious Minimal Language_

From the `hello` directory, run `cargo run`. This will compile the project and run. If you run `cargo run` again, Cargo will not compile again, because nothing has changed!

Cargo runs `target/debug/hello`. Cargo builds with debug symbols by default. Run `cargo run --release` to compile without debug symbols, and make it a lot faster to run, though a bit longer to compile. Now the result will be stored in `target/release`.

## Variables

Rust intentionally mimics as much syntax from other languages as it can; C and Python in particular.

Rust is a strongly typed language, but often the types can be inferred.

```rust
fn main() {
  let bunnies = 2;
  let carrots: i32 = 6;
  let (chickens, eggs) = (3, 10); // Tuple pattern
}
```

Variables are immutable by default:

```rust
fn main() {
  let bunnies = 2; // Change to let mut bunnies = 2;
  bunnies = 6; // Error!
  println!("bunnies = {}", bunnies);
  let (mut cow, milk) = (4, 2); // In tuples you add mut to elements of the tuple separately
  cow = 6;
}
```

### Constants

- `const` instead of `let`;
- SCREAMING_SNAKE_CASE for the names;
- Type annotation is required;
- Value must be a constant expression that can be determined at compile time.

```rust
const WARP_FACTOR: f64 = 9.9;
```

## Scope

```rust
fn main() {
  let x = 2;
  {
    let y = 6;
    println!("{}, {}", x, y);
  }
  println!("{}, {}", x, y); // Error at compile time
}
```

### Shadowing

```rust
fn main() {
  let x = 2;
  {
    let x = 6;
    println!("{}", x);
  }
  println!("{}", x);
}
```

```rust
fn main() {
  let mut x = 2;
  let x = 6; // Also valid, x is now immutable
}
```

```rust
fn main() {
  let description = "This is a picture of a cow";
  let description = make_image(description); // You can even shadow a variable to a different type
}
```

## Memory safety

Rust guarantees memory safety at compile time. Variables must be initialized before you use them, otherwise you get a compile time error `use of possibly uninitialized variable ...`. So an initialization in an `if` statement will give an error, whereas an `if...else` statement will work.

## Functions

Rust uses `snake_case` for function names. Parameters are defined with name, colon, type, and separated by commas. The return type of a function is specified by an arrow `->` and the type, after the parameters.

```rust
fn do_stuff(qty: f64, oz: f64) -> f64 {
  // return qty * oz;
  qty * oz // When leaving the semicolon off the last expression in a block, then it will be returned (tail expression)
}
```

In other words `{ return true; }` is the same as `{ true }`.  
When calling a function, you have to provide all the values in the correct order.

### Variable numbers of arguments

A single Rust function does not support variable numbers of arguments, or different types of the same argument. **Macros** do support this. A macro call looks just like a function call, but macro names always end with `!`.

## Module System

src/lib.rs

```rust
pub fn greet() { // Functions are private by default
  println!("Hi!!!");
}
```

src/main.rs

```rust
use hello::greet; // Path is the name of your project in Cargo.toml

fn main() {
  greet();
  // hello::greet(); would also work, but first importing is more concise
}
```

```rust
use std::collections::HashMap; // Use standard, collections, HashMap
```

Package registry: https://crates.io/  
Add the package to the `[dependencies]` in your Cargo.toml: `rand = "0.6.5"`.

## Scalar types

### Integers

Underscores in integer values are ignored: `1_000_000` is equivalent to `1_0_00_0_00` or `1000000`.

### Floating point numbers

Floating point numbers (`f32` or `f64`) always need at least one digit before the dot.

### Booleans

`true` and `false`.

### Characters

A character in Rust is 4 bytes, which allows a single character to be a complex symbol (UCS-4/UTF-32) They are specified using single quotes:

```rust
let my_letter = 'a';
let my_symbol = 'œ';
let my_emoji = '😀';
```

Strings are UTF-8. So strings don't use characters internally. So...

## Compound types

### Tuples

```rust
let numbers = (1, 6.43, 42);
let annotated_numbers: (u8, f64, i32) = (1, 6.43, 42);
let (one, six_point, answer) = annotated_numbers;

let number_0 = numbers.0;
let number_1 = numbers.1;
let number_2= numbers.2;
```

Currently, the arity of tuples is 12 at max.

### Arrays

```rust
let buf = [1, 2, 3];
let buf_ann: [u8; 3] = [1, 2, 3];
let buf_zeroes = [0; 3]; // [0, 0, 0]

let one = buf[0];
```

Because arrays live on the stack, and are fixed size. Furthermore, they are limited to a size of 32, above which they lose some functionality. Therefore, we will more often use **Vectors**. More on this later.

## Control flows

Take notes

## Ownership

Three rules to ownership:

1. Each value has an owner;
2. There is only one owner of a value;
3. Value gets dropped, when its owner goes out of scope.

```rust
let s1 = String::from("abc");
let s2 = s1; // The value of s1 is moved to s2, because only one variable can own the value
println!("{}", s1); // Error! "Value borrowed here after move"
```

As soon as line two is executed, the compiler invalidates `s1`. It still exists on the stack, but the compiler considers `s1` uninitialized after its value is moved to `s2`.

```rust
let s1 = String::from("abc");
let s2 = s1.clone();
println!("{}", s1);
```

Rust uses the word 'copy' when only stack data is being copied. When heap data and pointer updates are involved, we use 'clone'.  
When a value is dropped (because owner goes out of scope):

1. Destructor
2. Free heap (no memory leaks)
3. Pop stack (no dangling pointers)

## References & Borrowing

```rust
let s1 = String::from("abc");
do_stuff(s1);
println!("{}", s1); // Error! s1 will be moved into local variable s in do_stuff

fn do_stuff(s: String) {
  ...
}
```

Instead we can use references:

```rust
let s1 = String::from("abc");
do_stuff(&s1);
println!("{}", s1);

fn do_stuff(s: &String) {
  ... // At the end of the function, the reference goes out of scope, and the reference gets dropped. Not the value!
}
```

Rust automatically handles the creation and destruction of pointers. It also makes sure that they're always valid using **lifetimes**, so we don't have to worry about pointers that much.

Lifetimes can be summed up as: references must always be valid. You can't create a reference, that outlives the data it's ultimately referencing, and you can never point to null.

References default to immutable, even if value they reference is mutable. But if both value and reference are mutable, then we can use the reference to change the value.

```rust
let mut s1 = String::from("abc");
do_stuff(&mut s1);
println!("{}", s1);

fn do_stuff(s: &mut String) {
  s.insert_str(0, "Hi, "); // The dot operator before a method or field auto-de-references down to the actual value!
  *s = String::from("Replacement!"); // But you can also de-reference in a similar way to C.
}
```

You can either have

- Exactly _one_ mutable reference, or
- Any number of immutable references.

The reason for this is to guarantee thread safety.
