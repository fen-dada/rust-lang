use std::fs::File;
use std::io::ErrorKind;
fn main() {
    //    panic!("nmsl");
    let file_open_result = File::open("hello.txt");
    //  let greeting_file = File::open("hello.txt");
    //      .unwarp();
    //      .expect("Error opening file");
    let file_open = match file_open_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Error creating file cuz {:?}", err),
            },
            other_error => panic!("Error opening file {:?}", other_error),
        },
    };
}
