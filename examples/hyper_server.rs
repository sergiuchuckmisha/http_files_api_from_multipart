extern crate hyper;
extern crate multipart;

use std::io;
use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use hyper::server::response::Response as HyperResponse;
use multipart::server::hyper::{Switch, MultipartHandler, HyperRequest};
use multipart::server::{Multipart, Entries, SaveResult};
use multipart::mock::StdoutTee;
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
        if name.eq(&Arc::new("file".to_string())) {
            println!("found file");
            for (idx, field) in entries.iter().enumerate() {
                let mut data = field.data.readable()?;
                let headers = &field.headers;

                let mut file = std::fs::File::create(headers.clone().filename.unwrap()).unwrap();
                std::io::copy(&mut data, &mut file).unwrap();
            }
        }
    }

    entries.write_debug(out)
}

fn main() {
    println!("Listening on 127.0.0.1:3333");
    Server::http("127.0.0.1:3333").unwrap().handle(
        Switch::new(
            NonMultipart,
            EchoMultipart
        )).unwrap();
}
