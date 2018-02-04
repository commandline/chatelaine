extern crate dotenv;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;

extern crate chatelaine;

use iron::prelude::*;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let _server = Iron::new(chatelaine::route())
        .http("localhost:3000")
        .unwrap();
    info!("Server listening on 3000");
}
