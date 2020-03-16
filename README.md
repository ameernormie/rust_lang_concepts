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
