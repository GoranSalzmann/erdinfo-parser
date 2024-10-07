use std::{fs::File, io::Read};

pub fn cmd_list(path: &String) {
    let mut bytes = File::open(path)
        .expect("Could not open .erdinfo file!")
        .bytes();

    while let Some(Ok(byte)) = bytes.next() {
        println!("{:x}", byte);
    }
}
