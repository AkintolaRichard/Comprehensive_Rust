/*
Let Control Flow
Rust has a few control flow constructs which differ from other languages.
They are used for pattern matching:
* if let expressions
* let else expressions
* while let expressions
if let expressions
The if let expressions lets you execute different code depending on
whether a value matches a pattern.

let else expressions
For the common case of matching a pattern and returning from the function,
use let else. The "else" case must diverge (return, break, or panic -
anything but falling off the end of the block).

while let
Like with if let, there is a while let variant which repeatedly tests a
value against a pattern.
*/
use std::time::Duration;

fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
