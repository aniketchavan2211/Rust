## Rust 

![Rust Programming Logo ](https://github.com/aniketchavan2211/aniketchavan2211/blob/master/Images/Rust%20icon.png)

The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

- Compiled language like C, Java and C#.
- High-level simplicity with low-level performance.
- Great choice for building system [ OS / Game Engines Databases ]

> [!TIP]
> Rust Syntax looks like C++ 

### History

- Side project of Graydon Hoare in 2007.
- The rust fungus.
- Sponsored by Mozilla in 2009.

### Features

- Better memory management control.
- Rust uses Ownership & Borrowing.

### Quick Start

#### Installations

On Linux

```bash
sudo apt -y update && sudo apt -y install rustc rustup 
```

OR 

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

- To check rust complier ( rustc ):

```bash
rustc --version
```

#### Hello, World

- Using rustc

Create a rust script with extension `.rs`, example: `filename.rs`, 

**Filename.rs**:
```bash
fn main() {
    println!("Hello, World!");
}
```

Here, we create a main function which is entry point for program execution,
`println!()` function which display output on terminal or screen, 
we pass `Hello, World!` as string. and end the statement with `;` semicolon.

then compile with Rust Complier ( rustc ), 

```bash
rustc filename.rs -A warnings
```

Flag `-A warnings` will hide the warnings.

lastly run using `./outputfile`.

```bash
./filename
```

**Output**:
```
Hello, World!
```

- Using Cargo Package Manager

Create a cargo package with `cargo new packagename` or 
`cargo init`, `cargo init` will threat a directory or folder name as it package name, will not create another dir/folder 

You can set name of package using `cargo [init / new] --name "packagename"`

This will create some files and directories, 
`Cargo.toml` : this file contains some important stuffs package used, like used library or cartes, etc.

if suppose you want use `rand` crate then you have mention in `Cargo.toml` file with version then `cargo update` then use can use it.

**Cargo.toml**:
```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

you have write your code in `src/main.rs` file, run using `cargo run` or `cargo r` or `cargo run [main.rs]`.

Some cargo commands:

1. `cargo new `

2. `cargo init`

3. `cargo build`: `cargo b`

4. `cargo update`

5. `cargo check`

6. `cargo run`: `cargo r`

7. `cargo clean`

### Cargo - Package Manager

`Cargo` is Rust’s build system and package manager. Most `Rustaceans ( Rust Porgrammers )` use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)



### Variables and Datatypes

#### Ownership & Borrowing

Variables are immutable by default, means you can change value of variable.

```rust
let greetings = "Hello";
```

To make a variable mutable, add `mut` keyword before variable name.

```rust
let mut greetings = "Hello, Rust !!!";
```

now greetings variable value can change.

Variable is the `OWNER` of the value.

```rust
let z_owner = Flat::new();
```

when variable is not longer is needed Memory allocated is dropped.
```rust
{
    let z_owner = Flat::new();

    drop(z_onwer)
}
```

#### Numbers 


| Number literals | Example     |
| --------------- | ----------- | 
| Decimal	      |  98_222     |
| Hex	          | 0xff        |
| Octal	          | 0o77        |
| Binary	      | 0b1111_0000 |
| Byte (u8 only)  |	b'A'        |

In standard library (`std`) signed / unsigned integers and floating point numbers are defined.

unsigned integers: Numbers which can't be negative numbers.
mostly used in when program needs to stored age variable (age variable can't be negative [-2, -34, etc...]).

```rust
// Primitive DataTypes
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize. f32, f64}
```

`u8` can store numbers from `0` to `2 to power 8 - 1`, which equals `0` to `255`.

##### Signed Integers

1. `i8`: 8-bit of spaces
2. `i16`: 16-bit of spaces
3. `i32`: 32-bit of spaces
4. `i64`: 64-bit of spaces
5. `isize`: size will automatically assign by architecture.


**Snippets**:
```rust
{
    let _numi8: i8 = 23;        // 8-bit of spaces.
    let _num_i16: i16 = 34;     // 16-bit of spaces.
    let _num = 32;              // 32-bit of spaces [default].
    let _num = 64_i64;          // 64-bit of spaces.
    let _num = 64isize;         // size depends on architecture.

    // Ranges
    println!("Minimum ranges of i8 {}", i8::MIN);
    println!("Maximum ranges of i8 {}", i8::MAX);  
}
```

##### Unsigned Integers

Unsigned Integers don't accept numbers to be negative. 
Programmers used when a certain variable can't be negative, example 
when program needs age variable, which can be unsigned integer datatype because, 
age can't be negative.

1. `u8`: 8-bit of spaces
2. `u16`: 16-bit of spaces
3. `u32`: 32-bit of spaces
4. `u64`: 64-bit of spaces
5. `usize`: size will automatically assign by architecture.

**Snippets**:
```rust
{
    let _numu8: u8 = 23;        // 8-bit of spaces.
    let _num_u16: u16 = 34;     // 16-bit of spaces.
    let _num = 32;              // 32-bit of spaces [default].
    let _num = 64_u64;          // 64-bit of spaces.
    let _num = 64usize;         // size depends on architecture.

    // Ranges
    println!("Minimum ranges of u8 {}", u8::MIN);
    println!("Maximum ranges of u8 {}", u8::MAX);
}
```

##### Floating - Point Numbers

Floating - point numbers are numbers wiht decimal point numbers, or decimal numbers, 
example: 2.4, 45.67, etc...

1. `f32`: 32-bit size of spaces is assigned.
2. `f64`: 64-bit size of spaces is assigned.

**Snippets**:
```rust
{
    let num: f32 = 22.11 // 32-bit spaces.
    let sum: f64 = 34.56 // 64-bit spaces [default].

    // Ranges
    println!("Minimum ranges of f32 {}", f32::MIN);
    println!("Maximum ranges of f64 {}", f64::MAX);
}
```

#### strings

##### strings literal (&str)
strings are like any other programming langauges strings, 
alphanumeric value in double quotes `"strings"`.

- Static and Fixed Size:
 - String literals are static and have a fixed size known at compile time.
 - They are stored in the program's binary and are therefore more memory-efficient for constant, known values.

- Immutable:
 - String literals are immutable. Once defined, you cannot modify their contents.

- UTF-8 Encoding:
 - Like String, string literals in Rust are UTF-8 encoded.

**Snippets**:
```rust
{
    let name: &str = "Aniket"; // strings
}
```

To aviod warnings of snake case.
```rust
#![allow(non_snake_case)]

