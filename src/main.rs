mod structs;
use crate::structs::rectangle;
use crate::structs::user_struct;

fn main() {
    println!("******************Structs*******************\n");
    let ameer =
        user_struct::build_user(String::from("ameer"), String::from("ameernormie@gmail.com"));

    println!("Is user active {}", ameer.active);

    let rect = rectangle::get_rect(30, 30);
    rect.print_rect();
    println!("The area of rectangle is {}", rect.area());
    let a_square_rect = rectangle::get_square_rect(10);
    println!("The square rectangle is {:#?}", a_square_rect);

    println!("\n\n\n******************Enums*******************\n");
    println!("******************Option Enum*******************\n");

    let some_value = Some(3);

    // This will error out because Rust needs to know what type of Option<T> we have.
    // Because compile can't infer the type by looking at None
    // let none_value = None;

    let working_none: Option<u32> = None;

    println!("Some value {:?}", some_value);
    println!("None value {:?}", working_none);
}
