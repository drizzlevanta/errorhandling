use std::{
    fs::{self, File},
    io::{self, Read},
};

fn propagate_errors() -> Result<String, io::Error> {
    let file = File::open("hellouser.txt");

    let mut username_file = match file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn use_errors() {
    match propagate_errors() {
        Ok(username) => println!("username is {}", username),
        Err(e) => panic!("propagated error {:?}", e),
    }
}

fn propagate_errors_shortcut2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hellouser.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn propagate_errors_shortcut1() -> Result<String, io::Error> {
    let mut file = File::open("hellouser.txt")?;

    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file() {
    //shortest path to read file and to string
    fs::read_to_string("hellouser.txt");
}