{
    let first_name: &str = "Aniket";
}
```

##### Strings (String::)

`Strings` in Rust, represented by the `String` type, are used to handle and manipulate textual data. They are dynamic, growable, and UTF-8 encoded, allowing for flexibility in working with text. Let's explore why String is used and how it differs from string literals `&str`, along with examples of how to use them.

- Dynamic and Growable:
 - String allows you to create strings with dynamic lengths. You can append, modify, and manipulate the content during runtime.

- Heap Allocated:
 - String data is stored on the heap, allowing it to grow or shrink in size as needed.

- UTF-8 Encoding:
 - Strings in Rust are UTF-8 encoded by default, providing support for a wide range of characters and internationalization.

- Ownership and Borrowing:
 - Ownership rules apply to String. Ownership can be transferred, and borrowing ensures that only one part of the code has mutable access at a time.

**Snippets**:
```rust
fn main() {
    // Creating a new empty String
    let mut new_string = String::new();

    // Appending to the String
    new_string.push_str("Hello, ");
    new_string.push_str("world!");

    // Printing the String
    println!("{}", new_string); // prints "Hello, world!"
}
```

> [!NOTE]
> Understanding when to use String and when to use string literals `&str` depends on your specific use case. If you need a mutable dynamic string with variable length, use String. If you have a fixed-size, immutable string known at compile time, use a string literal `&str`.

#### Booleans

```rust
{
    let is_valid: bool = true; // true / false
}
```

#### Characters

```rust
{
    let character: char = 'a';
}
```

#### Constants

Constants are variable that can not be change, declare in global scope, not in Block scope,
All letters should be in upper case. if more than one letter use snake case.

**Snippet**:
```rust
// Global scope
const THE_CONSTANT: u32 = 2;

fn main() {
    // Block Scope
    println!("The Constant contains value of: {}", THE_CONSTANT);
}
```

> [!NOTE]
> `let` keyword used only used in block scope.
> you can't used `mut` to keyword to make constant mutable or re-assigned value.

### Comments

#### Single - Line Comments

**Snippets**:
```rust
// Single - Line Comments
```

#### Multi - Line Comments

**Snippets**:
```rust
/* 
 Multi
 Line
 Comments
*/
```

### Input / Output

#### Output

```rust
{
    let var1: &str = "abc";
    println!("This is var1 variable: {}", var1);
}
```

**Define multiple variables in one line**

**Snippets**:
```rust
{
    let(first_name, last_name) = ("Aniket", "Chavan");
    println!("My self {}, {}", first_name, last_name);
}
```

**Math Calculation**

**Snippets**:
```rust
{
    println!("20 + 4 = {}", 20 + 4);
    println!("20 - 4 = {}", 20 - 4);
    println!("20 x 4 = {}", 20 * 4);
    println!("20 / 4 = {}", 20 / 4);
    println!("20 % 4 = {}", 20 % 4);
}
```

### Conditions statements

Conditional statements used when programs needs decision making capabilities.
`if`, `else if` and `else` keywords used. 

**Snippets**:
```rust
fn main() {
    let age: i64 = 12;

    if age < 18 {
        println!("You are under age !!!");
    } else if age <= 20 {
        println!("You are ready for voting !!!");
    } else {
        println!("Your age limits excceds, Your age must be: {}", x);
    }
}
```

**Output**:
```
You are under age !!!
```

### Loops 

```rust

