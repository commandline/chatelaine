use diesel::sqlite::SqliteConnection;
use iron::prelude::*;
use iron::{typemap, BeforeMiddleware};
use r2d2;
use r2d2_diesel;
use std::error::Error;
use std::sync::Arc;

pub mod schema;
pub mod models;

/// Convenience type for working with an r2d2 pool of diesel connections to sqlite.
type DieselPool = Arc<r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>>;

/// Middleware that will add a `db_conn()` method to requests.
pub struct DieselMiddleware {
    pub pool: DieselPool,
}

/// Tuple struct used with the request extension.
pub struct Value(DieselPool);

/// Key required for the request extension.
impl typemap::Key for DieselMiddleware {
    type Value = Value;
}

impl DieselMiddleware {
    pub fn new(connection_str: &str) -> Result<DieselMiddleware, Box<Error>> {
        let manager = r2d2_diesel::ConnectionManager::<SqliteConnection>::new(connection_str);
        let pool = try!(r2d2::Pool::builder().build(manager));

        Ok(DieselMiddleware {
            pool: Arc::new(pool),
        })
    }
}

impl BeforeMiddleware for DieselMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions
            .insert::<DieselMiddleware>(Value(Arc::clone(&self.pool)));
        Ok(())
    }
}

pub trait DieselReqExt {
    /// Returns a pooled connection to the sqlite database. The connection is returned to
    /// the pool when the pooled connection is dropped.
    ///
    /// **Panics** if a `DieselMiddleware` has not been registered with Iron, or if retrieving
    /// a connection to the database times out.
    fn db_conn(&self) -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<SqliteConnection>>;
}

impl<'a, 'b> DieselReqExt for Request<'a, 'b> {
    fn db_conn(&self) -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<SqliteConnection>> {
        let poll_value = self.extensions.get::<DieselMiddleware>().unwrap();
        let &Value(ref poll) = poll_value;

        poll.get().unwrap()
    }
}
