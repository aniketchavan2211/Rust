## Rust Progarmming Language Tutorial


### Installation

- On Kali Linux, 

```bash
# This will install all rust packages.
sudo apt install rust-all
```

### Gettings Started

- **Type Safety**

In rust, their are safety rules for type, means that you can not change the type.

```rs
// This code will not compile 
// Throws an Compilation Error
// Rust throw an error at compile time, not in runtime (execution time)

fn main() {
    let x = 1;
    x = "String";
    println!(x);
}
```

- **System Programming Language**

Rust is a system programming langauge or low-level language, 
you write kernel modules in rust, control over hardware, can program OS/Kernel.
can interact with RAM, Computer resuurces other than RAM.

- **Fast**

Rust is Faster than languages, C / C++, Java, Python, Javascript etc.
Rust has a separate compilation step that spilts out an optimised binary and does a lot of `static analysis` at complile time.

```bash
[Rust Code] --> Build / Optimise --> [Binary] --> Run / Execute
```

- **Concurrency**

Rust has built-in support for concurrent programming allowing muliple threads to performs tasks simultaneously without risking data races.

- **Memory Safe**

Rust has a concept of owners, borrowing and lifetimes that make it extremely memory safe.

### Initializing project

- cargo new
  
- rustc 

- cargo init

```bash
mkdir project-name
cd project-name
cargo init
```

`cargo init` will initialzed the project by creating `Cargo.toml` and `src/main.rs`. `Cargo.toml` will contains packages or cartes, you have mention packages or crates or modules here in this file. In `src` directory, you will find `main.rs`. `main.rs` will main entry is program , here you start writing your program.


- **lib**
  
```bash
cargo init --lib
```

`cargo build` is build the source code `main.rs` into binary, and if you on linux run by `./target/debug/rust-proj` 

```bash
# This is Optimized version
cargo build --release
```

```bash
# this will build and run the code
cargo run
```

### Variables 

You can defined variables with `let` keyword, and assign type of the variable.

```rs
fn main() {
    let x = 1; // i32 is default 
    let x: i32 = 1;
    let x: i8 = 1; lowest bits
    let x: i128 = 128; highest bits 
    println!("{}", x);
}
```

`i8`, `i16`, `i32`, `i64` & `i128` : are signed integers positive and negative numbers.

`u8`, `u16`, `u32`, `u64` & `u128` : are unsigned integers , positive numbers only.

`i8`: range: `-128 to 127` : (-2 ** 7) to (2 ** {7-1}):  numbers can be 0 and 1, so 2 to the power of 7's and first bit is for positive or negative sign to store.

`f32`, `f64`: floating-point numbers , decimals numbers `1.2` etc.

```bash
fn main() {
    let x: i32 = -5;
    let y: u32 = 1000;
    let z: f32 = 1000.001;

    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

**OverFlow Runtime Logic**
```rs
fn main() {
    let mut x: i8 = 10;

    for i: i32 in 0..1000 {
        x = x + 100; // > 127 execceds the i8 bits limits; overflow
    } 

    println!("x = {}" ,x)
}
```

### Booleans

Booleans has two states: true and false used in decision making.


```rs
fn main() {
    let is_male = true;   
    let is_above_18 = true;

    if is_male {
        println!("You are male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You aree a legal male !!!");
    }
}
```

### Strings

```rs
{
    let greeting: String = String::from("Hello World!!!");
    println!("{}", greeting);

    let char1: Option<char> = greeting.chars().nth(1000);

    // match char1{
    //     Some(c: char) => println!("{}", c);
    //     None => println!("No character at index 1000"),
    // }

    // println!("char1: {}", char1.unwrap());

    // let greeting1: &str = "Hello World!!!";
    // println!("{}", greeting1);
}
```

### Conditions & loops

**Conditions**

```rs
{
    let is_even = true;

    if is_even {
        println!("The number is even");
    } else if !is_even {
        println!("The number is odd");
    }
}
```

**Loops**

```rs
{
    for i in 0..11 {
        println!("{}", i);
    }
}
```

iterators: arrays, maps, strings, vectors.
```rs
fn main() {

    let sentence : String = String::from("My Name is: Aniket");
    let first_word = get_first_word(sentence);

    let n = 11;
    for i in 0..n {
        println!("Hello, world!!! {}", i);
    } 

    println!("First word is: {}", first_word);

}

