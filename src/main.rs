use std::{fs::{self, File}, io::{self, Read, ErrorKind}};

fn main() {
    create_file_1();
    create_file_2();
    create_file_3();
    _ = read_username_from_file_1();
    _ = read_username_from_file_2();
    _ = read_username_from_file_3();
    _ = read_username_from_file_4();
}

// Simple Match
pub fn create_file_1() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// Matching on Different Errors
pub fn create_file_2() {
    let f = File::open("Hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem creating the file: {:?}", other_error)
            }
        },
    };
}

// Alternatives to Using match with Result<T, E>
pub fn create_file_3() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// Propagating Errors -- early return
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Propagating Errors -- A Shortcut for Propagating Errors: the ? Operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Propagating Errors -- Chaining method calls after the ? operator
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("Hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Propagating Errors -- shotest way
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("Hello.txt")
}
