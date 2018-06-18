
/**idea of module is to provide ability to read/write files in tmp directory in project folder*/

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
use code from
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/open.html
*/
//pub fn read_from_file(file_name: String) -> Result<Box<String>, Error> {
//pub fn read_from_file(file_name: String) -> Option<Box<String>> {
pub fn read_from_file(file_name: String) -> Option<String> {
    // Create a path to the desired file
//    let path = Path::new("lorem_ipsum.txt");
    let path = Path::new(&file_name);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
//        Err(why) => return Err(why),
//            Ok(file) => file,
            Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
//        Err(why) => return Err(why),
        Err(why) => None,
//            panic!("couldn't read {}: {}", display, why.description()),
//        Ok(_) => return Ok(s)
        Ok(_) => return Some(s)
//            print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
