//! Rust wrapper around Jolt Physics 5.6.1, connected directly through a private cxx bridge.
//!
//! The existing rolt API is retained. Low-level settings and raw operations are
//! available in [`raw`]; no separate sys crate or JoltC library is required.

mod jolt;
/// Low-level settings, result types, and unsafe operations used by the rolt API.
///
/// These retain the historical `JPC_*` names for compatibility. The implementation
/// calls Jolt C++ through a private cxx bridge, not through JoltC.
pub mod raw;

use crate::raw::*;

mod body;
mod body_interface;
mod conversions;
mod math;
mod narrow_phase;
mod physics_system;
mod reference;
mod remote_drop;
mod simple_types;
mod traits;

pub use crate::body::*;
pub use crate::body_interface::*;
pub use crate::conversions::*;
pub use crate::math::*;
pub use crate::narrow_phase::*;
pub use crate::physics_system::*;
pub use crate::reference::*;
pub use crate::simple_types::*;
pub use crate::traits::*;

/// [`JPH::RegisterDefaultAllocator`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/_memory_8h.html#a6ae804b1b68490f6e032ef6e7d9fc93e)
pub fn register_default_allocator() {
    unsafe { crate::jolt::ffi::RegisterDefaultAllocator() }
}

/// Creates a new global factory. Required for initialization and used by Jolt's
/// serialization.
///
/// See also: Jolt's [`Factory`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_factory.html) class.
pub fn factory_init() {
    unsafe {
        JPC_FactoryInit();
    }
}

/// Deletes the globally registered factory.
///
/// See also: Jolt's [`Factory`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_factory.html) class.
pub fn factory_delete() {
    unsafe {
        JPC_FactoryDelete();
    }
}

/// [`JPH::RegisterTypes`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/_register_types_8h.html#a033e662bc8b7d5a8acd9adcc692b7cb4)
pub fn register_types() {
    unsafe { crate::jolt::ffi::RegisterTypes() }
}

/// [`JPH::UnregisterTypes`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/_register_types_8h.html#a1e0db6031789e773039c7fc15ef47057)
pub fn unregister_types() {
    unsafe { crate::jolt::ffi::UnregisterTypes() }
}
