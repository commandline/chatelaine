use diesel::prelude::*;
use iron::headers::{Authorization, Basic};
use iron::middleware::*;
use iron::prelude::*;
use iron::status;

use db::DieselReqExt;
use db::models::Credentials;

pub struct Auth;

pub struct AuthHandler<H: Handler> {
    handler: H,
}

impl<H: Handler> Handler for AuthHandler<H> {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        if self.authorize(req) {
            self.handler.handle(req)
        } else {
            Ok(unauthorized())
        }
    }
}

impl<H: Handler> AuthHandler<H> {
    fn authorize(&self, req: &Request) -> bool {
        use db::schema::credentials::dsl::*;

        if let Some(&Authorization(ref basic)) = req.headers.get::<Authorization<Basic>>() {
            let conn = req.db_conn();
            if let Ok(results) = credentials
                .filter(username.eq(&basic.username))
                .limit(1)
                .load::<Credentials>(&*conn)
            {
                if results.len() == 1 {
                    Some(&results[0].password) == basic.password.as_ref()
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn unauthorized() -> Response {
    Response::with((status::Unauthorized, "unauthorized"))
}

impl AroundMiddleware for Auth {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(AuthHandler { handler }) as Box<Handler>
    }
}

#[cfg(test)]
mod tests {
    extern crate iron_test;

    use diesel;
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use iron::Headers;
    use iron::headers::{Authorization, Basic};
    use self::iron_test::{request, response};

    use std::env;

    use db::models::NewCredentials;
    use db::schema::credentials;
    use route;

    use super::*;

    #[test]
    fn no_auth() {
        setup_creds(NewCredentials {
            username: "admin".to_owned(),
            password: "admin".to_owned(),
        });
        let response =
            request::get("http://localhost:3000/hello", Headers::new(), &route()).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }

    #[test]
    fn with_valid_auth() {
        setup_creds(NewCredentials {
            username: "admin".to_owned(),
            password: "admin".to_owned(),
        });
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: Some("admin".to_owned()),
        }));

        let response = request::get("http://localhost:3000/hello", headers, &route()).unwrap();

        assert_eq!(Some(status::Ok), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"hello");
    }

    #[test]
    fn with_missing_auth() {
        setup_creds(NewCredentials {
            username: "admin".to_owned(),
            password: "admin".to_owned(),
        });
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "user".to_owned(),
            password: Some("user".to_owned()),
        }));

        let response = request::get("http://localhost:3000/hello", headers, &route()).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }

    #[test]
    fn with_invalid_auth() {
        setup_creds(NewCredentials {
            username: "admin".to_owned(),
            password: "admin".to_owned(),
        });
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: Some("user".to_owned()),
        }));

        let response = request::get("http://localhost:3000/hello", headers, &route()).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }

    fn setup_creds(to_insert: NewCredentials) {
        use db::schema::credentials::dsl::*;
        env::set_var("DATABASE_URL", ".test.db");
        let conn = SqliteConnection::establish(".test.db")
            .expect("Couldn't connect to SQLite test database");
        if let Ok(results) = credentials
            .filter(username.eq(&to_insert.username))
            .limit(1)
            .load::<Credentials>(&conn)
        {
            if results.len() == 0 {
                insert_creds(&conn, to_insert);
            }
        }
    }

    fn insert_creds(conn: &SqliteConnection, to_insert: NewCredentials) {
        diesel::insert_into(credentials::table)
            .values(&to_insert)
            .execute(conn)
            .expect("Couldn't insert credentials");
    }
}
