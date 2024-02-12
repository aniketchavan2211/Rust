## Rust 

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

### Quick Installation

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


### Cargo - Package Manager

`Cargo` is Rust’s build system and package manager. Most `Rustaceans ( Rust Porgrammers )` use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

### Hello, World

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

### Variables and Datatypes

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

#### Strings

Strings are like any other programming langauges strings, 
alphanumeric value in double quotes `"Strings"`.

**Snippets**:
```rust
{
    let name: &str = "Aniket"; // Strings
}
```

To aviod warnings of snake case.
```rust
#![allow(non_snake_case)]

{
    let first_name: &str = "Aniket";
}
```

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