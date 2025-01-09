/*
Borrow Errors
As a concrete example of how these borrowing rules prevent memory errors,
consider the case of modifying a collection while there are references to
it elements:
*/
/*fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = &vec[2];
    vec.push(6);
    println!("{elem}");
}*/

// Similarly, consider the case of iterator invalidation:
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for elem in &vec {
        vec.push(elem * 2);
    }
}

/*
 * In both of these cases, modifying the collection by pushing new elements
  into it can potentially invalidate existing references to the colection's
  elements if the collection has to reallocate.
*/
