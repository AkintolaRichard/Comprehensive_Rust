/*
Traits
Rust lets you abstract over types with traits. They are similar to
interfaces.
A trait defines a number of methods that types must have in order to
implement the trait.
*/

trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}", self.name)
    }
}

fn main() {
    let fido = Dog { name: String::from("Fido"), age: 5 };
    fido.greet();
}

/*
To implement Trait for Type, you use an impl Trait for Type { .. } block
Traits may provide default implementation of some methods. Default
implementations can rely on all the methods of the trait. In this case,
greet is provided, and relies on talk.
*/
