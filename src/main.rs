mod structs;
use crate::structs::rectangle;
use crate::structs::user_struct;

#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(unused_variables)]
#[allow(dead_code)]
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

    println!(
        "Dime value in cents using enum coin {:?}",
        value_in_cents(Coin::Dime)
    );
    println!(
        "Quarter of state Alabama has value in cents using enum coin {:?}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the state of quarter is {:?}", state);
            25
        }
    }
}
