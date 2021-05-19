# Notes

## Code

`println!` calls a Rust macro, a function would be withouth the `!`.

## Cargo

- Cargo: Rust package manager and build tool
- TOML (Tom's Obvious, Minimal Language): Rust's configuration format
- Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration
- files, and anything else not related to your code.
- `cargo build` from directory with `.toml` file: creates `Cargo.lock` (keeps track of exact dependencies)
- `cargo run` will compile if necessary, and run the binary
- `cargo check` will check whether the code compiles, but doesn't produce an executable
- `cargo build --release` will compile with optimizations

## Basics

- Rust needs to know at compile time what the type of a variable is, definitively
- By default, variables are immutable.
  - variables declared with `let` are immutable
  - variables declared with `let mut` are mutable
- You can declare constants with `const`
  - constants aren't immutable by default, they are always immutable
  - type of the value must be annotated
  - constants may only be set to a constant expression, not the result of a function or any other value that could only be computed at runtime
- You can declare a new variable with the same name as a previous variable:
  - you can change the type, which (of course) is different when using `let mut` and then re-assigning

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

## Data types

- Scalar vs Compound
- Statically typed: Rust must know the types of all variables at compile time
- Usually the compiler infers the type

### Scalar types

- Integers
  - 8/16/32/64/128 bit or arch and Signed or Unsigned
    - `i8/u8/i32/i28/u128`
- Floating-point numbers
- Booleans
- Characters

### Compound types

- Tuples

  - Tuples have a fixed length
  - Types in a tuple don't have to be the same
  - In the following example, optional type annotations were added:

    ```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    ```

  - We can use pattern matching to destructure a tuple:

    ```rust
    fn main() {
      let tup = (500, 6.4, 1); // Create tuple and bind it to variable tup

      let (x, y, z) = tup; // Turn tup into three separate variables x, y, and z (destructuring)

      println!("The value of y is: {}", y); // Print the value of y
    }
    ```

  - You can also access a tuple element directly like `tup.0`, `tup.1`, etc.

- Arrays

  - Just like tuples, arrays have a fixed length
    - An alternative is a vector, which does not have a fixed length. Unlike an array, a vector is allocated on the heap). When in doubt: use a vector.
  - Every element must have the same type

    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // With type annotation
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let b = [3; 5]; // An array with 5 elements, set to the value 3
    let first = a[0];
    let second = a[1];
    ```

  - Accessing an index that is out of bounds, will give a runtime error. Rust protects you from accessing invalid memory.

## Functions

- Functions start with `fn`
- Function names (just as variables) are in _snake case_ (`fn this_is_my_function() {...}`).
- In function signatures you must declare the type of each parameter
  - This is to guarantee that compiler almost never needs you to use them elsewhere in the code

### Statements and Expressions

- _Statements_ are instructions that perform some action and do not return a value
- _Expressions_ evealuate to a resulting value.

```rust
fn main() {
    let y = 6; // Statement
} // But also the function definition itself is a statement
```

```rust
fn main() {
    let x = (let y = 6);
}
```

This will give a compiler error, because the statement `let y = 6` does not return a value, so there is nothing for `x` to bind to.

This is unlike C or Ruby, where the assigment returns the value of the assignment (`x = y = 6` is valid in C).

Many other things are expressions: math operations, calling macros, calling functions. The block `{}` that creates new scopes is an expression:

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

Here the block

```rust
{
  let x = 3;
  x + 1
}
```

is an expression that evaluates to `4`.  
Note the `x + 1` line **without a semicolon**. Expressions do not include ending semicolons.

**If you add a semicolon to the end of an expression, you turn it into a statement, which will _not_ return a value.**

### Functions with Return values

A function can return a value like this:

```rust
fn five() -> i32 { // Use -> to declare the return type
    5 // The last value is returned implicitly. Do note: NO SEMICOLON
}
```

The following code will give a compiler error. Why?

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

This code implicitly returns an empty tuple `()`, which is not `i32` as indicated by the return type.  
You can also explicitly call `return`.

# Ownership

Ownership is
