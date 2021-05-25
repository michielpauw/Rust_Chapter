# Rust

## Introduction

_Safety like high level scripting languages, speed like system languages..._

### History

- 2006: personal project by Mozilla employee Graydon Hoare
- Sponsored by Mozilla, announced in 2010
- First successfully compiled itself in 2011
- First stable release in 2015
- 2017: Firefox Quantum core rewritten in Rust (less buggy, twice as fast)
- In 2020, most of Rust team at Mozilla laid off
- 2021:
  - Rust Foundation officially announced by five founding companies (AWS, Huawei, Google, Microsoft, Mozilla)
  - Google announced support for Rust within Android Open Source Project (as an alternative to C/C++)
  - Stack Overflow's most loved programming language **five years in a row**
  - Current version (13 April): 1.51.0

### Notable features

- Memory safety and management (today)
- Ownership, borrowing, and references (today)
- Traits and polymorphism (introduction today)
- Concurrency (next session)
- Macros (maybe next session)

Rust is a compiled, multi-paradigm programming language, which supports imperative, concurrent, object-oriented and pure functional styles.

Let's be very vague about this:

- Rust is not a functional programming language, but...
  - certain aspects of functional programming are available and even encouraged (closures, higher order/first class functions);
  - other aspects are lacking (tail recursion). Many Rustaceans do use Rust in a functional way.
- Depending on the definition of Object Oriented Programming, Rust is or is not an Object Oriented Programming language
- Rust is a imperative programming language.

## Cargo

Cargo is Rust's build system and package manager. Cargo handles (among other things) building code, downloading dependencies, and building dependencies.

The dependencies for a Rust package are specified in a Cargo.toml file along with version requirements, telling Cargo which versions of the dependency are compatible with the package. By default, Cargo sources its dependencies from the user-contributed registry crates.io but Git repositories and packages in the local filesystem can be specified as dependencies, too.

### Installation

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Start a new project

`cargo new chapter_time`
This creates the following:

- `src` directory with `main.rs`, a Rust source file.
- `Cargo.toml`: a config file, `toml` stands for _Tom's Obvious Minimal Language_

From the `chapter_time` directory, run `cargo run`. This will compile the project and run. If you run `cargo run` again, Cargo will not compile again, because nothing has changed!

Cargo runs `target/debug/hello`. Cargo builds with debug symbols by default. Run `cargo run --release` to compile without debug symbols, and make it a lot faster to run, though a bit longer to compile. Now the result will be stored in `target/release`.

## Syntax and variables

```rust
fn main() {
    println!("Hello, World!");
}
```

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
  let mut hares = 5;
  hares = 8;
  let (mut cow, milk) = (4, 2); // In tuples you add mut to elements of the tuple separately
  cow = 6;
}
```

A tuple is a way of grouping together a number of values with a variety of types.

```rust
let numbers = (1, 6.43, 42);
let annotated_numbers: (u8, f64, i32) = (1, 6.43, 42);
let (one, six_point, answer) = annotated_numbers; // Destructure a tuple

let number_0 = numbers.0; // Or access elements using dot notation
let number_1 = numbers.1;
let number_2= numbers.2;
```

Currently, the arity of tuples is 12 at max.

### Constants

- `const` instead of `let`;
- SCREAMING_SNAKE_CASE for the names;
- Type annotation is required;
- Value must be a constant expression that can be determined at compile time.

```rust
const WARP_FACTOR: f64 = 9.9;
```

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

## Control flow

```rust
if num == 5 { // everything between if and curly braces is the condition
  msg = "five";
} else if num == 4 { // only accepts true boolean expressions (while as well)
  msg = "four";
} else { // curly braces are mandatory
  msg = "other";
}
```

`if` is an expression, not a statement:

```rust
msg = if num == 5 {
  "five" // note the absence of ;, so that values get returned as tail expressions
} else if num == 4 { // we can't even use return
  "four"
} else {
  "other"
}; // there is a ; at the end.
```

And then there are loops:

```rust
loop { // unconditional loop
  break;
}

'bob: loop { // you can use labels to break out of nested loops
  loop {
    loop {
      break 'bob; // without the label it will break out of inner loop
    }
  }
}

'bob: loop {
  loop {
    continue 'bob; // without the label it will continue the inner loop
  }
}

while dizzy() { // while is just syntactic sugar for adding a break condition on top of loop
  // do stuff
}

for num in [7, 8, 9].iter() { // iter() iterates over all items of a collection in order if collection is ordered, randomly if not
  // do stuff with num
}

let array = [(1,2), (3, 4)];

for (x, y) in array.iter() {
  // do stuff with x and y
}

