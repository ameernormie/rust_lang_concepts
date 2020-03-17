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
    println!(
        "The area of rectangle is {}",
        rect.area()
    );
    let a_square_rect = rectangle::get_square_rect(10);
    println!(
        "The square rectangle is {:#?}",
        a_square_rect
    );
}
