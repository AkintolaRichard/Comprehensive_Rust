struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person { name: String::from("Jackie"), ..avery };
    describe(&jackie);
}

/*
Key Points
 * If you already have variables with the right names, then you can create
 the struct using a shorthand.
 * The syntax ..avery allows us to copy te majority of the fields from the
 old struct without having to explicitly type it all out. It must always be
 the last element.
*/
