extern crate hyper;
extern crate futures;

use futures::future::Future;
use futures::Stream;

// use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

struct DaemonService;



impl Service for DaemonService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        let future = match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Help:");
                futures::future::ok(response)
            },
            (&Method::Post, "/run") => {
                response.set_status(StatusCode::NoContent);
                futures::future::ok(response)
            },
            (&Method::Post, "/stop") => {
                let f = req.body().concat2().map(|chunk| {
                    let content = String::from_utf8(chunk.to_vec());
                    content.unwrap_or("".to_string())
                }).and_then(move |content|{
                    use std::thread;
                    use std::time::Duration;
                    thread::sleep(Duration::from_secs(30));
                    response.set_body(format!("Stop response: {}", content))
                });

                f
            },
            _ => {
                response.set_status(StatusCode::NotFound);
                futures::future::ok(response)
            }
        };

        Box::new(future)
    }
}

fn main() {
    let addr = "127.0.0.1:3003".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(DaemonService)).unwrap();
    server.run().unwrap();
}