for num in 0..50 { // use ..= for inclusive =
  // do stuff with num
}
```

## Memory management

### Scopes

Every data value in Rust has an owning scope. So first question to ask: what is a scope? A scope is represented by any block of code beginning with `{` and ending with `}`. The scope is the chunk of memory where the block's variables are stored.

When Rust is done with a scope, all of the data values that the scope owns are discarded and the memory that was used to store them is freed up for other uses. This includes memory that was allocated on the heap. The time between when a value is created and the time when its owning scope is done is called the lifetime of the value.

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

### Ownership

Three rules to ownership:

1. Each value has an owner;
2. There is only one owner of a value;
3. Value gets dropped, when its owner goes out of scope.

```rust
// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
  println!("Destroying a box that contains {}", c);

  // `c` is destroyed and the memory freed
}

fn use_int(n: u32) {
  println!("Using the copied integer {}", n);
}

fn main() {
  // _Stack_ allocated integer
  let x = 5u32;

  // *Copy* `x` into `y` - no resources are moved
  let y = x;

  use_int(y);

  // Both values can be independently used
  println!("x is {}, and y is {}", x, y);

  // `a` is a pointer to a _heap_ allocated integer
  let a = Box::new(5i32);

  println!("a contains: {}", a);

  // *Move* `a` into `b`
  let b = a;
  // The pointer address of `a` is copied (not the data) into `b`.
  // Both are now pointers to the same heap allocated data, but
  // `b` now owns it.

  // Error! `a` can no longer access the data, because it no longer owns the
  // heap memory
  // println!("a contains: {}", a);
  // TODO ^ Try uncommenting this line

  // This function takes ownership of the heap allocated memory from `b`
  destroy_box(b);

  // Since the heap memory has been freed at this point, this action would
  // result in dereferencing freed memory, but it's forbidden by the compiler
  // Error! Same reason as the previous Error
  //println!("b contains: {}", b);
  // TODO ^ Try uncommenting this line
}
```

But what happens when you want to pass a value to a function, but still be able to use it afterwards? Well, you _can_ return a tuple:

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But this is quite complicated...

### References and borrowing

Instead, we can use references:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

These ampersands are references, and they allow you to refer to some value without taking ownership of it. There are two main rules for references:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

This, for example, will not compile:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // This function returns a dangling pointer, which points to s, which will be dropped at the end of this scope
}
```

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

## Structs

Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.

```rust
struct Fox { // capital CamelCase
  name: String,
  enemy: bool,
  life: u8, // comma at the end is optional
}
```

To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.

```rust
// Instantiating is straightforward:
let red_fox = Fox {
  name: "Red fox",
  enemy: true,
  life: 70,
}
```

You can also implement an associated function to use as a constructor

```rust
impl Fox { // methods and associated functions are defined in an implementation block
  fn new(name: String) -> Self { // this is an associated function, because it doesn't have a form of Self as its first argument
    Self {
      name: name,
      enemy: true,
      life: 70,
    }
  }
  // methods take some form of self as their first argument
  fn some_method(self) { // can also be &self of &mut self for example
    println!("What does the fox say?");
  }
}


fn main() {
  let red_fox = Fox::new("Red fox".to_string()); // :: scope operator used to access namespace like things
  let life_left = red_fox.life;
  println!("I have {} years left to live.", life_left);
  red_fox.some_method(); // we use . syntax
}
```

## Traits

Traits are similar to interfaces.

```rust
struct Fox { // capital CamelCase
  name: "Red fox",
  enemy: bool,
  life: u8, // comma at the end is optional
}

// Traits define required behaviour
trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for Fox {
  fn get_noise(&self) -> &str { "Meow?" }
}
```

Using traits, we can start writing generic functions that accept any struct that implements a certain trait:

```rust
fn print_noise<T: Noisy>(item: T) {
  println!("{}", item.get_noise());
}

impl Noisy for u8 {
  fn get_noise(&self) -> &str { "BYTE!" }
}

fn main() {
  print_noise(5_u8);
  print_noise(red_fox);
}
```

A trait can inherit from another trait, and a trait can have default behaviours:

```rust
trait Run {
  fn run(&self) {
    println!("I'm running!");
  }
}

struct Robot {}
impl Run for Robot {}

fn main() {
  let robot = Robot {};
  robot.run();
}
```

Structured types are used to define fields. Implementations and traits cannot define fields themselves, and only traits can provide inheritance.

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

## Exercises

We will be solving puzzles! We will be focussing on creating an Intcode program, which is described in days 2, 5, 7, 9, and all other odd numbered days of the following page.

https://adventofcode.com/2019/day/2

I would recommend

https://doc.rust-lang.org/book/ch18-00-patterns.html
