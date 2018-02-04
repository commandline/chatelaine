extern crate iron;

extern crate chatelaine;

use iron::prelude::*;

fn main() {
    let _server = Iron::new(chatelaine::route())
        .http("localhost:3000")
        .unwrap();
    println!("On 3000");
}
