[![crates.io](https://img.shields.io/crates/v/mime_typed.svg)][`mime_typed`]
[![crates.io](https://img.shields.io/crates/d/mime_typed.svg)][`mime_typed`]

# More types for [`mime`]

## What is [`mime`] library?

At the moment of writing, [`mime`] is the most widely used [MIME types] library on [crates.io] with over 39.4M all-time downloads and over 5.5M recent downloads.

Its notable dependents are

* [`reqwest`]
* [`mime_guess`]
* [`headers`]
* [`actix-web`]
* [`serde_with`]
* [`actix-http`]
* [`multipart`]
* [`warp`]
* [`awc`]
* [`tower-http`]

Even though [`mime`]

1. doesn't have a README.md on [crates.io];
2. hasn't been updated for two years
3. doesn't support all [MIME types] listed in [RFC 6838](https://datatracker.ietf.org/doc/html/rfc6838) by IETF;
4. has last **published** version at <https://github.com/hyperium/mime/tree/e3e7444ca607ff87cd1475455c26876b936af77a> instead of latest commit;

it is nonetheless a reasonably well documented library that has been proven to be well suitable for many projects by its use.

## What does [`mime_typed`] bring to the table?

This library allows writing code generic over [MIME types] easier and has **opt-in** support for [`mime`]. With `mime_support` feature enabled, traits obtain new functions for returning instances of [`mime`] library types.

With `evcxr_support` feature, the crate also offers support for [`evcxr`] Rust kernel for Jupyter Notebook.

The types offered by library are zero-sized.

Opt-in support for other projects may be considered as well.

[`mime_typed`]: https://crates.io/crates/mime_typed
[`mime`]: https://crates.io/crates/mime
[`reqwest`]: https://crates.io/crates/reqwest
[`mime_guess`]: https://crates.io/crates/mime_guess
[`headers`]: https://crates.io/crates/headers
[`actix-web`]: https://crates.io/crates/actix-web
[`serde_with`]: https://crates.io/crates/serde_with
[`actix-http`]: https://crates.io/crates/actix-http
[`multipart`]: https://crates.io/crates/multipart
[`warp`]: https://crates.io/crates/warp
[`awc`]: https://crates.io/crates/awc
[`tower-http`]: https://crates.io/crates/tower-http
[`evcxr`]: https://github.com/google/evcxr

[MIME types]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
[crates.io]: crates.io

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>