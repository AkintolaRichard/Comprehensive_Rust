/*
Documentation
Rust comes with extensive documentation. For example:
* All of the details about loops.
* Primitive types like u8.
* Standard library types like Option or BinaryHeap.

Use rustup doc --std or https://std.rs to view the documentation.

In fact, you can document your own code:
*/

/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0;
}

/*
The contents are treated as Markdown.
To document an item from inside the item (such as inside a module), use //!
or /*! .. */, called "inner doc comments"
*/
