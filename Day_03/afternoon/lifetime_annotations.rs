/*
Lifetime Annotations
A reference has a lifetime, which must not "outlive" the value it refers to.
This is verified by the borrow checker.

The lifetime can be implicit-this is what we have seen so far. Lifetime can
also be explicit: &'a Point, &'document str. Read &'a Point as "a borrowed
Point which is valid for at least the lifetime a".

Lifetimes are inferred by the compiler: you cannot assign a lifetime yourself.
Explicit lifetime annotations create constraints where there is ambiguity; the
compiler verifies that there is a valid solution.

Lifetime become more complicated when considering passing values to and
returning values from functions.
*/

#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn main() {
    let p1 = Point(10, 10);
    println!("p1: {:p}", &p1);
    let p2 = Point(20, 20);
    println!("p2: {:p}", &p2);
    let p3 = left_most(&p1, &p2); // What is the lifetime of p3?
    println!("p3: {:p}", &p3);
    println!("p3: {p3:?}");
    //println!("p1: {p1:?}");
}

/*
In this example, the compiler does not know what lifetime to infer for p3.
Looking inside the function body shows that it can only safely assume that p3's
lifetime is the shorter of p1 and p2. But just like types, Rust requires
explicit annotations of lifetimes on function arguments and return values.

Add 'a appropriately to left_most:
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {

This says, "given p1 and p2 which both outlive 'a, the return value lives for
at least 'a.
*/
