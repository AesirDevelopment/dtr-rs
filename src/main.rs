extern crate iron;

use iron::*;
use std::io::Read;

fn main() {
    Iron::new(|request: &mut Request| {
        let mut buffer = String::new();
        request.body.read_to_string(&mut buffer);
        let response = buffer;

        Ok(Response::with((status::Ok, response)))
    }).http("localhost:3000").unwrap();
}
