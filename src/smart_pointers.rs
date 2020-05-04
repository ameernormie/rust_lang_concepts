use std::ops::Deref;
use std::rc::Rc;

// Enabling Recursive Types with Boxes
// A cons list is a data structure that comes from the Lisp programming language and its dialects.
// In Lisp, the cons function (short for “construct function”) constructs a new pair from its two
// arguments, which usually are a single value and another pair. These pairs containing pairs form a list.

// Each item in a cons list contains two elements: the value of the current item and the next item.
// The last item in the list contains only a value called Nil without a next item. A cons list is
// produced by recursively calling the cons function.

// Using Box<T> to Get a Recursive Type with a Known Size
pub enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}

pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// Defining Our Own Smart Pointer

// Let’s build a smart pointer similar to the Box<T> type provided by the standard library
// to experience how smart pointers behave differently from references by default.
// Then we’ll look at how to add the ability to use the dereference operator.
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping custom smart pointer data `{}`!", self.data);
    }
}

// Interior Mutability: A Mutable Borrow to an Immutable Value

// Here’s the scenario we’ll test: we’ll create a library that tracks a value against a
// maximum value and sends messages based on how close to the maximum value the current
// value is. This library could be used to keep track of a user’s quota for the number
// of API calls they’re allowed to make, for example.

// Our library will only provide the functionality of tracking how close to the maximum a value
// is and what the messages should be at what times. Applications that use our library will be
// expected to provide the mechanism for sending the messages: the application could put a message
// in the application, send an email, send a text message, or something else. The library doesn’t need
// to know that detail. All it needs is something that implements a trait we’ll provide called Messenger.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
