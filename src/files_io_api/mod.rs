
/**idea of module is to provide ability to read/write files in tmp directory in project folder*/

const TEMPORARY_FOLDER_PATH: &str = "tmp/";

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Result;

/**
use code from
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/open.html
*/
pub fn read_from_file(file_name: &str) -> Result<String> {
    // Create a path to the desired file
//    let path = Path::new("lorem_ipsum.txt");
    let file_name_2 = &(TEMPORARY_FOLDER_PATH.to_owned() + &file_name);//todo get rid of extra variable
    let path = Path::new(&file_name_2);

//    let display = path.display();

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

/**
use code
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/create.html
*/
pub fn write_to_file(file_name: &str, file_content: &str) -> Result<()> {
//    let path = Path::new("tmp/qwerty.txt");
    let file_name_2 = &(TEMPORARY_FOLDER_PATH.to_owned() + &file_name);//todo get rid of extra variable
//    let path = Path::new(&("tmp/".to_owned() + &file_name));
    let path = Path::new(file_name_2);
//    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(file_content.as_bytes()) {
        Err(why) => return Err(why),
        Ok(_) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        write_to_file("qwerty.txt", "qwerty");
    }


    #[test]
    fn test_read() {
        write_to_file("qwerty.txt", "qwerty");
        assert_eq!("qwerty", read_from_file("qwerty.txt").unwrap());
    }

    #[test]
    #[should_panic]
    fn test_read_negative() {
        assert_eq!("qwerty", read_from_file("qwerty2.txt").unwrap());
    }
}