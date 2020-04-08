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
    - [3.2.1 Creating a new string](#creating-a-new-string)
    - [3.2.2 Updating a string](#updating-a-string)
      - [3.2.2.1 Appending to a string with `push_str` and `push`](#appending-to-a-string-with-push_str-and-push)
      - [3.2.2.2 Concatenation with the `+` Operator or the format` Macro](#concatenation-with-the-plus-operator-or-the-format-macro)
    - [3.2.3 Indexing into strings](#indexing-into-strings)
      - [3.2.3.1 Internal Representation](#internal-representation)
      - [3.2.3.2 Bytes and Scalar Values and Grapheme Clusters](#bytes-and-scalar-values-and-grapheme-clusters)
    - [3.2.4 Slicing Strings](#slicing-strings)
    - [3.2.5 Methods for Iterating Over Strings](#methods-for-iterating-over-strings)
  - [3.3 Hash Maps](#hash-maps)
    - [3.3.1 Creating a new hash map](#creating-a-new-hash-map)
    - [3.3.2 Hash Maps and Ownership](#hash-maps-and-ownership)
    - [3.3.3 Accessing Values in a Hash Map](#accessing-values-in-a-hash-map)
    - [3.3.4 Updating a Hash Map](#updating-a-hash-map)
      - [3.3.4.1 Overwriting a Value](#overwriting-a-value)
      - [3.3.4.2 Only Inserting a Value If the Key Has No Value](#only-inserting-a-value-if-the-key-has-no-value)
      - [3.3.4.3 Updating a Value Based on the Old Value](#updating-a-value-based-on-the-old-value)
- [4 Error Handling](#error-handling)
  - [4.1 Unrecoverable Errors](#unrecoverable-errors)
  - [4.1 Recoverable Errors with `Result`](#recoverable-errors-with-result)
- [5 Generic Types, Traits and Lifetimes](#generic-types-traits-and-lifetimes)
  - [5.1 Generics](#generics)
    - [5.1.1 Function definitions](#function-definitions)
    - [5.1.2 In Struct Definitions](#in-struct-definitions)
    - [5.1.3 In Enum Definitions](#in-enum-definitions)
    - [5.1.4 In Method Definitions](#in-method-definitions)
    - [5.1.5 Performance of Code Using Generics](#performance-of-code-using-generics)
  - [5.2 Traits](#traits)
    - [5.2.1 Defining trait](#defining-trait)
    - [5.2.2 Implementing a Trait on a Type](#implementing-a-trait-on-a-type)
    - [5.2.3 Default implementations](#default-implementations)
    - [5.2.4 Traits as Parameters](#traits-as-parameters)
      - [5.2.4.1 Trait Bound Syntax](#trait-bound-syntax)
      - [5.2.4.2 Specifying Multiple Trait Bounds with the plus syntax](#specifying-multiple-trait-bounds-with-the-plus-syntax)
      - [5.2.4.3 Clearer Trait Bounds with `where` Clauses](#clearer-trait-bounds-with-where-clauses)
    - [5.2.5 Returning Types that Implement Traits](#returning-types-that-implement-traits)
    - [5.2.6 Using Trait Bounds to Conditionally Implement Methods](#using-trait-bounds-to-conditionally-implement-methods)
  - [5.3 Lifetimes](#lifetimes)
    - [5.3.1 The borrow Checker](#the-borrow-checker)
    - [5.3.2 Generic Lifetimes in Functions](#generic-lifetimes-in-functions)
    - [5.3.3 Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    - [5.3.4 Lifetime Annotations in Function Signatures](#lifetime-annotations-in-function-signatures)
    - [5.3.5 Thinking in Terms of Lifetimes](#Thinking-in-terms-of-lifetimes)
    - [5.3.6 Lifetime Annotations in Struct Definitions](#lifetime-annotations-in-struct-definitions)
    - [5.3.7 Lifetime Elision](#lifetime-elision)
      - [5.3.7.1 First rule](#first-rule)
      - [5.3.7.2 Second rule](#second-rule)
      - [5.3.7.3 Third rule](#third-rule)
    - [5.3.8 Lifetime Annotations in Method Definitions](#lifetime-annotations-in-method-definitions)
    - [5.3.9 The Static Lifetime](#the-static-lifetime)
- [6 Functional language features: Iterators and Closures](#functional-language-features)
  - [6.1 Closures](#closures)
  - [6.2 Iterators](#iterators)
    - [6.2.1 The `Iterator` Trait and the `next` method](#the-iterator-trait-and-the-next-method)

---

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

---

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

---

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

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

> The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. both String and string slices are `UTF-8` encoded.

##### Creating a new string

```rust
let mut s = String::new();

let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();

let s = String::from("initial contents");
```

##### Updating a string

A String can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format! macro` to concatenate `String` values.

###### Appending to a string with `push_str` and `push`

```rust
let mut s = String::from("foo");
s.push_str("bar");

// s will now be foobar

let mut s = String::from("lo");
s.push('l');
```

`

###### Concatenation with the `plus` Operator or the format Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

The reason `s1` is no longer valid after the addition and the reason we used a reference to `s2` has to do with the signature of the method that gets called when we use the `+` operator. The `+` operator uses the add method, whose signature looks something like this:
`fn add(self, s: &str) -> String {`

We can see in the signature that add takes ownership of self, because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and no longer be valid after that. So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```

For more complicated string combining, we can use the `format!` macro:

##### Indexing into strings

In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.

```rust
// This is invalid code
let s1 = String::from("hello");
let h = s1[0];
```

###### Internal Representation

**A String is a wrapper over a Vec<u8>.**

```rust
let len = String::from("Hola").len(); // 4
let len = String::from("Здравствуйте").len();  // 24
```

`len` returns the number of `bytes` it takes to encode a string.

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
```

What should the value of answer be? Should it be `З`, the first letter? When encoded in `UTF-8`, the first byte of `З` is `208` and the second is `151`, so answer should in fact be `208`, but `208` is not a valid character on its own. Returning `208` is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index `0`. Users generally don’t want the byte value returned, even if the string contains only Latin letters: if `&"hello"[0]` were valid code that returned the byte value, it would return `104`, not `h`. To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

###### Bytes and Scalar Values and Grapheme Clusters

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective:

1. Bytes
2. Scalar values
3. Grapheme clusters (closest thing to a letter)

If we look at the Hindi word `नमस्ते` written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

```rust
// That’s 18 bytes and is how computers ultimately store this data.
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

// If we look at them as Unicode scalar values, which are what Rust’s char type
['न', 'म', 'स', '्', 'त', 'े']

// Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters
["न", "म", "स्", "ते"]
```

##### Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a `byte` value, a `character`, a `grapheme` cluster, or a `string slice`. To be more specific in your indexing and indicate that you want a `string slice`.

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, `s` will be a `&str` that contains the first `4 bytes` of the string. Earlier, we mentioned that each of these characters was `2 bytes`, which means `s` will be `Зд`.

What would happen if we used `&hello[0..1]`? The answer: Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

##### Methods for Iterating Over Strings

If you need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method. Calling chars on `नमस्ते` separates out and returns six values of type char, and you can iterate over the result to access each element:

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}


for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

#### Hash Maps

> The type `HashMap<K, V>` stores a mapping of keys of type K to values of type V. It does this via a hashing function, which determines how it places these keys and values into memory.

##### Creating a new hash map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Another way of constructing a hash map is by using the collect method on a vector of tuples, where each tuple consists of a key and its value.

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

##### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

##### Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

Here, score will have the value that’s associated with the Blue team, and the result will be `Some(&10)`. The result is wrapped in Some because get returns an `Option<&V>`.

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

##### Updating a Hash Map

1. Replace the old value with the new value, completely disregarding the old value
2. Keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
3. Combine the old value and the new value

###### Overwriting a Value

`insert` overwrites the value

###### Only Inserting a Value If the Key Has No Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

###### Updating a Value Based on the Old Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

The or_insert method actually returns a mutable reference (&mut V) to the value for this key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (\*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

---

### Error Handling

- Recoverable errors
- Unrecoverable errors

Rust doesn't have `exceptions`. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

#### Unrecoverable Errors

> Rust has the `panic!` macro. When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

##### Unwinding the Stack or Aborting in Response to a Panic

> By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:

```rust
[profile.release]
panic = 'abort'
```

#### Recoverable Errors with `Result`

Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. `Result` enum is defined as having two variants, `Ok` and `Err`, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` represents the type of the value that will be returned in a success case within the `Ok` variant, and `E` represents the type of the error that will be returned in a failure case within the `Err` variant.

```rust
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => {
        panic!("Problem opening the file: {:?}", error)
    },
};
```

###### Matching on Different Errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

The type of the value that `File::open` returns inside the `Err` variant is `io::Error`, which is a struct provided by the standard library. This struct has a method kind that we can call to get an `io::ErrorKind` value. The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an `io` operation. The variant we want to use is `ErrorKind::NotFound`, which indicates the file we’re trying to open doesn’t exist yet. So we match on `f`, but we also have an inner match on `error.kind()`.

##### Shortcuts for Panic on Error: `unwrap` and `expect`

If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us. Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

##### Propagating Errors

When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

The body of the function starts by calling the `File::open` function. Then we handle the `Result` value returned with a match similar to the match in above code, only instead of calling `panic!` in the `Err` case, we return early from this function and pass the error value from `File::open` back to the calling code as this function’s error value. If `File::open` succeeds, we store the file handle in the variable `f` and continue.

Then we create a new `String` in variable s and call the `read_to_string` method on the file handle in `f` to read the contents of the file into `s`. The `read_to_string` method also returns a `Result` because it might fail, even though `File::open` succeeded. So we need another `match` to handle that `Result`: if `read_to_string` succeeds, then our function has succeeded, and we return the username from the file that’s now in s wrapped in an `Ok`. If `read_to_string` fails, we return the error value in the same way that we returned the error value in the match that handled the return value of `File::open`. However, we don’t need to explicitly say return, because this is the last expression in the function.

###### A Shortcut for Propagating Errors: the `?` Operator

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

The `?` placed after a `Result` value is defined to work in almost the same way as the match expressions we defined to handle the `Result` values in the previous example. If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

In the context of the above example, the `?` at the end of the `File::open` call will return the value inside an `Ok` to the variable `f`. If an error occurs, the `?` operator will return early out of the whole function and give any `Err` value to the calling code. The same thing applies to the `?` at the end of the `read_to_string` call.

`Even shorter version`

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

###### The ? Operator Can Be Used in Functions That Return Result

```rust
// Fails
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

This error points out that we’re only allowed to use the `?` operator in a function that returns `Result` or `Option` or another type that implements `std::ops::Try`. We have two choices to fix this problem. One technique is to change the return type of your function to be `Result<T, E>` if you have no restrictions preventing that. The other technique is to use a `match` or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.

The `main` function is special, and there are restrictions on what its return type must be. One valid return type for main is `()`, and conveniently, another valid return type is `Result<T, E>`, as shown here:

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

---

### Generic Types, Traits and Lifetimes

#### Generics

> We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

##### Function definitions

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.

`Same function with two different definitions because of different types`

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

```

To parameterize the types in the new function we’ll define, we need to name the type parameter, just as we do for the value parameters to a function. You can use any identifier as a type parameter name. But we’ll use T because, by convention.

When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means. Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it. To define the generic largest function, place type name declarations inside angle brackets, `<>`, between the name of the function and the parameter list, like this: `fn largest<T>(list: &[T]) -> T {`

> We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a value of the same type T.

```rust
fn largest<T>(list: &[T]) -> T {
let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest

}
```

The above code doesn't work because it gives the error that `>` operation doesn't work for all types. To solve this issue, we'll use `traits`. We'll learn about traits a bit later.

##### In Struct Definitions

We can also define structs to use a generic type parameter in one or more fields using the <> syntax.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

##### In Enum Definitions

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

##### In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

##### Performance of Code Using Generics

> Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. `Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.`

The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with. Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.

#### Traits

> A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

##### Defining a Trait

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

###### Scenario

`For example, let’s say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can have at most 280 characters along with metadata that indicates whether it was a new tweet, a retweet, or a reply to another tweet. We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we need to request that summary by calling a summarize method on an instance.`

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

After the method signature, instead of providing an implementation within curly brackets, we use a semicolon. **Each type implementing this trait must provide its own custom behavior for the body of the method**. The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

##### Implementing a Trait on a Type

Now that we’ve defined the desired behavior using the Summary trait, we can implement it on the types in our media aggregator.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

After implementing the trait, we can call the methods on instances of NewsArticle and Tweet in the same way we call regular methods, like this:

```rust
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

Note that because we defined the Summary trait and the NewsArticle and Tweet types in the same lib.rs in above example, they’re all in the same scope. Let’s say this `lib.rs` is for a crate we’ve called `aggregator` and someone else wants to use our crate’s functionality to implement the `Summary` trait on a struct defined within their library’s scope. They would need to bring the trait into their scope first. They would do so by specifying `use aggregator::Summary;`, which then would enable them to implement Summary for their type. The Summary trait would also need to be a public trait for another crate to implement it, which it is because we put the `pub` keyword before trait in Listing

> One restriction to note with trait implementations is that `we can implement a trait on a type only if either the trait or the type is local to our crate`. For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our `aggregator` crate functionality, because the type `Tweet` is local to our `aggregator` crate. We can also implement `Summary` on `Vec<T>` in our aggregator crate, because the trait `Summary` is local to our `aggregator` crate. But we can’t implement external traits on external types. For example, we can’t implement the `Display` trait on `Vec<T>` within our `aggregator` crate, because `Display` and `Vec<T>` are defined in the standard library and aren’t local to our `aggregator` crate. This restriction is part of a property of programs called `coherence`, and more specifically the `orphan rule`, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

##### Default implementations

> Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

To use a default implementation to summarize instances of `NewsArticle` instead of defining a custom implementation, we specify an empty impl block with `impl Summary for NewsArticle {}`.
Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

##### Traits as Parameters

Define functions that accept many different types.

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the `Summary` trait, such as `summarize`. We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`.

###### Trait Bound Syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a trait bound; it looks like this:

```rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// More complexity with two parameteres that implement Summary
pub fn notify(item1: impl Summary, item2: impl Summary) {

// If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:
pub fn notify<T: Summary>(item1: T, item2: T) {
```

###### Specifying Multiple Trait Bounds with the plus syntax

We can also specify more than one trait bound. Say we wanted `notify` to use `display` formatting on item as well as the `summarize` method: we specify in the `notify` definition that item must implement both `Display` and `Summary`. We can do so using the `+` syntax:

```rust
pub fn notify(item: impl Summary + Display) {}

pub fn notify<T: Summary + Display>(item: T) {}
```

###### Clearer Trait Bounds with `where` Clauses

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature.

```rust
// Instead of writing this
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// Write this
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

##### Returning Types that Implement Traits

We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait, as shown here:

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

> By using `impl Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the Summary trait without naming the concrete type. In this case, `returns_summarizable` returns a `Tweet`, but the code calling this function doesn’t know that.

##### Using Trait Bounds to Conditionally Implement Methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type `Pair<T>` in below example always implements the `new` function. But `Pair<T>` only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait. **Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations** and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:

```rust
impl<T: Display> ToString for T {
    // --snip--
}
```

#### Lifetimes

> Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

`The following code does not work`

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

`Error`

```rust
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

The variable `x` doesn’t “live long enough.” The reason is that `x` will be out of scope when the inner scope ends on line 7. But `r` is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” If Rust allowed this code to work, `r` would be referencing memory that was deallocated when `x` went out of scope, and anything we tried to do with `r` wouldn’t work correctly. So how does Rust determine that this code is invalid? It uses a `borrow checker`.

##### The borrow Checker

The Rust compiler has a `borrow checker` that compares scopes to determine whether all borrows are valid.

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}
```

Here, we’ve annotated the lifetime of r with `'a` and the lifetime of `x` with `'b`. As you can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn’t live as long as the reference.

```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}
```

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

##### Generic Lifetimes in Functions

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
```

The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `x` or `y`. Actually, we don’t know either, because the if block in the body of this function returns a reference to `x` and the else block returns a reference to `y`!

##### Lifetime Annotation Syntax

> Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. Most people use the name `'a`.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter `first` that is a reference to an `i32` with lifetime `'a`. The function also has another parameter named `second` that is another reference to an i32 that also has the lifetime `'a`. The lifetime annotations indicate that the references `first` and `second` must both live as long as that generic lifetime.

##### Lifetime Annotations in Function Signatures

We need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list. The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. We’ll name the lifetime `'a` and then add it to each reference.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

`Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes.`

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

In this example, `string1` is valid until the end of the outer scope, `string2` is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves of this code; it will compile and print The longest string is long string is long.

`Error`

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

As humans, we can look at this code and see that `string1` is longer than `string2` and therefore result will contain a reference to `string1`. Because `string1` has not gone out of scope yet, a reference to `string1` will still be valid for the println! statement. However, the compiler can’t see that the reference is valid in this case. **We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in**. Therefore, the borrow checker disallows the code in above example as possibly having an invalid reference.

##### Thinking in Terms of Lifetimes

If we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter.

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

##### Lifetime Annotations in Struct Definitions

So far, we’ve only defined structs to hold owned types. It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

This struct has one field, `part`, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.

##### Lifetime Elision

The following code compiles without lifetime error

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

> The patterns programmed into Rust’s analysis of references are called the `lifetime elision rules`. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

1. `Input Lifetimes`: Lifetimes on function or method parameters
2. `Output Lifetimes`: Lifetimes on return values

The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

###### First rule

The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

###### Second rule

The second rule is if there is exactly one `input lifetime` parameter, that lifetime is assigned to all `output lifetime` parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

###### Third rule

The third rule is if there are multiple `input lifetime` parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

##### Lifetime Annotations in Method Definitions

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the l`ifetime elision rules` often make it so that lifetime annotations aren’t necessary in method signatures.

##### The Static Lifetime

One special lifetime we need to discuss is `'static`, which means that this reference can live for the entire duration of the program. All string literals have the `'static` lifetime, which we can annotate as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is `'static`.

You might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not.

---

### Functional language features

#### Closures

> Anonymous Functions that Can Capture Their Environment

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

The above code has multiple calls to the slow calculation function. The first if block calls `simulated_expensive_calculation` twice, the if inside the outer else doesn’t call it at all, and the code inside the second else case calls it once.

###### Our goal

> we want to refactor this code so it calls the simulated_expensive_calculation function only once. We want to define code in one place in our program, but only execute that code where we actually need the result. This is a use case for closures!

##### Refactoring with closures

Instead of always calling the `simulated_expensive_calculation` function before the if blocks, we can define a closure and store the `closure` in a variable rather than storing the result of the function call.

```rust
generate_workout(intensity: u32, random_number: u32) {
    // Closure
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
```

This closure has one parameter named `num`: if we had more than one parameter, we would separate them with commas, like `|param1, param2|`.
**`expensive_closure` contains the definition of anonymous function not the resulting value of calling the function. we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in `expensive_closure`.**

##### Closures type inference

Closures don’t require you to annotate the types of the parameters or the return value like fn functions do. Closures are usually short and relevant only within a narrow context rather than in any arbitrary scenario. The compiler is reliably able to infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables.

> Closure definitions will have one concrete type inferred for each of their parameters and for their return value.

Following code will error out.

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

`The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked in to the closure in example_closure, and we get a type error if we try to use a different type with the same closure.`

##### Storing Closures Using Generic Parameters and the `Fn` Traits

We can create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as `memoization` or `lazy evaluation`.

To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields. `Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different`. To define structs, enums, or function parameters that use closures, we use generics and trait bounds.

The `Fn` traits are provided by the standard library. All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.

We add types to the `Fn` trait bound to represent the types of the parameters and return values the closures must have to match this trait bound. In this case, our closure has a parameter of type `u32` and returns a `u32`, so the trait bound we specify is `Fn(u32) -> u32`.

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

The `Cacher` struct has a calculation field of the generic type `T`. The trait bounds on `T` specify that it’s a closure by using the `Fn` trait. Any closure we want to store in the calculation field must have one `u32` parameter (specified within the parentheses after `Fn`) and must return a `u32` (specified after the `->`).

> Note: Functions can implement all three of the Fn traits too. If what we want to do doesn’t require capturing a value from the environment, we can use a function rather than a closure where we need something that implements an Fn trait.

The `value` field is of type `Option<u32>`. Before we execute the closure, `value` will be `None`. When code using a `Cacher` asks for the result of the closure, the `Cacher` will execute the closure at that time and store the result within a `Some` variant in the `value` field. Then if the code asks for the result of the closure again, instead of executing the closure again, the `Cacher` will return the result held in the `Some` variant.

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

Instead of saving the closure in a variable directly, we save a new instance of `Cacher` that holds the closure. Then, in each place we want the result, we call the `value` method on the `Cacher` instance. We can call the `value` method as many times as we want, or not call it at all, and the expensive calculation will be run a maximum of once.

##### Limitations of the `Cacher` Implementation

The first problem is that a `Cacher` instance assumes it will always get the same value for the parameter `arg` to the `value` method. That is, this test of `Cacher` will fail:

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

The second problem with the current `Cacher` implementation is that it only accepts closures that take one parameter of type `u32` and return a `u32`. We might want to cache the results of closures that take a `string slice` and return `usize` values, for example. To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.

##### Capturing the Environment with Closures

Closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead that we don’t want to pay in more common cases where we want to execute code that doesn’t capture its environment. Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: `taking ownership`, `borrowing mutably`, and `borrowing immutably`. These are encoded in the three Fn traits as follows:

- `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The `Once` part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
- `FnMut` can change the environment because it mutably borrows values.
- `Fn` borrows values from the environment immutably.

**When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.** All closures implement `FnOnce` because they can all be called at least once. Closures that don’t move the captured variables also implement `FnMut`, and closures that don’t need mutable access to the captured variables also implement `Fn`. In above example, the `equal_to_x` closure borrows `x` immutably (so `equal_to_x` has the `Fn` trait) because the body of the closure only needs to read the value in `x`.

If you want to force the closure to take ownership of the values it uses in the environment, you can use the `move` keyword before the parameter list. `This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.`

Following code will fail:

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);  // x is moved so it is not valid here

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

The `x` value is moved into the closure when the closure is defined, because we added the `move` keyword. The closure then has ownership of `x`, and main isn’t allowed to use `x` anymore in the `println!` statement. Removing `println!` will fix this example.

> Most of the time when specifying one of the Fn trait bounds, you can start with Fn and the compiler will tell you if you need FnMut or FnOnce based on what happens in the closure body.

#### Iterators

###### Iterators are lazy

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

Following code creates an `iterator` over the items in vector `v1`. The code itself doesn't do anything useful.

```rust
let v1 = vec![1, 2, 3];

// Iterator is stored but not iteration takes place
let v1_iter = v1.iter();

// Each element in iterator is used in one iteration of loop
for val in v1_iter {
    println!("Got: {}", val);
}
```

##### The `Iterator` Trait and the `next` method

All Iterators implement a trait named `Iterator` from the standard library.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

Things to note:

1. We needed to make `v1_iter` mutable: calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
2. We didn’t need to make `v1_iter` mutable when we used a `for` loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes.
3. Values we get from the calls to next are immutable references to the values in the vector.
4. The `iter` method produces an `iterator` over immutable references.
5. If we want to create an `iterator` that takes ownership of `v1` and returns owned values, we can call `into_iter` instead of iter.
6. If we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.
