use std::{fs::File, io::ErrorKind};

pub fn open_file() {
    let file_result = File::open("hello.txt");
    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //if not found create file
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            //panic on other errors
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
