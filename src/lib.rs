extern crate nickel;

use std::env;
use nickel::{MiddlewareResult, Request, Response};

mod defaults;
use defaults::*;

pub fn enable_cors<'mw>(_: &mut Request, mut resp: Response<'mw>) -> MiddlewareResult<'mw> {
    let allow_credentials =
        env::var(NICKEL_ORS_ALLOW_CREDENTIALS).unwrap_or(String::from(DEFAULT_ALLOW_CREDENTIALS));
    if !allow_credentials.is_empty() && allow_credentials == "true" {
        resp.headers_mut()
            .set_raw(ALLOW_CREDENTIALS_HADER, vec![b"true".to_vec()])
    };

    let allow_origin =
        env::var(NICKEL_ORS_ALLOW_ORIGIN).unwrap_or(String::from(DEFAULT_ALLOW_ORIGIN));
    resp.headers_mut()
        .set_raw(ALLOW_ORIGIN_HEADER, vec![allow_origin.as_bytes().to_vec()]);

    let allow_methods =
        env::var(NICKEL_ORS_ALLOW_METHODS).unwrap_or(String::from(DEFAULT_ALLOW_METHODS));
    resp.headers_mut().set_raw(
        ALLOW_METHODS_HEADER,
        vec![allow_methods.as_bytes().to_vec()],
    );

    let allow_headers =
        env::var(NICKEL_ORS_ALLOW_HEADERS).unwrap_or(String::from(DEFAULT_ALLOW_HEADERS));
    resp.headers_mut().set_raw(
        ALLOW_HEADERS_HEADER,
        vec![allow_headers.as_bytes().to_vec()],
    );

    let max_age = env::var(NICKEL_ORS_MAX_AGE).unwrap_or(String::from(DEFAULT_MAX_AGE));
    resp.headers_mut()
        .set_raw(MAX_AGE_HADER, vec![max_age.as_bytes().to_vec()]);

    resp.next_middleware()
}

#[cfg(test)]
mod tests {
    extern crate nickel;
    extern crate reqwest;

    use nickel::Nickel;
    use self::reqwest::{Client, Method};
    use std::env;
    use std::thread;
    use super::defaults::*;
    use super::enable_cors;

    #[test]
    fn test_defaults_works() {
        static TEST_URL: &str = "127.0.0.1:55555";
        let mut server = Nickel::new();
        server.utilize(enable_cors);
        let mut children = vec![];
        children.push(thread::spawn(move || {
            server.listen(TEST_URL).unwrap();
        }));
        let client = Client::new();
        let url: &str = &format!("http://{}", TEST_URL);
        let resp = client.request(Method::Options, url).send();
        let resp_ = resp.unwrap();
        let resp_headers = resp_.headers();
        assert_eq!(
            resp_headers.get_raw(ALLOW_HEADERS_HEADER).unwrap(),
            DEFAULT_ALLOW_HEADERS
        );
        assert_eq!(
            resp_headers.get_raw(ALLOW_METHODS_HEADER).unwrap(),
            DEFAULT_ALLOW_METHODS
        );
        assert_eq!(
            resp_headers.get_raw(ALLOW_ORIGIN_HEADER).unwrap(),
            DEFAULT_ALLOW_ORIGIN
        );
        assert_eq!(
            resp_headers.get_raw(MAX_AGE_HADER).unwrap(),
            DEFAULT_MAX_AGE
        );
        assert_eq!(resp_headers.get_raw(ALLOW_CREDENTIALS_HADER), None);
    }

    #[test]
    fn test_set_env_works() {
        static TEST_URL: &str = "127.0.0.1:55556";
        env::set_var(NICKEL_ORS_ALLOW_CREDENTIALS, "true");
        env::set_var(NICKEL_ORS_ALLOW_HEADERS, "X-POWERBY");
        env::set_var(NICKEL_ORS_MAX_AGE, "60");
        env::set_var(NICKEL_ORS_ALLOW_METHODS, "GET, POST");
        env::set_var(NICKEL_ORS_ALLOW_ORIGIN, "https://rust-lang.org");

        let mut server = Nickel::new();
        server.utilize(enable_cors);
        let mut children = vec![];
        children.push(thread::spawn(move || {
            server.listen(TEST_URL).unwrap();
        }));
        let client = Client::new();
        let url: &str = &format!("http://{}", TEST_URL);
        let resp = client.request(Method::Options, url).send();
        let resp_ = resp.unwrap();
        let resp_headers = resp_.headers();
        assert_eq!(
            resp_headers.get_raw(ALLOW_HEADERS_HEADER).unwrap(),
            "X-POWERBY"
        );
        assert_eq!(
            resp_headers.get_raw(ALLOW_METHODS_HEADER).unwrap(),
            "GET, POST"
        );
        assert_eq!(
            resp_headers.get_raw(ALLOW_ORIGIN_HEADER).unwrap(),
            "https://rust-lang.org"
        );
        assert_eq!(resp_headers.get_raw(MAX_AGE_HADER).unwrap(), "60");
        assert_eq!(
            resp_headers.get_raw(ALLOW_CREDENTIALS_HADER).unwrap(),
            "true"
        );
    }
}
