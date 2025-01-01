/*
Exercise: Fibonacci
The Fibonacci sequence begins with [0, 1]. For n > 1, the n'th Fibonacci
number is calculated recursively as the sum of the n-1'th and n-2'th
Fibonacci numbers.
Write a function fib(n) that calculates the n'th Fibonacci number. When
will  this function panic?
*/

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return n;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

/*
It should panic when the value is greater than the width of u32
*/
