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
use files_io_api::*;
use std::io::Result;
use std::path::Path;

use config::PORT;
use config::CLIENT_TEMPORARY_FOLDER_PATH;
use files_io_api::visit_dirs::visit_dirs;


fn save_content_to_file<P: AsRef<Path>, B: AsRef<[u8]>>(file_name: P, file_content: B) -> Result<()>
{
    write_to_file(file_name, file_content, CLIENT_TEMPORARY_FOLDER_PATH)
}

/**function creates file*/
pub fn upload_data_as_file<P: AsRef<Path>, B: AsRef<[u8]>>(file_name: P, file_content: B) -> Result<()>
    where P: Copy
{
    save_content_to_file(file_name, file_content);
    upload_file(file_name)
}

pub fn upload_file<P: AsRef<Path>>(file_path: P) -> Result<()>
{
//    let url = "http://localhost:3333".parse()
    let url = format!("{}{}", "http://localhost:", PORT).parse()
        .expect("Failed to parse URL");

    let request = Request::new(Method::Post, url)
        .expect("Failed to create request");

    let mut multipart = Multipart::from_request(request)
        .expect("Failed to create Multipart");

//    write_body(&mut multipart, &format!("{}{}", CLIENT_TEMPORARY_FOLDER_PATH, file_path))
    write_body(&mut multipart, &(CLIENT_TEMPORARY_FOLDER_PATH.to_string() + &file_path.as_ref().as_os_str().to_os_string().into_string().unwrap()))
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

pub fn write_body<P: AsRef<Path>>(multi: &mut Multipart<Request<Streaming>>, file_path: P) -> hyper::Result<()> {
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
        init(CLIENT_TEMPORARY_FOLDER_PATH);
        save_content_to_file("file_name", "file_content");
    }

    #[test]
    fn test_upload_data_as_file()
    {
//        upload_data_as_file("qqqq.txt", "file_content7");//todo understand why '.txt' extension is mandatory for test to pass
        upload_data_as_file("hyper_server.exe", "file_content7");//todo understand why '.txt' extension is mandatory for test to pass
    }

    /**idea of the test is to find all files in certain dir and upload them to the server*/
    #[test]
    fn test_upload_folder() {
        //1st: write down all files's pathes
        //use code
        //https://doc.rust-lang.org/std/fs/fn.read_dir.html

        visit_dirs(CLIENT_TEMPORARY_FOLDER_PATH, &|ref entry| {
//            upload_file(entry.path());
            println!("{:?}", entry.path())
        });
    }
}