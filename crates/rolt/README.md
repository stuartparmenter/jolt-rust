# rolt

Existing rolt methods and traits over a private cxx bridge to Jolt Physics 5.6.1.
No separate sys crate or JoltC library is used. Low-level settings, results and
unsafe operations are exposed in `rolt::raw` with their existing `JPC_*` names.

Use `asserts` to enable native Jolt assertions. The `double-precision` and
`object-layer-u32` feature names are retained but currently rejected by this
prototype. See the workspace README for build
instructions.
