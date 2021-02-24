use std::{fs::File, io::prelude::*};

fn main() {
    // TODO: Download
    // TODO: into target dir
    let mut file = File::create("driver").unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
