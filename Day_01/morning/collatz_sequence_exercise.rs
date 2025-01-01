/*
Exercise: Collatz
The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
* if ni is 1, then the sequence terminates at ni.
* if ni is even, then ni+1 = ni / 2.
* if ni is odd, then ni+1 = 3 * ni + 1.
*/
// Write a function to calculate the length of the collatz sequence for a
// given initial n.

/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;

    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}
