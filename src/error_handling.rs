#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use core::panic;
use std::{io, fs, error, num::ParseIntError};

pub fn unrecoverable_errors() {
    let v = vec!["one", "two", "three"];
    println!("{}", v[3]);

    panic!("Somethign went horribly wrong");
}

pub fn recoverable_errors() {
    let file = fs::File::open("example.txt").expect("Failed to open file!");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file: {:?}", error)
    //     }
    // };

}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can not be empty".to_owned())
    } else {
        Ok(1)
    }
}
// -------------

pub fn propagating_errors() {
    let content = read_file("example.txt");
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut contents = fs::read_to_string(filename)?;
    Ok(contents)
}

//---------

pub fn multiple_error_types() {
    let i = parse_file("example.txt");
}

enum ParseFileError {
    File,
    Parse(ParseIntError)
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

//----------------------
// ADVANCED ERROR HANDLING
//----------------------

