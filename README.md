# rolt with cxx

This branch preserves rolt's methods and traits while replacing JoltC and
`joltc-sys` with a private `#[cxx::bridge]` to Jolt 5.6.1. Public settings and
callback types move from `joltc_sys` to `rolt::raw`; their existing names remain.
Heightfield creation is included, with a smoke test that settles a sphere on it.

Requires Rust, a C++20 compiler, CMake 3.22+, Make and Git. Currently validated
only on x86_64 Linux with single precision and u16 object layers; other
configurations are rejected explicitly.

```sh
cargo xtask setup
cargo test --workspace
cargo run -p hello-world
```

Start reviewing at `crates/rolt/src/jolt.rs`: handwritten cxx declarations of
actual Jolt methods. `native/callbacks.cpp` implements virtual callbacks;
`native/shims.cpp` handles constructors and queries cxx cannot bind directly.
Public settings and value records are declared in `crates/rolt/src/raw/bridge.rs`
using cxx shared structs. cxx generates their Rust/C++ definitions and call glue.
`native/settings.inc` converts these records to native Jolt settings; callback
function-pointer tables remain in `raw/types.rs` and `native/types.h`.
The former bindgen-only zero-length padding fields are omitted from shared records.

No Python, bindgen, JoltC build, or separate sys crate is needed. Jolt source is
fetched at the pinned revision in `upstream.toml` and is not committed.
This is an experimental backend, not full coverage of the former raw sys API.
Constraint construction, characters, vehicles and soft bodies are not included.
The inherited CI matrix has not been adapted to this prototype.

## License
Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[JoltC]: https://github.com/SecondHalfGames/JoltC
[HelloWorld]: https://github.com/jrouwe/JoltPhysics/blob/master/HelloWorld/HelloWorld.cpp
