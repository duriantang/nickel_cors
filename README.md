# **nickel_cors** is a middleware for [nickel.rs][]

for more tech detail, see [MDN CORS][]

[nickel.rs]: https://github.com/nickel-org/nickel.rs "nickel.rs - web application framework for rust"
[MDN CORS]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS "Cross-Origin Resource Sharing (CORS) - HTTP | MDN"

## Install

Add this line to your ``Cargo.toml``

```toml
nickel_cors = "0.2.0"
```

## Usage

It's simple.

```rust
/* get server instance */
extern crate nickel;
use nickel::Nickel;
let mut server = Nickel::new();

/* enable cors */
extern crate nickel_cors;
server.utilize(nickel_cors::enable_cors);

```

This middleware will add these CORS headers to your every response:

* ``Access-Control-Allow-Methods: *``

* ``Access-Control-Allow-Origin: *``

* ``Access-Control-Allow-Headers: Origin, X-Requested-With, Content-Type, Accept``

* ``Access-Control-Max-Age: 86400``

<!-- * ``Access-Control-Allow-Credentials``

    default: *not set* -->

## Dev & Test

first you need install or switch to **stable** rust version.

```shell
$ rustup default stable
```

runing test use under line, or it will be failed.

```sh
cargo test -- --test-threads=1
```