fn get_first_word(sentence: String) -> String{

    let mut ans: String = String::from("");

    for char  in sentence.chars() {

        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans;
}
```

### Functions

```rs
fn main() {
    let a: i32 = 10;
    let b: i32 = 20;
    let sum:i32 = do_sum(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
```

### Memory Management

Whenever you run a program (C/C++, Rust, Java, Python, JS), it `allocates` and `deallocates` memory on the RAM.

- Garbage Collector 

1. Usually no dangling pointers/memory issue.
2. You can't do manual memory mangement.
3. Example:- Python, Javascript, Java, etc.


- Manual

1. You alloacte and deallocate memory yourself.
2. Can lead to dangling pointers/memory issue.
3. Learning curve is high since you have to do manual Memory Mangement.
4. Example:- C Language.

- Ownership Model 

1. Rust has its own ownership model for memory management.
2. makes it extremly safe to memory errors.

Memory mangement is a crucial aspect of programming in Rust, desgined to ensure safety and efficiency without the need for a garbage collector.

Not having a garbage collector is one of the key reason rust is so fast.

it achieves this using the

- Mutability
- Heap and Stack
- Ownership Model
- Borrowing and References
- Lifetimes
  
#### Mutability

Immutable variables represent variables whose value can't be changed once assigned.
by default, all variables in rust are immutable because 

1. Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
2. Knowing that certain data will not change allows the compiler to optimise code better.

This code code will throw an error x is immutable

```rs
    {
        let x = 1;
        println!("{}", x);
        x = 2;
        println!("{}", x);
    }
```

to make mutable variable add `mut` keyword to variable, so you can update the value.

```rs
    {
        let mut x = 1;
        println!("{}", x);
        x = 2;
        println!("{}", x);
    }
```

### Stack & Heap

**Stack**: Fast allocation and deallocation, Rust uses the stack for most primitive data types and for data where
the size is known at compile time(example: numbers), organised data.

**Heap**: Used for data that can grow at runtime, such as vectors or strings, disorganised data.

**what store data in ?**
- **Stack Stored** :- Numbers(i32, i64, f64..), Booleans(true or false), Fixed sized arrays.
- **Heap Stored** :- Strings, Vectors.
  
Heap Store Management

```rs 
fn main() {
    let s1 = String::from("hello");
    println!("{}", s1);
}
```

on compile time it locate some space in stack and heap, specficly accroding to code, here 5 bits on heap, and in stack stack frame it store pointing address of store heap first address of h.

demonstration
```
"hello"  ->  stack frame(s1(name, value)) -> heap(index, value)
# code   ->  (ptr->0x01)                  ->    0, h
             len, 5                       ->    1, e
             capacity, 5                  ->    2, l
                                          ->    3, l
                                          ->    4, o

```

On runtime heap can grow.

```rs
fn main() {
    stack_fn();     // call the function that uses stack memory
    heap_fn();      // call the function that uses heap memory
    update_fn();    // call the function that changes size of variables at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_fn() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After updates: {}", s);
    println!("Capacity: {}, Length: {}, pointer: {:p}", s.capacity(), s.len(), s.as_ptr());
}
```

### Ownership

**What is Ownership?**

Ownership is a set of rules that governs how a Rust program manges memory. All program have to manage
the way they use a computer's memory while running. Some language have garbage collection that regularly
looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly
allocate and the free the memory. Rust uses a third approach: memory is managed through a system of ownership
with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile.
None of the features of ownership will slow down your program while it's running.

#### Stack Variables

**Example 1: Passing stack variables inside functions**
```rs
fn main() {
    let x = 1;                          // created on stack, owner is main fn
    let y = 3;                          // created on stack, owner is main fn
    println!("{}", sum(x, y));
    println!("Hello, World!!!");
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;                      // owner of c, a, b is the function sum
    return c;
}
```

This might sound trivial since if the function is popped of the stack, all variables go away with it.

**Example 2: Scoping variables in the same fn**

```rs
fn main() {
    let x = 1; // created on stack
    {
        let y = 3; // created on stack
    }

    println!("{}", y); // throws an error, y is out of scope
}
```

#### Heap Variables

Heap variables, They always want to have a single owner, and if their owner goes out of scope,
they get deallocated.

**Example: Passing strings (heap vars) to functions as args**
**Ownership**
```rs
fn main() {
    let s1 = String::from("Hi there");
    println!("s1: {}", s1); 
    let s2 = s1;

    // WILL NOT COMPILE
    // println!("{}", s1); 
    
    // s2 borrow s1 string
    println!("s2: {}", s2); 
}
```


```rs
// Throws an Error Ownership / Borrowing 
fn main() {
    let my_string: String = String::from("hello");
    takes_ownership(my_string);
    println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // 'some_string' now owns the data.
}
```

**To solve ownership/borrowing errors**
1. Use `.clone()` function which will copy the data to stack.

```rs
takes_ownership(my_string.clone());
```

2.  Retuning var

```rs
fn main() {
    let mut my_string: String = String::from("hello");
    my_string = takes_ownership(my_string);
    println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); // 'some_string' now owns the data.
    return some_string;
}
```

```rs
fn main() {
    let s1: String = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("{}", s2); // This line would cause a compile error because ownership has been moved.
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string); // 'some_string' now owns the data.
    return some_string;
}
```

### Borrowing and References

`References` mean giving the address of a string rather than the ownership of the string over to a function.

```rs
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;

    println!("{}", s2);
    println!("{}", s1); // This is valid, The first pointer wasn't invalidated.
}
```

`Borrowing`: You can transferring ownership of variables to function. By passing a reference to the string to the
function `take_ownership`, the onwership of the string remains with the original variable, in the `main`
function. This allows you to use `my_string` again after the function call.

```rs
fn main() {
    let my_string = String::from("Hello. Rust");
    borrowed_var(&my_string);    // Pass a reference to my_string
    println!("{}", my_string);      // This is vaild because ownership was not transferred.
}

