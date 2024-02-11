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

##### Integers

1. `i8`
2. `i16`
3. `i32`
4. `i64`
5. `isize`

##### Unsigned Integers

1. `u8`
2. `u16`
3. `u32`
4. `u64`
5. `usize`

##### Floating - Point Numbers

1. `f32`
2. `f64`