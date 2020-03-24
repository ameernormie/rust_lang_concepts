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
