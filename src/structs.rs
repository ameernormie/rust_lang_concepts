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

    pub fn get_rect(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn calculate_area(rect: &Rectangle) -> u32 {
        rect.height * rect.width
    }
}
