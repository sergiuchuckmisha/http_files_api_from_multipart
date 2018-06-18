
/**idea of module is to provide ability to read/write files in tmp directory in project folder*/

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Result;

/**
use code from
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/open.html
*/
pub fn read_from_file(file_name: String) -> Result<String> {
    // Create a path to the desired file
//    let path = Path::new("lorem_ipsum.txt");
    let path = Path::new(&file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => return Err(why),
            Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err(why),
        Ok(_) => return Ok(s)
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn write_to_file() {
    let path = Path::new("tmp/qwerty.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all("qwerty".as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}