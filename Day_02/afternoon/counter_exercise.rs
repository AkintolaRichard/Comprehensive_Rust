/*
Exercise: Counter
In this exercise you will take a very simple data structure and make it
generic. It uses a std::collections::HashMap to keep track of which values
have been seen and how many times each one has appeared.

The initial version of Counter is hard coded to only work for u32 values.
Make the struct and its methods generic over the type of value being tracked,
that way Counter can track any type of value.
If you finish early, try using the entry method to halve the number of hash
lookups required to implement the count method.
*/

use std::collections::HashMap;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
#[derive(Debug)]
struct Counter<T>{
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T>{
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}
