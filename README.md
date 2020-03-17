# Rust Concepts

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

##### Create instances from other instances

It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You’ll do this using struct update syntax.

###### Without update syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

###### With update syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

##### Tuple Structs

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

##### Unit like Structs without any fields

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

##### Method Syntax

Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct, and their first parameter is always self, which represents the instance of the struct the method is being called on.

###### Defining methods

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

###### Associated functions

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

##### Ownership of Struct Data

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