fn borrowed_var(some_string: &String) {
    println!("{}", some_string);    // some_string is borrowed and not owned.
}
```

#### Mutable references

What if you want a function to update the value of a variable?

```rs
fn main() {
    let mut s1 = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}
```

**RULES OF BORROWING**

- There can many `immuables references` at the same time.
- There can be only one `mutable references` at a time.
- If there is a `mutable references`, you can't have another immutable references either.

This is to avoid data races/inconsistent behaviour

If someone makes an immuatable references, they don't expect the value to change suddenly
If more than one mutable references happen, there is a possibility of a data race and synchroization issues.

### Structs

Structs in rust let you structure data yourself. Similiar to objects in javascript.

```rs
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let name = String::from("Alice");
    let user = User{
        name: name,
        age: 30,
        active: true,
    };
    println!("{} is {} years old", user.name, user.age);

    // tuples struct , unit struct
}
```

### Implementing structs

You can also `implement structs`, which means you can attach functions to instances of structs.

```rs
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height // OR return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height) // OR return  2 * (self.width + self.height);
    }
}

fn main() {
    let rect = Rect{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
}
```


### Enums

Enums allow you to define a type by enumerating its possible varants.

### Pattern Matching

Let you pattern match across various variants of an enum and run some logic.

```rs
fn calc_area(shape: Shape) -> f64 {
    let ans: f32 = match shape {
        Shape::Circle(radius: f32) => 3.14 * radius * radius,
        Shape::Rectangle(width:f32, height: f32) => {
            width * height
        }
         Shape::Square(side: f32) => side * side
    }
    return ans;
}
```

### Error Handling

```rs
use std::fs;

enum Result<A, B> {
    Ok(A),
    Err(B),
}

fn main() {
    let res = fs::read_to_string("example.txt");
    match  res {
        Ok(content) => {
            println!("File contents: {}",content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
```

### Options Enum

The Options enum was introduced in Rust to handle the concept of nullability in a safe and
expressive way. Unlike many progarmming languages that uses a null or similiar keyword to 
represent the absence of a value, Rust doesn't have null.

```rs
enum Option<T> {
    None,
    Some(T),
}
```

### Cargo, Packages and Dependencies

Cargo is a package manager for rust, which means we can use it bring packages to our projects.

**Generate a random Number**

first add to rand to system / project.
```bash
cargo add rand
```

code:
```rs
use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();
    let n: u32 = rng.gen();
    println!("Random number is : {}", n);
}
```