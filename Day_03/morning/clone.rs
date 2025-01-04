/*
Clone
Sometimes you want to make a copy of a value. The Clone trait accomplishes
this.
* clone generally performs a deep copy of the value, meaning that if you
 e.g. clone an array, all the elements of the array are cloned as well.
* The behaviour for clone is user-defined, so it can perform  custom
 cloning logic if needed.
*/

fn say_hello(name: String) {
    println!("Hello {name}");
}

fn main() {
    let name = String::from("Alice");
    say_hello(name.clone());
    say_hello(name);
}
