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
