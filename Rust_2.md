## Observations after exercises

The difficult parts:

- Quite a steep learning curve (as I was warned about)
- Different types of integers and working with them was not easy
- Understanding how to call methods/associated functions in `impl` was not straightforward
- Many errors can appear, which can work quite demotivating
- Not super easy to decide how to structure the code
- Many different types of arrays can be confusing

The fun parts:

- The fact that many compile time errors appear, follows partially from guaranteed memory safety
- Error messages are generally very clear
- After some struggles, I got some things working
- Many in-built solutions that can help
- Fun to be working with a newer language

## Why is it so popular?

Opinions that were found online:

- Compile time checking goes the extra mile:
  - Correct types
  - Thread safety
  - Ownership (automatic memory management at compile time)
- Extremely backwards compatible
- Great build system (Cargo) allows you to focus on code
- A hobbyist's choice?
- High challenge, high rewards
  - Easy problems are harder to get started on, but it doesn't create hard problems later on
- Passionate community
- Everyone will find something in Rust that will make them a better programmer

https://stackoverflow.blog/2020/06/05/why-the-developers-who-use-rust-love-it-so-much/  
https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/

## Enums

Enums allow you to define a type by enumerating its possible variants. In some situations, enums are more appropriate to use than structs.

Take the example of IP-addresses:

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
  // Do something
}
```

Though we now have the type of the IP-address, it still doesn't contain any data. We _could_ solve this using structs, but fortunately, Rust gives us another option:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Or as it is actually defined in the standard library:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

### The Option Enum

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

```rust
enum Option<T> {
    Some(T), // The Some variant of Option<T> can hold data of value T, which in this case can be any type of data
    None,
}
```

`T` can be any type. Here we have some examples of using the `Option` with strings and integers:

```rust
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;
  println!("{}", some_number.unwrap()); // Will print 5
  println!("{}", absent_number.unwrap()); // Error: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
  println!("{}", absent_number.unwrap_or_else(|| 3));
```

### Using enums in Vectors

Vectors `Vec<T>` allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.

This can sometimes be inconvenient, but don't worry! We can use enums as the value type and story different sorts of values:

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## The `match` control flow operator

The `match` control flow operator works in a similar way as the `switch` operator in languages like Java. It is _much_ more powerful, though. Patterns against which you match can be made up of literal values, variable names, wildcards, and many other things.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{}", get_ip_string(home));
}

fn get_ip_string(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(first, second, third, fourth) => first.to_string(),
        IpAddr::V6(address) => address,
    }
}
```

Matches must be exhaustive. You can use a `_` placeholder for matching any (remaining) value.

## Bit more on generics

In the previous meeting we talked about traits: interface like functionality, that can add generics to structs or other data types. It allowed you to write code that didn't need to know exactly what type of data it could expect, as long as it implemented certain functionality.

```rust
trait Summary {
    fn summarize(&self) -> String {
        // String::from("(Read more...)") // A trait can have a default implementation
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
} // We do not replace the default implementation

struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content) // Any implementation will replace the default implementation
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        headline: String::from("Oranje wins the European Championship"),
        location: String::from("London, UK"),
        author: String::from("Michiel Pauw"),
        content: String::from(
            "Maarten Stekelenburg stops two penalties in final.",
        ),
    }

    print_summary(&tweet);
    print_summary(&news_article);
}

fn print_summary<T: Summary>(item: &T) { // Also possible: pub fn print_summary(item: &impl Summary)
    println!("Breaking news! {}", item.summarize());
}
```

If you want a parameter to be bound by more than one trait, you can do that like this:

```rust
fn print_summary<T: Summary + Display>(item: &T) {
fn print_summary(item: &(impl Summary + Display)) {
```

Or with another alternative syntax:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

we can use a where clause, like this:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Example

Let's consider this piece of code, where we want to print the largest number from a list, but also the largest character from a list. We have two functions with (almost) exactly the same functionality. What should we do to improve this code?

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

### Another example

```rust
// enum IntValue {
//     Signed(i32),
//     Unsigned(usize),
// }

// trait GetWithAnyInt<T> {
//     fn getWithAnyInt(&self, value: &IntValue) -> Option<T>;
// }

// impl<T: Copy> GetWithAnyInt<T> for Vec<T> {
//     fn getWithAnyInt(&self, value: &IntValue) -> Option<T> {
//         match value {
//             IntValue::Signed(index) => None,
//             IntValue::Unsigned(index) => Some(self[*index])
//         }
//     }
// }

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let index_values = vec![
        IntValue::Signed(3),
        IntValue::Unsigned(4),
        // IntValue::Signed(-2),
        IntValue::Unsigned(2),
    ];

    for i in index_values.iter() {
        println!("{}", number_list[i]);
    }

    // for i in index_values.iter() {
    //     println!("{}", number_list.getWithAnyInt(i).unwrap_or_else(|| -1));
    // }
}
```

## Functional programming features

### Closures

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // Normal function
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure with annotated parameter type
let add_one_v3 = |x|             { x + 1 }; // Closure with inferred parameter type
let add_one_v4 = |x|               x + 1  ; // Closure without optional curly braces
```

For more information: https://doc.rust-lang.org/book/ch13-01-closures.html

## Still to cover

- Testing
- Concurrency
- Macros
