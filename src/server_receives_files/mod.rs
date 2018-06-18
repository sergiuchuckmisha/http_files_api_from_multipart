extern crate hyper;
extern crate multipart;

use config::{SERVER_TEMPORARY_FOLDER_PATH, PORT};
use std::io;
use std;
use self::hyper::server::{Handler, Server, Request, Response};
use self::hyper::status::StatusCode;
use self::hyper::server::response::Response as HyperResponse;
use self::multipart::server::hyper::{Switch, MultipartHandler, HyperRequest};
use self::multipart::server::{Multipart, Entries, SaveResult};
use self::multipart::mock::StdoutTee;
use std::sync::Arc;

struct NonMultipart;
impl Handler for NonMultipart {
    fn handle(&self, _: Request, mut res: Response) {
        *res.status_mut() = StatusCode::ImATeapot;
        res.send(b"Please send a multipart req :(\n").unwrap();
    }
}

struct EchoMultipart;
impl MultipartHandler for EchoMultipart {
    fn handle_multipart(&self, mut multipart: Multipart<HyperRequest>, res: HyperResponse) {
        match multipart.save().temp() {
            SaveResult::Full(entries) => process_entries(res, entries).unwrap(),
            SaveResult::Partial(entries, error) => {
                println!("Errors saving multipart:\n{:?}", error);
                process_entries(res, entries.into()).unwrap();
            }
            SaveResult::Error(error) => {
                println!("Errors saving multipart:\n{:?}", error);
                res.send(format!("An error occurred {}", error).as_bytes()).unwrap();
            }
        };
    }
}

fn process_entries(res: HyperResponse, entries: Entries) -> io::Result<()> {
    let mut res = res.start()?;
    let stdout = io::stdout();
    let out = StdoutTee::new(&mut res, &stdout);

    for (name, entries) in &entries.fields {
        //means file is found and should be processed: it's content should be saved to SERVER_TEMPORARY_FOLDER_PATH
        if name.eq(&Arc::new("file".to_string())) {
            for (idx, field) in entries.iter().enumerate() {
                let mut data = field.data.readable()?;
                let headers = &field.headers;

                let mut file = std::fs::File::create(format!("{}{}", SERVER_TEMPORARY_FOLDER_PATH, headers.clone().filename.unwrap())).unwrap();
                std::io::copy(&mut data, &mut file).unwrap();
            }
        }
    }

    entries.write_debug(out)
}

fn main() {
    println!("Listening on 127.0.0.1:{}", PORT);
    Server::http(format!("127.0.0.1:{}", PORT)).unwrap().handle(
        Switch::new(
            NonMultipart,
            EchoMultipart
        )).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run()
    {
        main();
    }
}