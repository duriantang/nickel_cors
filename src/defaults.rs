pub static DEFAULT_ALLOW_METHODS: &str = "*";
pub static DEFAULT_ALLOW_ORIGIN: &str = "*";
pub static DEFAULT_ALLOW_HEADERS: &str = "Origin, X-Requested-With, Content-Type, Accept";
pub static DEFAULT_MAX_AGE: &str = "86400";
pub static DEFAULT_ALLOW_CREDENTIALS: &str = "";

pub static ALLOW_METHODS_HEADER: &str = "Access-Control-Allow-Methods";
pub static ALLOW_ORIGIN_HEADER: &str = "Access-Control-Allow-Origin";
pub static ALLOW_HEADERS_HEADER: &str = "Access-Control-Allow-Headers";
pub static MAX_AGE_HADER: &str = "Access-Control-Max-Age";
pub static ALLOW_CREDENTIALS_HADER: &str = "Access-Control-Allow-Credentials";

pub static NICKEL_ORS_ALLOW_CREDENTIALS: &str = "NICKEL_CORS_ALLOW_CREDENTIALS";
pub static NICKEL_ORS_ALLOW_ORIGIN: &str = "NICKEL_CORS_ALLOW_ORIGIN";
pub static NICKEL_ORS_ALLOW_METHODS: &str = "NICKEL_CORS_ALLOW_METHODS";
pub static NICKEL_ORS_ALLOW_HEADERS: &str = "NICKEL_CORS_ALLOW_HEADERS";
pub static NICKEL_ORS_MAX_AGE: &str = "NICKEL_CORS_MAX_AGE";
