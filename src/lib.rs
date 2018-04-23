extern crate nickel;
use nickel::{MiddlewareResult, Request, Response};

pub fn enable_cors<'mw>(_: &mut Request, mut resp: Response<'mw>) -> MiddlewareResult<'mw> {
    resp.headers_mut()
        .set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    resp.headers_mut()
        .set_raw("Access-Control-Allow-Methods", vec![b"*".to_vec()]);
    resp.headers_mut().set_raw(
        "Access-Control-Allow-Headers",
        vec![b"Origin, X-Requested-With, Content-Type, Accept".to_vec()],
    );
    resp.next_middleware()
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
