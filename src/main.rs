extern crate dotenv;
extern crate env_logger;
extern crate hyper;

extern crate chatelaine;

use hyper::server::Http;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let address = "127.0.0.1:3000"
        .parse()
        .expect("Unable to parse bind address.");
    let server = Http::new()
        .bind(&address, || Ok(chatelaine::route()))
        .expect("Unable to create server.");
    server.run().expect("Unable to run server");
}
