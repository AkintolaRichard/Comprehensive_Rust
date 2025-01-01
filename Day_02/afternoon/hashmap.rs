/*
HashMap
Standard hash map with protection against HashDoS attacks.
*/

use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn", 207);
    page_counts.insert("Grimms' Fairy Tales", 751);
    page_counts.insert("Pride and Prejudice", 303);

    if !page_counts.contains_key("Les Miserables") {
        println!(
            "We know about {} books, not Les Miserables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:?}");
}

/*
 * HashMap is not defined in the prelude and nedds to be brought into scope.
 * Try the following lines of code. The first line will see if a book is in
  the hashmap and if not return an alternative value. The second line will
  insert the alternative value in the hashmap if the book is not found.
    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);
    let p2 = page_counts
        .entry("The Hunger Games")
        .or_insert(374)
 * Unlike vec!, there is unfornately no standard hashmap! macro
    * Although, since Rust 1.56, HashMap implements From<[(K, V); N]>, which
     allows us to easily initialize a hash map from a literal array:
    let page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
 * Alternatively HashMap can be built from any Iterator which yields
  key-value tuples.
 * This type has several "method-specific" return types, such as
  std::collections::hash_map::Keys. These types often appear in searches of
  the Rust docs.
*/
