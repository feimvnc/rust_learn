# rust_learn

## Setup 
1. Automatic linting & formating
2. Live reloading
3. Continuous integration

## Workflow
1. Make project working 
2. Make project right (working in proper way)
3. Make project working fast

## Important crates

#### Serialization and Deserialization
serde
serde_json
#### Error handling
thiserror
anyhow 
#### Async for Rust
tokio

## Rust Specific Design Patterns

#### Extension traits, add additional type to std library

/*
pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for String {
    fn is_palindrome(&self) -> bool {
        let s = self.chars().collect::<Vec<char>>();
        s == s.iter().rev().cloned().collect::<Vec<char>>()
    }
}

use my_crate::Palindrome;

fn main() {
    let s = String::from("racecar");
    s.is_palindrome();
}
*/

#### Other patterns

Type state
Interior mutability
RAII
Builder

## Rust Features

#### Zero-Cost Abstractions (for memory and thread safety)

What you don't use, you don't pay for.  
What you do use, you couldn't hand code any better.

High level generic abstractions:  
generics, iterators, templates, collections, classes

Low level abstractions
for loops, counters, if/else, raw pointers

#### Ownership Resource Acquisition Is Initialization (RAII)

Tied to object lifetime

allocated memory
file handles
database connections 

Ownership Rules
1. Each value in Rust has a variable that's called it's owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

#### Borrowing Rules

1. At any given time, you can have either one mutable reference or any number of immutable references. (Prevent races)
2. References must be always be valid.

/*

let mut connection = DatabaseConnection::new("example.db")?;

let conn1 = &Connection;
let conn2 = &Connection;
let conn3 = &mut connection;  // error, violate Borrowing Rules

Ok(())
*/

#### Algebraic Data Types (from Functional Languages)

Sum Types (Or Variant Types)
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

Product Types (similar to C)
struct Person {
    nae: String, 
    age: u32, 
    height: f32,
}

#### Polymorphism Traits & Generics

Traits: Defines a set of functions and methods that types can implement
 
1. Flexibility & Composition
2. Non-invasive & Extensive
3. No of Fragile Base Classs Problem
4. Static Dispatch & Performance


#### async / await

Lazy
Parallelism

#### Meta programming

Macros 




