
/**idea of module is to provide ability to read/write files in tmp directory in project folder*/

use config::TEMPORARY_FOLDER_PATH;

use std::fs::{File, remove_dir_all, create_dir_all};
use std::io::prelude::*;
use std::path::Path;
use std::io::Result;

/**
use code from
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/open.html
*/
pub fn read_from_file(file_name: &str, folder_path: &str) -> Result<String> {
    // Create a path to the desired file
//    let path = Path::new("lorem_ipsum.txt");
    let file_name_2 = format!("{}{}", folder_path, file_name);//todo get rid of extra variable
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
removes tmp folder and recreates it
todo for some reason sometimes remove and create overlaps and folder is not created. Investigate?
*/
pub fn init(folder_path: &str) -> Result<()>  {
    remove_dir_all(folder_path)?;
    create_dir_all(folder_path)?;
    Ok(())
}

/**
use code
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/create.html
*/
pub fn write_to_file(file_name: &str, file_content: &str, folder_path: &str) -> Result<()> {
    // use code https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/2
    let file_name_2 = format!("{}{}", folder_path, file_name);//todo get rid of extra variable
    let path = Path::new(&file_name_2);

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
    fn test_init() {
        init(TEMPORARY_FOLDER_PATH);
    }

    #[test]
    fn test_write() {
        write_to_file("qwerty.txt", "qwerty", TEMPORARY_FOLDER_PATH);
    }


    #[test]
    fn test_read() {
        init(TEMPORARY_FOLDER_PATH);
        write_to_file("qwerty.txt", "qwerty", TEMPORARY_FOLDER_PATH);
        assert_eq!("qwerty", read_from_file("qwerty.txt", TEMPORARY_FOLDER_PATH).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_read_negative() {
        init(TEMPORARY_FOLDER_PATH);
        assert_eq!("qwerty", read_from_file("qwerty2.txt", TEMPORARY_FOLDER_PATH).unwrap());
    }
}