pub mod generics {
    // pub fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];

    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // This allows us to use two different types in struct
    #[derive(Debug)]
    pub struct Point<T, U> {
        pub x: T,
        pub y: U,
    }

    impl<T, U> Point<T, U> {
        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn y(&self) -> &U {
            &self.y
        }

        pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

pub mod traits {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        // Default implementation
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }

        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
