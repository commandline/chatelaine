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
extern crate luminal_pathparam;
extern crate luminal_router;
//extern crate r2d2;
//extern crate r2d2_diesel;
//extern crate rand;

use hyper::header::ContentLength;
use hyper::server::{self, Request, Response};
use luminal_router::{Router, ServiceFuture};

use std::collections::HashMap;
//use std::env;

//mod auth;
//mod db;

//use self::db::DieselMiddleware;

pub fn route() -> Router {
    let router = Router::new();

    let router = router
        .get("/", server::service_fn(handler))
        .expect("Should have been able to add index route")
        .get("/:query", server::service_fn(handler))
        .expect("Should have been able to add index route with param");

    debug!("Routes added.");
    router
}

fn handler(req: Request) -> ServiceFuture {
    info!("Index handler: '{}'", req.path());
    let query = if req.path() == "/" {
        info!("Default");
        String::from("Hello")
    } else {
        let params =
            luminal_pathparam::parse("/:query", req.path()).collect::<HashMap<&str, &str>>();
        if let Some(query) = params.get(":query") {
            info!("Queried: {}", query);
            (*query).to_owned()
        } else {
            info!("Default");
            String::from("Hello")
        }
    };
    let response = Response::new()
        .with_header(ContentLength(query.len() as u64))
        .with_body(query);
    Box::new(futures::future::ok(response))
}
