pub mod user_struct {
    #[allow(dead_code)]
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        pub active: bool,
    }

    pub fn build_user(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: false,
        }
    }
}

pub mod rectangle {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn print_rect(&self) {
            println!("The rectangle is {:#?}", self);
        }

        pub fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size
            }
        }
    }

    pub fn get_rect(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn get_square_rect(size: u32) -> Rectangle {
        Rectangle::square(size)
    }

}
