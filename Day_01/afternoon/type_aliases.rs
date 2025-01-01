/*
Type Aliases:
A type alias creates a name for another type. The two types can be used
interchangeably
*/

enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, Rwlock};
type PlayerInventory = Rwlock<Vec<Arc<RefCell<Item>>>>;

/*
 * A newtype is often a better alternative since it creates a distinct type.
 Prefer struct InventoryCount(usize) to type InventoryCount = usize.
 * C programmers will recognize this as similar to a typedef.
*/
