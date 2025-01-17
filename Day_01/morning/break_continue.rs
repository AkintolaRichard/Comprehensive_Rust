/*
break and continue:
If you want to immediately start the next iteration use continue.
If you want to exit any kind of loop early, use break. With loop,
this can take an optional expression that becomes the value of the
loop expression.
*/
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}
