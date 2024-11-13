# all about rust notes

- ## Understanding Ownership

  `Ownership` is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make `memory safety guarantees` without needing a `garbage collector`, so it’s important to understand how ownership works. we’ll talk about ownership as well as several related features: `borrowing`, `slices`, and how Rust lays data out in memory.

  - ### What Is Ownership?

    Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

    Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you’ll find it to naturally develop code that is safe and efficient. Keep at it!

    When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique. In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.

  - ### Ownership Rules

    First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.

  - ### Variable Scope

    A scope is the range within a program for which an item is valid.

    ```rust
    let s = "hello";
    {// s is not valid here, it’s not yet declared
      let s = "hello";   // s is valid from this point forward

      // do stuff with s
    }// this scope is now over, and s is no longer valid

    ```

    At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we’ll build on top of this understanding by introducing the String type.

  - ### the String type

    - #### String literal

      `let greeting = "Hello, world!";` : is a hardcoded, immutable sequence of text within double quotes, stored directly in the binary of the program, making it a slice of a string (&str type) with a fixed size at compile time.

      - Key Characteristics of String Literals:

        1. Immutable: String literals cannot be changed or modified after they are defined.
        2. Fixed Size: The length of a string literal is known at compile time.
        3. Stored in Read-Only Memory: They are stored in a special read-only section of memory, making them efficient but unmodifiable.
        4. Type: They have the type `&str`, which is a reference to a string slice.

        String literals are best suited for situations where the text content is known at compile time and will not need to be changed. For more dynamic or mutable text, Rust provides the `String` type, which allocates memory on the heap.

    - #### String type

      `let s = String::from("hello");`

      The double colon `::` operator allows us to namespace this particular from function under the `String` type rather than using some sort of name like string_from.

      ```rust
          let mut s = String::from("hello");
      s.push_str(", world!"); // push_str() appends a literal to a String
      println!("{s}"); // This will print `hello, world!`
      ```

      the String type is a growable, mutable, and heap-allocated string that can store text of unknown size at compile time. Unlike a string literal (&str), which is fixed and immutable, a String can be modified and resized as needed.
      Key Characteristics of the String Type:

      1. Heap-Allocated: String stores its data on the heap, allowing it to dynamically expand or shrink as the program runs.
      1. Mutable: Unlike string literals, String values can be changed after they’re created.
      1. Owned Type: String is an owning type, meaning it takes ownership of the text it contains. When a String goes out of scope, Rust automatically cleans up the memory it used.
      1. UTF-8 Encoded: String in Rust supports UTF-8 encoding, so it can store and handle a wide range of characters beyond ASCII, including emojis and international characters.

      You typically create a String using methods like `String::from` or `.to_string()` from a string literal

      The String type is useful when:

      - You need to store user input or dynamically generated text.
      - You need a mutable string.
      - You want to work with text whose size or content isn’t fixed at compile time.

    - #### Variables and Data Interacting with Move

      Multiple variables can interact with the same data in different ways in Rust

      ```rs
        let x = 5;
        let y = x;
      ```

      We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.

      Now let’s look at the `String` version:

      ```rs
          let s1 = String::from("hello");
        let s2 = s1;
      ```

      This looks very similar, so we might assume that the way it works would be the same: that is, the second line would make a copy of the value in s1 and bind it to s2. But this isn’t quite what happens.
      A String is made up of three parts, shown on the left: a `pointer` to the memory that holds the contents of the string, a `length`, and a `capacity`. This group of data is stored on the `stack`. On the right is the memory on the `heap` that holds the contents.
      The `length` is how much memory, in bytes, the contents of the String are currently using. The `capacity` is the total amount of memory, in bytes, that the String has received from the allocator. The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.

      > When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to.
      > meaning both s1 and s2 point to the same data in the heap

      ##### Double free Error

      Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
      To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created; it won’t work:

      ```rust
        let s1 = String::from("hello");
        let s2 = s1;

        println!("{s1}, world!");
      ```

      If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2. So, what actually happens
      That solves our problem! With only s2 valid, when it goes out of scope it alone will free the memory, and we’re done.

      In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

    - #### Variables and Data Interacting with Clone

      If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`, but because methods are a common feature in many programming languages, you’ve probably seen them before.

      ```rust
      let s1 = String::from("hello");
      let s2 = s1.clone();
      println!("s1 = {s1}, s2 = {s2}");
      ```

      This works just fine and explicitly produces the behavior shown before , where the heap data does get copied.

      When you see a call to `clone`, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.

    - #### Stack-Only Data: Copy

      There’s another wrinkle we haven’t talked about yet. This code using integers—part of which was shown in Listing 4-2—works and is valid:

      ```rs
        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");
      ```

      But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into `y`.

      - Stack-only data types like integers do not need `clone`; they are trivially copied by assignment.
      - Example: `let x = 5; let y = x;` Both x and y are valid without clone.
      - Types with the `Copy` trait are not moved but trivially copied, remaining valid after assignment.
      - Only simple types that do not require special cleanup (i.e., no `Drop` trait) can implement `Copy`.
      - Scalar types such as integers (e.g., `u32`), booleans (bool), floats (f64), and characters (char) implement Copy.
      - Tuples can implement `Copy` if all elements within them implement Copy (e.g., (i32, i32) can, but (i32, String) cannot).

  - ### Ownership and Functions

    The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

    ````rs
      fn main() {
          let s = String::from("hello");  // s comes into scope

          takes_ownership(s);             // s's value moves into the function...
                                          // ... and so is no longer valid here

          let x = 5;                      // x comes into scope

          makes_copy(x);                  // x would move into the function,
                                          // but i32 is Copy, so it's okay to still
                                          // use x afterward

      } // Here, x goes out of scope, then s. But because s's value was moved, nothing
        // special happens.

      fn takes_ownership(some_string: String) { // some_string comes into scope
          println!("{some_string}");
      } // Here, some_string goes out of scope and `drop` is called. The backing
        // memory is freed.

      fn makes_copy(some_integer: i32) { // some_integer comes into scope
          println!("{some_integer}");
      } // Here, some_integer goes out of scope. Nothing special happens.
      ```

      If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error. These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

    - ### Return Values and Scope
    Returning values can also transfer ownership.
    ````

  ```rust
    fn main() {
      let s1 = gives_ownership();         // gives_ownership moves its return
                                          // value into s1

      let s2 = String::from("hello");     // s2 comes into scope

      let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                          // takes_and_gives_back, which also
                                          // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String {             // gives_ownership will move its
                                              // return value into the function
                                              // that calls it

      let some_string = String::from("yours"); // some_string comes into scope

      some_string                              // some_string is returned and
                                              // moves out to the calling
                                              // function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope

      a_string  // a_string is returned and moves out to the calling function
    }
  ```
