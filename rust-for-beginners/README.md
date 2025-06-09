## Rust For Beginners

**WHAT IS RUST?**
Rust is a System Programming Language like C and C++, Used for building Operating System and Game Engines, etc. And also need a complier, which will convert rust source code into machine code / binary code or low-level code which Computer understand and performs action accordingly.


Rust Offers you

- Speed
- Safety
- Concurrency
- Portability

**Memory Management**
- High-level languages like python provides garbages collection.
- Lower-level langauges like C++ provide functions for memory allocations.
- Rust uses a concept called ownership and borrrowing.

`cargo` is a package manager

### Getting Started

On browser, visit Official site of [rust-lang.org](https://www.rust-lang.org/) and download and install according to your OS. 
You can built CLI tools, Networking tools, Webassembly code, Embedded code using Rust Progarmming Language.

> [!NOTE]
> We are using Linux OS for this Tutorials.

You can also Learn Rust from Official [Rust Books](https://doc.rust-lang.org/book/), Videos from Youtube, etc.

If you install rust using `rustup` then check in Terminal is perfectly install by

```bash
rustup --version
```

OR 

If you use other method to install then check Rust complier and Cargo Package Manager is install or not.

```bash
rustc --version
cargo --version
```

First create a directory, then inside the directory create a rust main source file, named it `hello.rs` remember rust extension always end with `.rs`

```bash
mkdir hello
cd hello
touch hello.rs
```

If you are using `VS Code` then install `rust-analyser` extensions. 

```rs
fn main() {
    println!("Hell, World!");
}
```

here we create a function using `fn` keyword, following with function, which is `main`, rust main function is main entry point, every code starts from here,
if main() is not written in script it's likely a `lib.rs` a libaray file. in scope of main() function within curly brackets `{}` we write code, 
here we will be printing a line says `Hello, World!`, for this we need a macro `println!()`, mostly macro ends with `!`. 

now that we write the code let's compiled and run.

```bash
rustc hello.rs
./hello
```

using rustc rust compiler and filename which is `hello.rs`, we compiled the file, and output of file store in filename `hello` which is binary file, which we will be executing, for this on linux, we will using `./` and filename.


```bash
Hello, World!
```

With this we sucessfully compiled and run, our's first hello world rust program.

we can use `cargo` for creating and initiazing projects.

```bash
cargo new project-name
```

This will a project, contains projects files, `src/main.rs`, `Cargo.toml`& `.gitignore`

now let's run project files, sightly changed the `src/main.rs` code.
```rs
fn main() {
    println!("Hello, Rust !!!");
}
```
save the code, get inside the project directory, and run `cargo run` on Terminal.
This will execute the program buiding and compiling and executing or running the program.

**Output:**
```
Hello, Rust !!!
```

In case, you already created a directory with project name, and want initalized it, used `cargo init`.
This will created sources files, and others files for you.

### Primitive Data Types

Rust is a Statically Typed Programming Language.

```rs

fn main() {
    // Primitive types    
    // int, float, char, bool

    // integer [Integer Type]
    // Rust has signed (+ or -) and unsigned ( only + ) integers
    // i8, i16, i32, i64, i128: Singed integers
    // u8, u16, u32, u64, u128: Unsigned integers

    let x :i32 = -42;
    let y: u64 = 100;
    println!("Signed Integers: {}, Unsigned Integers: {}", x, y);

    // difference between i32 (32 bits) and i64 (64 bits)
    // range of i32 is -2,147,483,648 to 2,147,483,647
    // range of i64 is -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("MAX of value of i32: {}, MAX of value of i64: {}", e, i);

    // float [Floating Point Type]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // bool [Boolean Type]
    // true or false
    let is_snowing: bool = true;
    print!("Is it Snowing?: {}\n", is_snowing);

    // char [Character Type]
    let letter: char = 'a';
    println!("First letter of alphabets: {}", letter);

}
```


### Compound Data Types

```rs
// Compound Data Types
// Arrays, Tuples, Slices, Strings (Slice String)

fn main() {
    // Arrays
    // Fixed size, same type
    let numbers: [i32; 5]  = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    // let mix = [1, 2, "apple", true]; // This will not work because the array must be of the same type
    // println!("Mix Array: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("All Fruits: {:?}", fruits);
    println!("1st Fruits: {}", fruits[0]);
    println!("2nd Fruits: {}", fruits[1]);
    println!("3rd Fruits: {}", fruits[2]);

    // Tuples  
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human: {:?}", human);
    println!("Name: {}", human.0);
    println!("Age: {}", human.1);
    println!("Is Human: {}", human.2);
    
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Numbers of Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Tiger", "Bear"];
    println!("Animals: {:?}", animal_slices);

    let books_slices: &[&String] = &[&"Computer Science".to_string(), &"Harry Potter".to_string(), &"The Hobbit".to_string()];
    println!("Books: {:?}", books_slices);

    // Strings vs String Slice (&str)
    let mut stone_cold: String = String::from("Hell, "); // Stored on Heap Memory
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!"); // Mutable
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice) On Stack Memory Immutable
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("String Slice: {}", slice);

}
```

### Functions

```rs
// Functions
// Entry point
// Any Functions / Variables should be written in snake case
// snake case: hello_world
// kebab case: hello-world not allowed

fn main() {
    hello_world(); // Call the function   
    tell_height(182); // Call the function with input
    human_id("Alex", 21, 182.3);
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is {}", x);

    let y = add(4, 6);
    println!("Value of y is {}", y);
    println!("Value from function 'add' is : {}.", add(4, 6));

    // Calling a BMI function
    let weight = 70.0; // in kg
    let height = 1.82; // in m
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

// Hoisting: can call the function anywhere in your code
fn hello_world() {
    println!("Hello, Rust!");
}
// you can insert input values
fn tell_height(height: u32) {
    println!("My height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

// Expression and Statements
// Expression: Anything that returns a value.
// Statements: Anything that does not return a value.
// Example of expression: 5, true or false, add(4, 5), if condition {value} else {value}, ({code})

// Functions also return values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Statements: do not return a value
// almost all statements in rust end with a semicolon;
// let y = let x = 10; 
// 1. Variables declaration: let x = 5;
// 2. Function declaration: fn foo() {..}
// 3. Control flow: if condition {..} else {..}, while condition {..}, etc


// Example:
// BMI -= height(kg) / height(m) * 2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```

### Ownerships

```rs

// Ownership
// C & C++ -> Memory Management Control Issue
// Garbage Collection solved this issue, but created a new issue -> slow performance
// [stopping / resuming the program]

// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.

// What is Ownership?
// Every value has a single owner [every variable has one value, and is it's sole owner].

// Ownership Rules:
// 1. Each value in Rust has a variable that’s called its “owner”.
// 2. A value can only have one owner at a time.
// 3. When the owner of a value goes out of scope, the value will be dropped.

// Example: Each value in Rust has a variable that’s called its “owner”.
/*
fn main() {
   let s1 = String::from("RUST");
   let len = calculate_length(&s1);
   println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
   s.len()
}
*/

// Example 2: A value can only have one owner at a time.
/*
fn main() {
   let s1 = String::from("RUST");
   let s2 = s1; // transfer ownership from s1 to s2.

   // println!("{}", s1);
   println!("{}", s2);
}
*/

// Example 3: When the owner of a value goes out of scope, the value will be dropped.
/*
fn main() {
   let s1 = String::from("RUST");
   let len = calculate_length(&s1);
   println!("Length of '{}' is '{}'", s1, len);

   // s1 goes out of scope an dits value will be dropped.
}

fn printLost(s: &String) {
   println!("{}", &s1);
}

fn calculate_length(s: &String) -> usize {
   s.len()
}
*/
```

### Borrowing & References

```rs
// Borrowing & References
// Safety and Performance

// References
// References: Enable you to borrow values without taking ownership.
// Immutable Reference.
// Mutable References.
// Create Reference by adding `&`. 

// Immutable Reference
/*
fn main() {
   let mut _x = 5;
   let _r = &mut _x;

   *_r += 1;
   *_r -= 3;
   

   println!("Value of _x: {}", _x);
   // println!("Value of _r: {}", _r);
}
*/

// struct: A data structure that allows you to group mutiple fields together under one name.

fn main() {
   let mut account = BankAccount{
      owner: "Alice".to_string(),
      balance: 150.55,
   };

   // Immutable borrrow to check the balance
   account.check_balance();

   // Mutable borrow to withdraw money
   account.withdraw(45.50);

   // Imutable borrow to check the balance
   account.check_balance();
}

struct BankAccount {
   owner: String,
   balance: f64,
}

impl BankAccount{
   fn withdraw(&mut self, amount: f64) {
      println!("Withdrawing {} from account owned by {}", amount, self.owner);
      self.balance -= amount;
   }

   fn check_balance(&self) {
      println!("Account owned by {} has a balance of {}", self.owner, self.balance);
   }
}
```

### Variables & Mutability

```rs
// Variables & Mutability

fn main() {
   let mut a = 5;
   println!("The value of a: {}", a);

   a = 10;
   println!("The value of a: {}", a);

}
```


### Constants

```rs
// Constants

fn main() {
   // const mut y = 10;
   // constants can not be mutable, by default immutable and you can't change it to mutable.
   // you need provide the type for constant: i32,..
   // You always defined const using captial letters. A,B,C,Y,X,Z,..

   const Y: i32 = 10;
   println!("The Value of Constant Y is: {}", Y);
   println!("The Value of Constant PI is: {}", PI);
   println!("THE Value of three hours in second is :{}", THREE_HOURS_IN_SECOND)
}


// Global Scope
// You can declare a constant with a type annotion.
const PI: f64 = 3.141;
const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
```

### Shadowing 

```rs
// Shadowing
// Shadowing is not the same as marking a variable as mutable.
// Very usefull when changing types.

fn main() {
   let x = 5;
   println!("The first variable of x: {x}");

   let x = x + 1; // shadowing the first variable x, overshadowing to the second varible x. value of of second x value is 6.
   println!("The second variable of x is: {x}");

   {
      let x = x * 2; // third variable x is 12.
      println!("The value of x in  inner scope is: {x}")
   }

      println!("The second variable of x is: {x} in main function");
}
```

### Comments

```rs
// Comments


fn main() {
   // One-Line Comment

   println!("Hello, "); // Here is another comments
   // println!("Rust!!!");
   
   /* Mutiple-Line / Block Comments */
}
```

### Control Flows (if .. else .. & Loops)

**Conditional Statements**
```rs
// Control Flows
// ff .. else if .. else ..


fn main() {
   // let age: u16 = 18;
   // if age >= 18 {
   //    println!("You can drive a car!");
   // } else {
   //    println!("You can't drive a car!");
   // }

   // Mutiple conditions with else if 
   // let number = 6;
   // if number % 4 == 0 {
   //    println!("Number is divisble by 4");
   // } else if number % 3 == 0 {
   //    println!("Number is divisible by 3");
   // } else if number % 2 == 0 {
   //    println!("Number is divisible by 2");
   // } else {
   //    println!("The Number is not divisible by 2, 3, or 4");
   // }

   // Using if in a let statement
   let condition = true;
   let number = if condition {5} else {6};
   println!("Number: {number}");
}
```

**Loops**
```rs
// Control Flows
// Loops (while loops & for loops)

fn main() {
   // loop{
   //    println!("Hello, World");
   //    // break;
   // }

   // let mut counter = 0;

   // let result = loop {
   //    counter += 1;
      
   //    if counter == 10 {
   //       break counter * 2;
   //    };
      
   // };
   // println!("The result is {result}");

   // loops labels: 'counting_up
   //  let mut count = 0;
   //  'counting_up: loop {
   //      println!("count = {count}");
   //      let mut remaining = 10;

   //      loop {
   //          println!("remaining = {remaining}");
   //          if remaining == 9 {
   //              break;
   //          }
   //          if count == 2 {
   //              break 'counting_up;
   //          }
   //          remaining -= 1;
   //      }

   //      count += 1;
   //  }
   //  println!("End count = {count}");
   
   // While loops
   // let mut number = 3;
   // while number != 0 {
   //    println!("{number}!");
   //    number -= 1;
   //    // break;
   // }
   // println!("Hey !!!");

   // For loops
   // let a = [1, 2, 3, 4, 5];
   // let b = ["a", "b", "c", "d", "e"];
   // for element in a {
   //   println!("The value is: {element}");
   // }
   // for element in b {
   //    println!("The value is: {element}");
   // }
}
```


### Stucts

Structs are similar to tuples, in the both hold mutiple related values. Like tuples, the pieces of a struct can be diferent types. Unlike
with tuples, in a struct you'll name each piece of data so it's clear what the value mean. Adding these names means that structs are more 
flexible than tuples: you don't have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword `struct` and name the entire struct. A struct's name should describe the significance of the pieces of data
being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. For example, listing 5-1 shows 
a struct that stores information about a user account. 

```rs
// struct

fn main() {
   // tuple 
   let rect = (200, 500);

   // struct
   // structs are used to name and package related values similar to tuples
   struct Books{
    title: String,
    author: String,
    pages: u32,
    available: bool,
   } 

   struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
   }

   let mut user1 = User{
    active: true,
    username: String::from("someusername"),
    email: String::from("someusername@gmail.com"),
    sign_in_count: 1,
   };

   // user1.email = String::from("anotheremail@gmail.com"); // updating email
   println!("User email is: {} ", user1.email);

   // return a struct from a function
   fn build_user(email: String, username: String) -> User{
      User{
         active: true,
         email,
         username,
         sign_in_count: 1,
      }
   }

   // create instances from other instances
   let user2 = User{
      email: String::from("another@m.com"),
      ..user1
   };

   // Tuple structs
   struct Color(i32, i32, i32);
   struct Point(i32, i32, i32);

   let black = Color(0, 0, 0);
   let white = Color(255, 255, 255);

   // Unit-like struct
   struct AlwaysEqual;
   let subject = AlwaysEqual;

}
```


### Enums

```rs
use std::{net::IpAddr, u8};

fn main() {
   // Enums
   // Enums is a versatile tool used to represent a type that can take on one of serveral possible variants

   enum IpAddrKind {
      V4(),
      V6(),
   }

   let _four = IpAddrKind::V4;
   let _six = IpAddrKind::V6;

   fn route(ip_kind: IpAddrKind) {}
   
   // route(IpAddrKind::V4);
   // route(IpAddrKind::V6);

   enum IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
   }

   // Using Enums
   // let home = IpAddr::V4(String::from("127.0.0.1"));
   // let loopback = IpAddr::V4(String::from("::1"));
   
   // Enhanced Enums
   let home = IpAddr::V4(127,0,0,1);
   let loopback = IpAddr::V6(String::from("::1"));
   

   // // using structs
   // struct IpAddr{
   //    kind: IpAddrKind,
   //    adddress: String,
   // }

   // let home = IpAddr{
   //    kind: IpAddrKind::V4,
   //    adddress: String::from("127.0.0.1"),

   // };

   // let loopback = IpAddr{
   //    kind: IpAddrKind::V6,
   //    adddress: String::from("::1"),
      
   // };

}
```

### Error Handling 

```rs

// Error Handling
// 1. Result<T, E> 
// 2. Option<T>

// Approach 1 
// enum Option<T> { // Define the generic Option Type
//    Some(T), // Represents a value
//    None,    // Represents no value
// }

// fn divideOption(numerator: f64, denominator: f64) ->Option<f64> {
//    if denominator == 0.0 {
//       None
//    } else {
//        Some(numerator / denominator)
//    }
// }

   
// Approach 2
// enum Result<T, E> {  // Define the generic Result type
//    Ok(T),   // Represents a value
//    Err(E),  // Respresents an error
// }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
   if denominator == 0.0 {
      Err("Cannot divide by zero".to_string())
   } else {
      Ok(numerator / denominator)
   }
}

fn main() {

   // let result = divide(10.0, 2.0);
   // match result{
   //    Some(x) => println!("Result: {}", x),
   //    None => println!("can't divide by zero!"),
   // };

   match divideResult(100.23,73.98) {
      Ok(result) => println!("Result: {}", result),
      Err(err) => println!("Error: {}", err),
   }

}
```

### Collections Types

#### Vectors
```rs
// Collections Types
// Vectors
fn main() {
   // let _v: Vec<i32> = Vec::new();
   // Macro to create a vector of numbers 
   // let mut _v: Vec<i32> =  Vec::new();
   // let mut _v: Vec<i32> = vec![1, 2, 3];
   // _v.push(5);
   // _v.push(6);
   // _v.push(7);
   // _v.push(8);
   // _v.push(9);
   // println!("{:?}", _v);

   let _v = vec![1, 2, 3, 4, 5];
   let third: &i32 = &_v[2];  // Direct indexing
   // println!("The third elements: {third}");

   let third = _v.get(4);
   match third {
      Some(third) => println!("The third elements for a GET method is {third}"),
      None => {
        println!("There is no third element.")
    },
   }

}
```

#### UTF8

```rs
// Collections Types
// UTF8
fn main() {
   let s = "whatever".to_string(); // 1
   let s = String::from("wahtever");  // 2 
   let mut s = String::from("foo");  // Mutating  the varaible [push to it]
   s.push_str("bar");
   s.push('!');

   // println!("the value if s = {}", s);

   // If you want to combine strings, use the + operator
   let s1 = String::from("Hello, ");
   let s2 = String::from("World!");
   let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
   println!("The value of s3:  {}", s3);

   // Formatting Strings
   let full_message = format!("{salam} {salut}"); // salam and salut is not defined
   println!("{full_message}");

}
```

#### HashMaps

```rust
// Hash Maps 
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}: {score}");

    // If you want to print all key-value pairs:
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

```