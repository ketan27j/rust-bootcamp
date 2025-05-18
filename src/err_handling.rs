use std::fs;
use core::fmt;
use std::{fmt::{Debug, Formatter}};

fn main() {
    //let file = fs::read_to_string("hello.txt");
    // match file {
    //     Ok(content) => println!("{}", content),
    //     Err(e) => println!("Error reading file: {}", e),
    // }
    let file = read_file("hello.txt".to_string());
    match file {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error reading file: {:?}", e.message),
    }
}

pub struct FileReadError{
    message: String,
}

fn read_file(file_path: String) -> Result<String, FileReadError> {
    let greeting_file_result = fs::read_to_string(file_path);
    match greeting_file_result {
        Ok(file_content) => {
            Ok(file_content)
        },
        Err(error) => {
            let err = FileReadError { message: error.to_string() };
            Err(err)
        }
    }
}