pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub struct InfiniteEvenNumbers {
    num: u32,
}

impl InfiniteEvenNumbers {
    pub fn new() -> InfiniteEvenNumbers {
        InfiniteEvenNumbers { num: 0 }
    }
}

impl Iterator for InfiniteEvenNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.num += 2;

        Some(self.num)
    }
}

#[test]
fn counter_next_method() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
