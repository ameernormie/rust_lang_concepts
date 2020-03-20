# Rust Concepts

### Table of Contents

- [1 Structs](#structs)
  - [1.1 Create instances from other instances](#create-instances-from-other-instances)
    - [1.1.1 Without update syntax](#without-update-syntax)
    - [1.1.1 With update syntax](#with-update-syntax)
  - [1.2 Tuple Structs](#tuple-structs)
  - [1.3 Unit like Structs without any fields](#unit-like-structs-without-any-fields)
  - [1.4 Method Syntax](#method-syntax)
    - [1.4.1 Defining methods](#defining-methods)
    - [1.4.2 Associated functions](#associated-functions)
  - [1.5 Ownership of Struct Data](#ownership-of-struct-data)
- [2 Enums and Pattern Matching](#enums-and-pattern-matching)
  - [2.1 Defining Enums](#defining-enums)
  - [2.2 Enums Values](#enums-values)
  - [2.3 Option Enum](#option-enum)
  - [2.4 Match Control Flow Operator](#match-control-flow-operator)
    - [2.4.1 Patterns that Bind to Values](#patterns-that-bind-to-values)
    - [2.4.2 Matching with Option<T>](#matching-with-option)
  - [2.5 Concise Control Flow with if let](#concise-control-flow-with-if-let)
- [3 Common Collections](#common-collections)
  - [3.1 Vectors](#vectors)
    - [3.1.1 Creating a New Vector](#creating-a-new-vector)
    - [3.1.2 Updating a Vector](#updating-a-vector)
    - [3.1.3 Reading Elements of Vector](#reading-elements-of-vector)
    - [3.1.4 Iterating over values of Vector](#iterating-over-values-of-vector)
    - [3.1.5 Using an Enum to Store Multiple Types](#using-an-enum-to-store-multiple-types)
  - [3.2 String](#string)
  - [3.3 Hash Maps](#hash-maps)

### Structs

**A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.**

A simple struct:

```rust
// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    pub active: bool,
}

// Immutable instance
let ameer =  User {
    username: String::from("ameer"),
    email: String::from("ameernormie@gmail.com"),
    sign_in_count: 1,
    pub active: true,
}

// Mutable instance
let mut ameer =  User {
    username: String::from("ameer"),
    email: String::from("ameernormie@gmail.com"),
    sign_in_count: 1,
    pub active: true,
}

ameer.username = String::from("ameernormie");
```

#### Create instances from other instances

It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You’ll do this using struct update syntax.

##### Without update syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

##### With update syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

#### Tuple Structs

They are similar to `tuples`. They have the added meaning the struct name provides but don't have names associated with them; rather they just have the types of fields.
`They are useful when you want to give the whole tuple a name and differentiate them from normal tuples`

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

A gotcha:
`In above example,`black`and`origin`are not of the same type. Each struct you define is its own type, even though the fields within the struct have the same types.`

#### Unit like Structs without any fields

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

#### Method Syntax

Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct, and their first parameter is always self, which represents the instance of the struct the method is being called on.

##### Defining methods

To define the function within the context of Rectangle, we start an impl (implementation) block. Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

##### Associated functions

Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter. These are called associated functions because they’re associated with the struct.

Associated functions are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let a_square = Rectangle::square(20);
```

#### Ownership of Struct Data

It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss later.Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

### Enums and Pattern Matching

`Enums allow you to define a type by enumerating its possible variants.`

##### Defining Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

##### Enums Values

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. The reason this is useful is that now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`. We can then, for instance, define a function that takes any `IpAddrKind`:
`fn route(ip_kind: IpAddrKind) { }`

Enums can represent what kind of data they'll have. Rather than an enum inside a struct, by putting data directly into each enum variant.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, so there is no need for an extra struct.
There’s another advantage to using an `enum` rather than a `struct`: **each variant can have different types and amounts** of associated data. Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

##### We can define methods on enums like structs

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

#### Option Enum

`The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other programming languages.`
Rust doesn’t have the null feature that many other languages have.
The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by the standard library.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. In addition, so are its variants: you can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.
`<T>` means the Some variant of the Option enum can hold one piece of data of any type.

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If we use None rather than Some, we need to tell Rust what type of `Option<T>` we have, because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.

In short, because `Option<T>` and T (where T can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an i8 to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

In other words, you have to convert an `Option<T>` to a `T` before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
**Everywhere that a value has a type that isn’t an `Option<T>`, you can safely assume that the value isn’t null.**
In order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner `T`. You want some other code to run if you have a None value, and that code doesn’t have a `T` value available.

#### `Match` Control flow Operator

Rust has an extremely powerful control flow operator called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression.

##### Patterns that Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

##### Matching with `Option`

In the previous section, we wanted to get the inner `T` value out of the Some case when using`Option<T>`; we can also handle`Option<T>` using match as we did with the Coin enum! Instead of comparing coins, we’ll compare the variants of`Option<T>`, but the way that the match expression works remains the same.

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` and `Some` cases.

##### The `_` pattern

Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the special pattern \_ instead:

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

#### Concise Control Flow with if let

The `if let` syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// Instead we can write the above code as this (less verbose)
if let Some(3) = some_u8_value {
    println!("three");
}
```

We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the \_ case in the match expression that is equivalent to the `if let` and else.

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

### Common Collections

Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

1. `Vector`: A vector allows you to store a variable number of values next to each other.
2. `String`: A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
3. `Hash Map`: A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

#### Vectors

Vectors allow you to store more than one value in a single data structure that `puts all the values next to each other in memory`. Vectors can only store values of the same type.

##### Creating a New Vector

```rust
let v: Vec<i32> = Vec::new();
```

We added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.

Rust can often infer the type of value you want to store once you insert values, so you rarely need to do this type annotation. It’s more common to create a `Vec<T>` that has initial values, and Rust provides the vec! macro for convenience.

```rust
let v = vec![1, 2, 3];
```

##### Updating a Vector

```rust
let mut v = Vec::new();

v.push(5);
```

##### Reading Elements of Vector

There are two ways to reference a value stored in a `vector`.

1. Using reference to the element like `let value = &v[1]`
2. Using `get` method, like `v.get(1)`. This gives us the `Option<&T>`

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

```rust
let v = vec![1, 2, 3, 4, 5];

// This method will cause the program to panic because it references a nonexistent element.
let does_not_exist = &v[100];

// This is same because it will return `None` without panicking
let does_not_exist = v.get(100);
```

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules. You can’t have mutable and immutable references in the same scope.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

`Why should a reference to the first element care about what changes at the end of the vector?`
This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory.

##### Iterating over values of Vector

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

##### Using an Enum to Store Multiple Types

Fortunately, `the variants of an enum are defined under the same enum type`, so when we need to store elements of a different type in a vector, we can define and use an enum!

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

#### String

#### Hash Maps
