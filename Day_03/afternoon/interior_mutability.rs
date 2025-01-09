/*
Interior Mutability
In some situations, it's necessary to modify data behind a shared(read-only)
reference. For example, a shared data structure might have an internal cache,
and wish to update that cache from read-only methods.

Note: In Rust, an exclusivve reference is a mutable reference: that allows the
value it refers to to be changed. An exclusive reference means that no other
reference to the same value can exist at the same time.

The "interior mutability" pattern allows exclusive (mutable) access behind a
shared reference. The standard library provides several ways to do this, all
while still ensuring safety, typically by performing a runtime check.
*/

/*
Cell
Cell wraps value and allows getting or setting the value using only a shared
reference to the Cell. However, it does not allow any references to the inner
value. Since there are no references, borrowing rules cannot be broken.
*/
/*use std::cell::Cell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);

    cell.set(123);
    println!("{}", cell.get());
}*/

/*
RefCell
RefCell allows accessing and mutating a wrapped value by providing alternative
types Ref and RefMut that emulate &T/&mut T without actually being Rust
references.

These types perform dynamic checks using a counter in the RefCell to prevent
existence of a RefMut alongside another Ref/RefMut.

By implementing Deref (and DerefMut for RefMut), these types allow calling
methods on the inner value without allowing references to escape.
*/
use std::cell::RefCell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = RefCell::new(5);

    {
        let mut cell_ref = cell.borrow_mut();
        *cell_ref = 123;

        // This triggers an error at runtime.
        // let other = cell.borrow();
        // println!("{}", *other);
    }

    println!("{cell:?}");
}
