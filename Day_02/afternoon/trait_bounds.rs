/*
Trait Bounds
When working with generics, you often want to require the types to
implement some traits, so that you can this trait's methods.
You can do this with T: Trait
*/
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

#[derive(Debug)]
struct NotCloneable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    // let noclone = NotCloneable;
    // let no_pair = duplicate(noclone);
    println!("{pair:?}");
    // println!("{no_pair:?}");
}

/*
* Try making a NonCloneable and passing it to duplicate.
* When multiple traits are necessary, use + to join them.
* Show a where clause, students will encounter it when reading code.
* fn duplicate<T>(a: T) -> (T, T) {
* where
*   T: Clone,
* {
*       (a.clone(), a.clone())
* }
    * It declutters the function signature if you have many parameters.
    * It has additional features making it more powerful.
        * If someone asks, the extra feature is that the type on the left
         ":" can be arbitrary, like Option<T>.
* Note that Rust does not (yet) support specialization. For example, given
 the original duplicate, it is invalid to add a specialized
 duplicate(a: u32).
*/
