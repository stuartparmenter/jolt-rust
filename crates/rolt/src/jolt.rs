//! Direct declarations of native Jolt methods. These are maintained as Rust source;
//! cxx checks their signatures against the included Jolt headers on every build.
#![allow(non_snake_case)]
use crate::raw::{JPC_Quat, JPC_Vec3};
use std::pin::Pin;

// The public API retains raw handles; constructing a pinned receiver requires
// the same validity/exclusive-access obligations as the underlying native call.
pub(crate) unsafe fn pin<'a, T>(p: *mut T) -> Pin<&'a mut T> {
    Pin::new_unchecked(&mut *p)
}

// Native value types have explicit representations, checked in native/jolt.h.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BodyID(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EActivation(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EPhysicsUpdateError(pub u32);
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
macro_rules! native_value {
    ($ty:ident,$id:literal) => {
        unsafe impl cxx::ExternType for $ty {
            type Id = cxx::type_id!($id);
            type Kind = cxx::kind::Trivial;
        }
    };
}
native_value!(BodyID, "JPH::BodyID");
native_value!(EActivation, "JPH::EActivation");
native_value!(EPhysicsUpdateError, "JPH::EPhysicsUpdateError");
native_value!(Vec3, "JPH::Vec3");
native_value!(Quat, "JPH::Quat");
impl From<JPC_Vec3> for Vec3 {
    fn from(v: JPC_Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.z,
        }
    }
}
impl From<Vec3> for JPC_Vec3 {
    fn from(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            _w: v.z,
        }
    }
}
impl From<JPC_Quat> for Quat {
    fn from(v: JPC_Quat) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

// cxx emits public inherent methods on the exported raw aliases, even when the
// bridge is private. Keep these unsafe: copied opaque handles are not native
// objects, and receivers/pointers must satisfy each Jolt method's requirements.
#[cxx::bridge(namespace = "JPH")]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("jolt.h");
        include!("bridge.h");
        type Body = crate::raw::JPC_Body;
        type TempAllocator;
        type TempAllocatorImpl = crate::raw::JPC_TempAllocatorImpl;
        type JobSystem = crate::raw::JPC_JobSystem;
        type BodyInterface = crate::raw::JPC_BodyInterface;
        type BodyCreationSettings;
        #[namespace = "compat"]
        #[cxx_name = "JPC_BodyCreationSettings"]
        type PublicBodyCreationSettings = crate::raw::JPC_BodyCreationSettings;
        #[namespace = "compat"]
        unsafe fn native_body_settings(
            settings: &PublicBodyCreationSettings,
        ) -> Result<UniquePtr<BodyCreationSettings>>;
        unsafe fn CreateBody(
            self: Pin<&mut BodyInterface>,
            settings: &BodyCreationSettings,
        ) -> *mut Body;
        type PhysicsSystem = crate::raw::JPC_PhysicsSystem;
        type Shape = crate::raw::JPC_Shape;
        type CastShapeCollector = crate::raw::JPC_CastShapeCollector;
        type CollideShapeCollector = crate::raw::JPC_CollideShapeCollector;
        type NarrowPhaseQuery = crate::raw::JPC_NarrowPhaseQuery;
        type Constraint = crate::raw::JPC_Constraint;
        type BroadPhaseLayerInterface = crate::raw::JPC_BroadPhaseLayerInterface;
        type ObjectVsBroadPhaseLayerFilter = crate::raw::JPC_ObjectVsBroadPhaseLayerFilter;
        type ObjectLayerPairFilter = crate::raw::JPC_ObjectLayerPairFilter;
        type ContactListener = crate::raw::JPC_ContactListener;
        type SimShapeFilter = crate::raw::JPC_SimShapeFilter;
        type BodyID = super::BodyID;
        type EActivation = super::EActivation;
        type EPhysicsUpdateError = super::EPhysicsUpdateError;
        type Vec3 = super::Vec3;
        type Quat = super::Quat;
        unsafe fn RegisterDefaultAllocator();
        unsafe fn RegisterTypes();
        unsafe fn UnregisterTypes();
        unsafe fn GetID(self: &Body) -> &BodyID;
        #[rust_name = "body_user_data"]
        unsafe fn GetUserData(self: &Body) -> u64;
        unsafe fn AddRef(self: &Shape);
        unsafe fn Release(self: &Shape);
        unsafe fn Init(
            self: Pin<&mut PhysicsSystem>,
            bodies: u32,
            mutexes: u32,
            pairs: u32,
            contacts: u32,
            layers: &BroadPhaseLayerInterface,
            vs: &ObjectVsBroadPhaseLayerFilter,
            pair_filter: &ObjectLayerPairFilter,
        );
        unsafe fn SetContactListener(self: Pin<&mut PhysicsSystem>, listener: *mut ContactListener);
        unsafe fn SetSimShapeFilter(self: Pin<&mut PhysicsSystem>, filter: *const SimShapeFilter);
        unsafe fn OptimizeBroadPhase(self: Pin<&mut PhysicsSystem>);
        unsafe fn Update(
            self: Pin<&mut PhysicsSystem>,
            dt: f32,
            steps: i32,
            temp: *mut TempAllocator,
            jobs: *mut JobSystem,
        ) -> EPhysicsUpdateError;
        #[namespace = "rolt_native"]
        unsafe fn allocator_base(p: *mut TempAllocatorImpl) -> *mut TempAllocator;
        unsafe fn AddConstraint(self: Pin<&mut PhysicsSystem>, constraint: *mut Constraint);
        unsafe fn RemoveConstraint(self: Pin<&mut PhysicsSystem>, constraint: *mut Constraint);
        unsafe fn GetBodyInterface(self: Pin<&mut PhysicsSystem>) -> Pin<&mut BodyInterface>;
        unsafe fn GetBodyInterfaceNoLock(self: Pin<&mut PhysicsSystem>) -> Pin<&mut BodyInterface>;
        unsafe fn GetNarrowPhaseQuery(self: &PhysicsSystem) -> &NarrowPhaseQuery;
        unsafe fn AddBody(self: Pin<&mut BodyInterface>, id: &BodyID, activation: EActivation);
        unsafe fn ActivateBody(self: Pin<&mut BodyInterface>, id: &BodyID);
        unsafe fn ActivateBodies(self: Pin<&mut BodyInterface>, ids: *const BodyID, count: i32);
        unsafe fn RemoveBody(self: Pin<&mut BodyInterface>, id: &BodyID);
        unsafe fn DestroyBody(self: Pin<&mut BodyInterface>, id: &BodyID);
        unsafe fn IsActive(self: &BodyInterface, id: &BodyID) -> bool;
        unsafe fn GetUserData(self: &BodyInterface, id: &BodyID) -> u64;
        unsafe fn SetUserData(self: &BodyInterface, id: &BodyID, data: u64);
        unsafe fn GetCenterOfMassPosition(self: &BodyInterface, id: &BodyID) -> Vec3;
        unsafe fn GetLinearVelocity(self: &BodyInterface, id: &BodyID) -> Vec3;
        unsafe fn SetLinearVelocity(self: Pin<&mut BodyInterface>, id: &BodyID, velocity: Vec3);
        unsafe fn SetObjectLayer(self: Pin<&mut BodyInterface>, id: &BodyID, layer: u16);
        unsafe fn SetShape(
            self: &BodyInterface,
            id: &BodyID,
            shape: *const Shape,
            update_mass: bool,
            activation: EActivation,
        );
        unsafe fn NotifyShapeChanged(
            self: &BodyInterface,
            id: &BodyID,
            old_com: Vec3,
            update_mass: bool,
            activation: EActivation,
        );
        unsafe fn SetPositionAndRotationWhenChanged(
            self: Pin<&mut BodyInterface>,
            id: &BodyID,
            position: Vec3,
            rotation: Quat,
            activation: EActivation,
        );
        #[rust_name = "cast_early_out"]
        unsafe fn UpdateEarlyOutFraction(self: Pin<&mut CastShapeCollector>, fraction: f32);
        #[rust_name = "collide_early_out"]
        unsafe fn UpdateEarlyOutFraction(self: Pin<&mut CollideShapeCollector>, fraction: f32);

    }
}

impl From<glam::Vec3> for Vec3 {
    fn from(v: glam::Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.z,
        }
    }
}
impl From<Vec3> for glam::Vec3 {
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
impl From<glam::Quat> for Quat {
    fn from(v: glam::Quat) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}
