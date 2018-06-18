mod files_io_api;

use files_io_api::read_from_file;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::read_from_file;

    #[test]
    fn test_read() {
        println!("qwerty");
        println!("{:?}", super::read_from_file("LICENSE".to_string()).unwrap());
        println!("qwerty");
    }

}