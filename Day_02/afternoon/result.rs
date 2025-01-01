/*
Result
Result is similar to Option, but indicates the success or failure of an
operation, each with a different enum variant. It is generic: Result<T, E>
where T is used in the Ok variant and E appears in the Err variant.
*/

use std::fs::File;
use std::io::Read;

fn main() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} {bytes} bytes");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}

/*
 * As with Option, the successful value sits inside of Result, forcing the
 developer to explicitly extract it. This encourages error checking. In the
 case where an error should never happen, unwrap() or expect() can be called,
 and this is a signal of the developer intent too.
*/
