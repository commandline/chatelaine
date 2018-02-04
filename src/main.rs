extern crate iron;

extern crate chatelaine;

use iron::prelude::*;

use std::collections::HashMap;

fn main() {
    let _server = Iron::new(chatelaine::route(HashMap::new()))
        .http("localhost:3000")
        .unwrap();
    println!("On 3000");
}
