/*
Scope and Shadowing:
A variable's scope is limited to the enclosing block.
You can shadow variables, both those from outer scopes and variables
from the same scope.
*/
fn main() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
/*
Shadowing is different from mutation, because after shadowing both
variables' memory locations exist at the same time. Both are available
under the same name, depending where you use it in the code.
Shadowing looks obscure at first, but is convenient for holding on to
values after .unwrap().
*/
