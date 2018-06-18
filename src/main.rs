mod files_io_api;

use files_io_api::*;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read() {
        println!("qwerty");
        println!("{:?}", read_from_file("LICENSE".to_string()).unwrap());
        println!("qwerty");
    }

    #[test]
    fn test_write() {
//        write_to_file();
        write_to_file("qwerty.txt".to_string(), "qwerty".to_string());//todo get rid of to_string()
    }

}