extern crate iron;
#[macro_use]
extern crate router;

use iron::middleware::*;
use iron::prelude::*;
use iron::status;
use router::Router;

use std::collections::HashMap;

mod auth;

pub fn route(credentials: HashMap<String, String>) -> Chain {
    let router = router!(index: get "/" => handler,
                         query: get "/:query" => handler);
    let mut chain = Chain::new(router);
    let auth = auth::Auth { credentials };
    chain.link_around(auth);
    chain
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let query = &req.extensions
        .get::<Router>()
        .unwrap()
        .find("query")
        .unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}
