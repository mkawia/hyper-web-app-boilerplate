extern crate hyper;

extern crate tokio_core;
extern crate tokio_service;

extern crate futures;


extern crate serde;
extern crate serde_derive;
extern crate serde_json;

extern crate url;

use tokio_service::Service;

use hyper::StatusCode;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response};

static TEXT: &'static str = "<h1>Hello, World!</h1>";

struct WebService;

impl Service for WebService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<futures::Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let response = Response::<hyper::Body>::new()
            .with_header(ContentLength(TEXT.len() as u64))
            .with_header(ContentType::html())
            .with_status(StatusCode::Ok)
            .with_body(TEXT);


        Box::new(futures::future::ok(response))
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let server = match Http::new().bind(&addr, || Ok(WebService)) {
        Ok(s) => {
            println!("Server running at localhost:3000");
            s
        }
        Err(err) => {
            panic!("Error starting server {:?}", err);
        }
    };

    match server.run() {
        Ok(_) => {}
        Err(err) => {
            panic!("Error running server future {:?}", err);
        }
    };
}
