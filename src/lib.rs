#[macro_use]
extern crate diesel;
extern crate iron;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate router;

use iron::middleware::*;
use iron::prelude::*;
use iron::status;
use router::Router;

use std::env;

mod auth;
mod db;

use self::db::DieselMiddleware;

pub fn route() -> Chain {
    let router = router!(index: get "/" => handler,
                         query: get "/:query" => handler);
    let mut chain = Chain::new(router);
    let auth = auth::Auth {};
    chain.link_around(auth);
    let mut chain = Chain::new(chain);
    chain.link_before(
        DieselMiddleware::new(&env::var("DATABASE_URL").unwrap_or_else(|_| "chatelaine.db".to_owned()))
            .unwrap(),
    );
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