{
    let mut n = 0;

    loop {
        n += 1; // incrementing by 1.

        // continue, skip 13
        if n == 13 {
            continue;
        }

        // break, loop until meets 15
        if n > 15 {
            break;
        }
        println!("The value is {}", n);
    }
}
```

**Output**:
```
The value is 1
The value is 2
The value is 3
The value is 4
The value is 5
The value is 6
The value is 7
The value is 8
The value is 9
The value is 10
The value is 11
The value is 12
The value is 14
The value is 15
```

#### For Loops


**Snippet**:
```rust
{
    for i in 1..6 {
        println!("The Line is: {}", i)
    }
}
```

This Loops from 1 to 5. index starts from 0, here range is set to `1..6`, 
`start..end`.

Also you make defined range in variable.
**Snippets**:
```rust
{
    let x = 1..6;

    for i in x {
        println!("The Number is: {}", i);
    }
}
```

Loops through elements.

**Snippets**:
```rust
{
    let fruits = vec!["orange", "apple", "mango"];

    for (index, i) in fruits.iter().enumerate() {
        println!("Fruits {} is {}", index, i);
    }

}
```

**Output**:
```
Fruits 0 is orange
Fruits 1 is apple
Fruits 2 is mango
```

#### While Loops 

**Snippets**:
```rust
{
    let mut num: i32 = 1;

    while num < 6 {
        println!("The Number is: {}", num);
        num += 1;
    }
}
```

**Output**:
```
The Number is: 1
The Number is: 2
The Number is: 3
The Number is: 4
The Number is: 5
```

**Snippets**:
```rust
{
    let mut num: i32 = 1;

    while num < 10 {
        if num % 2 == 0 {
            println!("The Number {} is Even", num);
        } else {
            println!("The Number {} is Odd", num);
        }
        num += 1;
    }
}
```

**Output**:
```
The Number 1 is Odd
The Number 2 is Even
The Number 3 is Odd
The Number 4 is Even
The Number 5 is Odd
The Number 6 is Even
The Number 7 is Odd
The Number 8 is Even
The Number 9 is Odd
```

### Tuples

Tuples is DataTypes in Rust, Which contains mixed datatypes like `strings`, `1`, `2.4`, `true` or even nested tuples.
Tuples defined using Rounded Brackets `()`.

**Snippet**:
```rust
{
    let tuple = ("strings", 1, 2.4, true, (1, 2, 3));

    println!("This is {}, Numbers are: {} and {}, Boolean value is {}, Nested Tuple is: ({}, {}, {})", 
        tuple.0, tuple.1, tuple.2, tuple.3, (tuple.4).0, (tuple.4).1, (tuple.4).2);
}
```

### Enum 

An `enum` (enumeration) is a custom data type that allows you to define a type by enumerating its possible values. Each value is called a `variant`, and you can associate data with each `variant`.

**Snippets**:
```rust
enum Color {
    Red,
    Green,
    Blue,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

```

- `Color` is an `enum` with variants representing different colors.
- `Shape` is an `enum` with variants representing different geometric shapes, each with associated data.


**Example**:
```rust
// Define an enum called TrafficLight with three variants
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Main function to demonstrate the usage of the enum
fn main() {
    // Create instances of the TrafficLight enum
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    // Use match to pattern match and act accordingly
    match red_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Prepare to stop or go!"),
        TrafficLight::Green => println!("Go!"),
    }

    // Pattern match another instance
    match green_light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Prepare to stop or go!"),
        TrafficLight::Green => println!("Go!"),
    }
}
```

**Output**:
```
Stop!
Go!
```

- We define an enum named TrafficLight with three variants: Red, Yellow, and Green.
- We create instances of the TrafficLight enum using those variants.
- We use the match expression to pattern match on the enum instances and execute code based on the matched variant.

In the first match, we match against red_light, and it prints "Stop!" because the variant is TrafficLight::Red. In the second match, we match against green_light, and it prints "Go!" because the variant is TrafficLight::Green.

Enums are particularly useful for situations where you have a fixed set of possible values and want to express that in a clear and type-safe manner. The match expression is a powerful tool for handling different cases or variants and allows you to write concise and expressive code.

#### Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to destructure and inspect the values of enums or other data types. The match keyword is used for pattern matching.

**Snippets**:
```rust
fn main() {
    let color = Color::Blue;

    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
    }

    let shape = Shape::Circle(3.0);

    match shape {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle with width {} and height {}", width, height),
        Shape::Triangle(side1, side2, side3) => println!("Triangle with sides {}, {}, and {}", side1, side2, side3),
    }
}
```

- We use match to compare the value of color and execute the corresponding block of code.
- Similarly, we use match to destructure the shape and print information based on its variant and associated data.

### Functions

**Snippets**:
```rust
fn add(x: i32, y: i32) {
    // Adding two Numbers
    let z: i32 = x + y; 
    println!("The Total of {} + {} is: {}", x, y, z);
}


fn main() {

    // Calling a function
    add(23, 43);

}
```

**Output**:
```
The Total of 23 + 43 is: 66
```

**Snippets**:
```rust
fn main() {

    // Using predefined methods
    let strings: &str = "strings";

    println!("The Total Length of Variable strings is: {}", strings.len());

}
```

**Output**:
```
The Total Length of Variable strings is: 7
```