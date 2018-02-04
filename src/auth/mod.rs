use iron::headers::{Authorization, Basic};
use iron::middleware::*;
use iron::prelude::*;
use iron::status;

use std::collections::HashMap;

pub struct Auth {
    pub credentials: HashMap<String, String>,
}

pub struct AuthHandler<H: Handler> {
    credentials: HashMap<String, String>,
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
        if let Some(&Authorization(ref basic)) = req.headers.get::<Authorization<Basic>>() {
            self.credentials.get(&basic.username) == basic.password.as_ref()
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
        Box::new(AuthHandler {
            handler,
            credentials: self.credentials.clone(),
        }) as Box<Handler>
    }
}

#[cfg(test)]
mod tests {
    extern crate iron_test;

    use iron::Headers;
    use iron::headers::{Authorization, Basic};
    use self::iron_test::{request, response};

    use route;

    use super::*;

    #[test]
    fn no_auth() {
        let response = request::get(
            "http://localhost:3000/hello",
            Headers::new(),
            &route(HashMap::new()),
        ).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }

    #[test]
    fn with_valid_auth() {
        let mut credentials = HashMap::new();
        credentials.insert("admin".to_owned(), "admin".to_owned());
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: Some("admin".to_owned()),
        }));

        let response =
            request::get("http://localhost:3000/hello", headers, &route(credentials)).unwrap();

        assert_eq!(Some(status::Ok), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"hello");
    }

    #[test]
    fn with_missing_auth() {
        let mut credentials = HashMap::new();
        credentials.insert("admin".to_owned(), "admin".to_owned());
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "user".to_owned(),
            password: Some("user".to_owned()),
        }));

        let response =
            request::get("http://localhost:3000/hello", headers, &route(credentials)).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }

    #[test]
    fn with_invalid_auth() {
        let mut credentials = HashMap::new();
        credentials.insert("admin".to_owned(), "admin".to_owned());
        let mut headers = Headers::new();
        headers.set(Authorization(Basic {
            username: "admin".to_owned(),
            password: Some("user".to_owned()),
        }));

        let response =
            request::get("http://localhost:3000/hello", headers, &route(credentials)).unwrap();

        assert_eq!(Some(status::Unauthorized), response.status);

        let result_body = response::extract_body_to_bytes(response);
        assert_eq!(result_body, b"unauthorized");
    }
}
