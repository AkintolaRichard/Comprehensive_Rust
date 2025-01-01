/*
if expressions:
You use if expressions exactly like if statements in other languages:
*/
/*fn main() {
    let x = 10;
    if x == 0 {
        println!("zero");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}*/

/*
In addition, you can use if as an expression. The last expression of each
block becomes the value of the if expression:
*/
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}
/*
Because if is an expression and must have a particular type, both of its
branch blocks must have the same type.
An if expression should be used in the same way as the other expression.
For example, when it is used in a let statement, the statement must be
terminated with a ; as well.
*/
