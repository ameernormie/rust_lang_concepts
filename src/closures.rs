/*
Consider this hypothetical situation: we work at a startup that’s making an app to generate
custom exercise workout plans. The backend is written in Rust, and the algorithm that generates
the workout plan takes into account many factors, such as the app user’s age, body mass index,
exercise preferences, recent workouts, and an intensity number they specify. The actual algorithm used
isn’t important in this example; what’s important is that this calculation takes a few seconds.
We want to call this algorithm only when we need to and only call it once so we don’t
make the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the function `simulated_expensive_calculation`
*/

use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                // If value is changed then save new otherwise return already cached value
                if v != arg {
                    let new_value = (self.calculation)(arg);
                    self.value = Some(new_value);
                    new_value
                } else {
                    v
                }
            }
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        println!(
            "value with arg {} is {}",
            intensity,
            expensive_result.value(intensity)
        );
        println!("value with arg 4 is {}", expensive_result.value(4));
        println!("value with arg 4 is {}", expensive_result.value(4));
        println!("value with arg 4 is {}", expensive_result.value(4));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_cacher_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
