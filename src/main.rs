mod closures;
mod generics_traits_lifetimes;
mod iterators;
mod structs;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::closures as ClosureModule;
use crate::generics_traits_lifetimes::generics as Generics;
use crate::generics_traits_lifetimes::traits as Traits;
use crate::generics_traits_lifetimes::traits::Summary;
use crate::iterators as Iterators;
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

    println!("\n******************Strings*******************\n");
    let mut name = String::from("ameer");
    let last_name = " hamza";
    name.push_str(last_name);

    println!("My name is {} after using push_str method", name);

    for c in "नमस्ते".chars() {
        println!("iterating over string नमस्ते in char   {}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("iterating over string नमस्ते in bytes   {}", b);
    }

    println!("\n******************Hash Maps*******************\n");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    // Only Inserting a Value If the Key Has No Value
    scores.entry(String::from("Red")).or_insert(100);

    println!("hashmap scores {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let new_scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("hashmap new scores {:?}", new_scores);

    let team_name = String::from("Blue");

    println!(
        "get the score of Blue team {:?}",
        new_scores.get(&team_name)
    );

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    println!("\n\n\n******************Error Handling*******************\n");
    let read_from_file_result = read_username_from_file();

    match read_from_file_result {
        Ok(res) => println!("the result is {}", res),
        Err(e) => println!("error while reading hello.txt {}", e),
    }

    let read_from_file_using_question_operator_result =
        read_username_from_file_using_question_operator();

    match read_from_file_using_question_operator_result {
        Ok(res) => println!("the result using ? operator {}", res),
        Err(e) => println!("error while reading hello.txt using ? operator {}", e),
    }

    let read_from_file_using_question_operator_improved_result =
        read_username_from_file_using_question_operator_improved();

    match read_from_file_using_question_operator_improved_result {
        Ok(res) => println!("the result using ? operator improved {}", res),
        Err(e) => println!(
            "error while reading hello.txt using ? operator improved {}",
            e
        ),
    }

    println!("\n\n\n******************Generics, Traits and Lifetimes*******************\n");
    println!("\n******************Generics*******************\n");
    let num_list = vec![1, 2, 3, 4, 5, 6];
    let largest = Generics::largest(&num_list);

    println!(
        "Largest value using generic function in list [1, 2, 3, 4, 5, 6] is {}",
        largest
    );

    let p1 = Generics::Point { x: 4, y: 3.9 };
    let p2 = Generics::Point { x: "Hello", y: 'c' };
    println!("two points using generic structs {:?}", p1);
    println!("x of point {}", p1.x());
    println!("y of point {}", p1.y());

    let p3 = p1.mixup(p2);

    println!("p3.x = {} and p3.y = {}", p3.x, p3.y);

    println!("\n******************Traits*******************\n");
    let tweet = Traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(tweet);

    let news_article = Traits::NewsArticle {
        headline: String::from("Florida man does something"),
        location: String::from("Florida"),
        author: String::from("A florida man"),
        content: String::from(
            "Florida man does something and you know what a florida man does; everything",
        ),
    };

    notify(news_article);

    let tweet2 = returns_summarizable();
    notify(tweet2);

    println!("\n******************Lifetimes (Validating references)*******************\n");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let longest_string = longest_of_two_strings(string1.as_str(), string2);
    println!("longest of two strings is {}", longest_string);

    println!(
        "\n\n\n******************Functional features: Closures and Iterators*******************\n"
    );
    println!("\n******************Closures*******************\n");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    ClosureModule::generate_workout(simulated_user_specified_value, simulated_random_number);

    println!("\n******************Iterators*******************\n");
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Iteration: {}", val);
    }

    println!("\nCustom iterator implementation that counts first five numbers");
    let counter = Iterators::Counter::new();

    for count in counter {
        println!("count is: {}", count);
    }
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

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_using_question_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_using_question_operator_improved() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Traits as parameters
// functions that accept many different types.
fn notify(item: impl Summary) {
    println!("Traits as parameters: Breaking news! {}", item.summarize())
}

// Returning Types that implement traits
fn returns_summarizable() -> impl Summary {
    Traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn longest_of_two_strings<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
