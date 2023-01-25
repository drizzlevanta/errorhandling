use std::fs::File;

pub fn _unwrap() {
    let _file = File::open("hello.txt").unwrap();
}

pub fn expect() {
    //expect is similar to unwrap but can have customized msg
    let _file = File::open("hello.txt").expect("Hello.txt should be included in the dir");
}
