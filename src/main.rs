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
    println!("\n******************Structs*******************\n");
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

    let five = Some(5);
    let six = plus_one(five);
    println!("The six is {:?}", six);
    let none = plus_one(None);
    println!("The none is {:?}", none);

    let get_value = match five {
        None => 0,
        Some(i) => i,
    };

    println!("get value from option {} ", get_value);

    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("The value is three");
    } else {
        println!("The value is not three");
    }

    println!("\n\n\n******************Collections*******************\n");
    println!("******************Vectors*******************\n");
    let mut v = vec![1, 2, 3];
    v.push(8);
    println!("vector v vec![1, 2, 3] after pushing a value is {:?}", v);

    // Referncing a vector value
    let third: &i32 = &v[2];

    println!("vector v is {}", third);
    // Referencing using get: v.get(2) gives us the Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    };

    // Iterating over vectors
    for i in &v {
        println!(
            "Iterating over vector v and value of current iteration is {}",
            i
        );
    }

    // Iterating over mutable references
    for i in &mut v {
        *i += 50;
    }

    println!("vector v mutating each value a value is {:?}", v);
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
