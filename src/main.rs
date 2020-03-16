mod structs;
use crate::structs::rectangle;
use crate::structs::user_struct;

fn main() {
    println!("******************Structs*******************\n");
    let ameer =
        user_struct::build_user(String::from("ameer"), String::from("ameernormie@gmail.com"));

    println!("Is user active {}", ameer.active);

    let rect = rectangle::get_rect(30, 30);
    println!("The rectangle is {:#?}", rect);
    println!(
        "The area of rectangle is {}",
        rectangle::calculate_area(&rect)
    );
}
