# **nickel_cors** is a middleware for [nickel.rs]

[nickel.rs]: https://github.com/nickel-org/nickel.rs "nickel.rs - web application framework for rust"
[mdn cors]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS "Cross-Origin Resource Sharing (CORS) - HTTP | MDN"
[language]: https://img.shields.io/badge/language-rust-blue.svg
[language website]: https://www.rust-lang.org/ "The Rust Programming Language"
[license]: https://img.shields.io/github/license/duriantang/nickel_cors.svg
[mit]: https://raw.githubusercontent.com/duriantang/nickel_cors/master/LICENSE "MIT"
[ci]: https://img.shields.io/travis/duriantang/nickel_cors.svg
[travis-ci]: https://travis-ci.org/duriantang/nickel_cors "Travis CI"
[cov]: https://img.shields.io/codecov/c/github/duriantang/nickel_cors.svg
[codecov]: https://codecov.io/gh/duriantang/nickel_cors "Codecov.io"
[libs]: https://img.shields.io/librariesio/github/duriantang/nickel_cors.svg
[libraries.io]: https://libraries.io/github/duriantang/nickel_cors "Libraries.io for GitHub"
[crates version]: https://img.shields.io/crates/v/nickel_cors.svg
[crates.io]: https://crates.io/crates/nickel_cors "nickel_cors - Cargo: packages for Rust"

[![language]][language website]
[![license]][mit]
[![ci]][travis-ci]
[![cov]][codecov]
[![crates version]][crates.io]
[![libs]][libraries.io]

for more tech detail, see [MDN CORS]

## Install

Add this line to your `Cargo.toml`

```toml
nickel_cors = "0.3.3"
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

- `Access-Control-Allow-Methods: *`

- `Access-Control-Allow-Origin: *`

- `Access-Control-Allow-Headers: Origin, X-Requested-With, Content-Type, Accept`

- `Access-Control-Max-Age: 86400`

<!-- * ``Access-Control-Allow-Credentials``

    default: *not set* -->

## Dev & Test

first you need install or switch to **stable** rust version.

```shell
rustup default stable
```

runing test use under line, or it will be failed.

```sh
cargo test
```
