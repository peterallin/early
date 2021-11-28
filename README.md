# early
![CI](https://github.com/peterallin/early/actions/workflows/ci.yml/badge.svg)
[![MIT/Apache-2 licensed](https://img.shields.io/github/license/peterallin/early)](./LICENSE-APACHE)

A Rust crate for generating families of URLs. The major use case is to
generate the URLs used to interact with an API, where a small part of
the path is changed but hostname, port number and the start of the path
is the same.

## Example

```rust
use early::Early;
let base = Early::new("https", "example.com")
    .path("api")
    .query("api-version", "42")
    .port(8080);
let people_url = base.clone().path("people").build();
let machines_url = base.path("machines").build();
assert_eq!(people_url, "https://example.com:8080/api/people?api-version=42");
assert_eq!(machines_url, "https://example.com:8080/api/machines?api-version=42");
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.