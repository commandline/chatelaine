//extern crate argon2rs;
//extern crate base64;
//#[macro_use]
//extern crate diesel;
//extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate luminal_router;
//extern crate r2d2;
//extern crate r2d2_diesel;
//extern crate rand;

use hyper::header::ContentLength;
use hyper::server::{self, Request, Response};
use luminal_router::{Router, ServiceFuture};

//use std::env;

//mod auth;
//mod db;

//use self::db::DieselMiddleware;

pub fn route() -> Router {
    let mut router = Router::new();

    router
        .get("/", server::service_fn(handler))
        .expect("Should have been able to add index route");

    debug!("Routes added.");
    router
}

fn handler(_req: Request) -> ServiceFuture {
    info!("Index handler");
    let msg = String::from("index");
    let response = Response::new()
        .with_header(ContentLength(msg.len() as u64))
        .with_body(msg);
    Box::new(futures::future::ok(response))
}
