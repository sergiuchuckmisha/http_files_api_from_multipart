/**
idea of this mod is to make tests which uploads files to server

idea of test:
 - clear tmp folder
 - upload file to server
 - assuming that server will keep loaded files in the tmp folder
 - verify that uploaded file in tmp folder exists
*/
extern crate hyper;
extern crate multipart;

use self::hyper::client::Request;
use self::hyper::method::Method;
use self::hyper::net::Streaming;

use self::multipart::client::Multipart;

use std::io::Read;

use config::PORT;
use config::CLIENT_TEMPORARY_FOLDER_PATH;
use files_io_api::*;
use std::io::Result;

fn prepare_data(file_name: &str, file_content: &str) -> Result<()> {
    init(CLIENT_TEMPORARY_FOLDER_PATH);
    write_to_file(file_name, file_content, CLIENT_TEMPORARY_FOLDER_PATH)
}

/**function creates file*/
pub fn upload_data_as_file(file_name: &str, file_content: &str) -> Result<()> {
    prepare_data(file_name, file_content);

//    let url = "http://localhost:3333".parse()
    let url = format!("{}{}", "http://localhost:", PORT).parse()
        .expect("Failed to parse URL");

    let request = Request::new(Method::Post, url)
        .expect("Failed to create request");

    let mut multipart = Multipart::from_request(request)
        .expect("Failed to create Multipart");

    write_body(&mut multipart, &format!("{}{}", CLIENT_TEMPORARY_FOLDER_PATH, file_name))
        .expect("Failed to write multipart body");

    let mut response = multipart.send().expect("Failed to send multipart request");

    if !response.status.is_success() {
        let mut res = String::new();
        response.read_to_string(&mut res).expect("failed to read response");
        println!("response reported unsuccessful: {:?}\n {}", response, res);
    }

    Ok(())
    // Optional: read out response
}

pub fn write_body(multi: &mut Multipart<Request<Streaming>>, file_path: &str) -> hyper::Result<()> {
    let mut binary = "Hello world from binary!".as_bytes();

//    multi.write_text("text", "Hello, world!")?;
//    multi.write_text("client", "client_name")?;
    multi.write_file("file", file_path)?;
    // &[u8] impl Read
    multi.write_stream("binary", &mut binary, None, None)
        .and(Ok(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_data()
    {
        prepare_data("file_name", "file_content");
    }

    #[test]
    fn test_upload_data_as_file()
    {
        upload_data_as_file("file_name.txt", "file_content3");//todo understand why '.txt' extension is mandatory for test to pass
    }
}