use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};
//let mut username = String::new();
//File::open("hello.txt")?.read_to_string(&mut username)?;
//Ok(username);
fn read_name_from_file() -> Result<String,io::Error> { // fs::read_to_string("hello.txt")
    let file_result = File::open("hello.txt");

    let mut file_open = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),//its a Result type of the whole function
    };

    let mut username = String::new();

    match file_open.read_to_string(&mut username) { //borrow a mutable reference to username
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
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
    read_name_from_file();
}
