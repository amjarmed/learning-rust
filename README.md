# learning-rust

ust is a powerful systems programming language that focuses on performance, memory safety, and concurrency. It has become increasingly popular for tasks like low-level system development, web assembly, and more.

## Resources

- website : <https://www.rust-lang.org/>
- repo : <https://github.com/rust-lang/rust>
- Small exercises : <https://github.com/rust-lang/rustlings>
- book: <https://doc.rust-lang.org/book/>
- the standards library :<https://doc.rust-lang.org/std/index.html>

## key concepts in rust

1. Ownership and Borrowing
   Rust’s ownership model ensures memory safety and prevents data races.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // This will cause a compile-time error since `s1` is moved to `s2`
    // println!("{}", s1);

    println!("{}", s2); // This works fine
}

```

In Rust, when you assign s1 to s2, ownership of the string moves to s2. s1 is no longer valid. This prevents issues like dangling pointers and double-free errors common in languages like C/C++.

2. Borrowing and References
   If you want to use a variable without transferring ownership, you can use borrowing:

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass a reference to s1
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

```

The &s1 is a reference to s1, allowing you to access its value without taking ownership.

3. Mutability

   In Rust, variables are immutable by default. You need to explicitly mark them as mutable:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

```

With mut, you can modify the variable after it’s initialized.

## Basics of rust

### Variables and Mutability

```rust
fn main() {
    let mut x = 5;
    x = 6; // This works because `x` is mutable
    println!("{}", x); // Prints: 6
}
```

### Data Types

Rust is statically typed, which means types are checked at compile-time. Unlike JavaScript, where types are dynamic and you don’t have to specify types (e.g., numbers, strings, arrays), Rust expects you to be explicit about types in some cases or relies on type inference.

```rust
fn main() {
    let a = 10;      // a is i32 by default (integer)
    let b = "Hello"; // b is &str (string slice)

    // If you need to explicitly declare a type:
    let c: f64 = 4.5; // c is a 64-bit floating point number
}

```

Rust will infer types when possible, but sometimes you'll need to be explicit about the type, especially for more complex data types or to prevent ambiguity.

### Ownership, Borrowing, and References

This is the biggest difference between JavaScript and Rust, and it can feel strange at first, but once you get used to it, it makes Rust incredibly powerful for memory management.

Ownership
In JavaScript, memory is managed with a garbage collector, and you don’t think much about memory. In Rust, you need to manage memory with ownership, but the compiler helps ensure you don’t make mistakes like double frees or dangling pointers.

Ownership means only one variable can own a piece of data at a time.

**_javascript example_**

```js
let a = [1, 2, 3];
let b = a; // `b` now points to the same array as `a`
console.log(a); // Still works, because `a` is pointing to the same array
```

**_rust example_**

```rust
fn main() {
    let a = String::from("Hello");
    let b = a;  // Ownership of the string is moved to `b`
    // println!("{}", a); // This would throw an error: `a` is no longer valid
    println!("{}", b); // This works
}

```

**_Borrowing and References_**
If you want to let multiple variables access the same data without transferring ownership, you can use references.

```rust
fn main() {
    let a = String::from("Hello");
    let len = calculate_length(&a); // Pass a reference to `a`
    println!("The length of '{}' is {}.", a, len); // `a` is still valid here
}

fn calculate_length(s: &String) -> usize {
    s.len() // We can use the reference `s` without taking ownership
}

```

In Rust, you pass a reference using the & symbol. This allows you to "borrow" data without taking ownership.

### Functions

Functions in Rust are similar to JavaScript in their syntax, but Rust is stricter about return types.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Return without semicolon, because this is the return value
}

fn main() {
    println!("{}", add(2, 3)); // Outputs: 5
}

```

In Rust, the return type is declared with -> i32. Also, notice that Rust functions do not need an explicit return statement if the last line is the return value.

### Control Flow (if, loops)

Rust and JavaScript share similar control flow syntax, but Rust has more strict type requirements.

```rust
fn main() {
    let num = 5;
    if num > 3 {
        println!("Greater");
    } else {
        println!("Smaller");
    }
}


```

However, in Rust, conditions in if statements must always evaluate to a boolean (bool), unlike JavaScript where conditions like if (5) or if ("hello") would be valid.

### Loops

Rust has a loop keyword for infinite loops (like JavaScript's while(true)), and it also supports while and for loops.

```rust
fn main() {
    for i in 0..5 {  // 0..5 is a range (0 to 4)
        println!("{}", i);
    }
}

```

## Key Rust Features for JavaScript Developers

1. Pattern Matching with match Rust’s match is more powerful than JavaScript’s switch. It allows you to match values, ranges, and even destructure data:

```rust
fn main() {
    let number = 7;
    match number {
        1 => println!("One"),
        2..=5 => println!("Between 2 and 5"),
        _ => println!("Something else"),
    }
}

```

2. Option and Result Types Rust avoids null or undefined with types like Option and Result. These force you to handle missing or erroneous values explicitly.

```rust
fn find_item(items: Vec<&str>, search: &str) -> Option<usize> {
    for (index, item) in items.iter().enumerate() {
        if item == &search {
            return Some(index);
        }
    }
    None
}

```

## environment workspace

- recommended editor : vscode
- cargo == npm

**_Working with Cargo_**

```bash
#This compiles your project and creates an executable in the target/debug directory.
cargo build

#This builds and runs your project in one step.
cargo run
cargo test # run tests

```

**_Add dependencies_**
In Rust, you add dependencies by editing the Cargo.toml file, similar to adding dependencies in package.json. For example, to add the serde library for JSON serialization, you'd add this to Cargo.toml:

```toml
[dependencies]
serde = "1.0"

```

then, run:

```bash
cargo build
```

Cargo will fetch and compile your dependencies for you.

## Tooling for Rust

Just like in JavaScript, you’ll use various tools to improve your development process.

- Formatter (rustfmt):
  Rust comes with a built-in code formatter, rustfmt, that automatically formats your code according to the official Rust style guide. To format your code, run:

```bash
#This is similar to using Prettier in JavaScript.
cargo fmt

```

- Linter (clippy):
  Rust has a tool called clippy, which provides additional linting checks beyond the compiler warnings. You can install and use clippy with:

```bash
cargo install clippy
cargo clippy

```

It provides helpful hints on how to make your code more idiomatic and efficient, similar to ESLint in JavaScript.

## Testing in Rust

Rust has a built-in testing framework. To write tests, create a tests module inside your src/main.rs or in separate test files, and use the #[test] attribute.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

```

You can run tests with:

```bash
cargo test

```
