#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
mod types;
pub use bridge::ffi::{
    JPC_BodyCreationSettings, JPC_BodyManager_DrawSettings, JPC_BoxShapeSettings,
    JPC_CapsuleShapeSettings, JPC_CollideShapeResult, JPC_CollideShapeSettings,
    JPC_CollisionEstimationResult, JPC_CollisionGroup, JPC_Color, JPC_ConstraintSettings,
    JPC_ContactManifold, JPC_ContactPoints, JPC_ContactSettings, JPC_ConvexHullShapeSettings,
    JPC_CylinderShapeSettings, JPC_DMat44, JPC_DVec3, JPC_DistanceConstraintSettings,
    JPC_FixedConstraintSettings, JPC_Float3, JPC_HeightFieldShapeSettings,
    JPC_HingeConstraintSettings, JPC_IndexedTriangle, JPC_IndexedTriangleNoMaterial,
    JPC_MassProperties, JPC_Mat44, JPC_MeshShapeSettings, JPC_MotorSettings,
    JPC_MutableCompoundShapeSettings, JPC_NarrowPhaseQuery_CastRayArgs,
    JPC_NarrowPhaseQuery_CastShapeArgs, JPC_NarrowPhaseQuery_CollideShapeArgs,
    JPC_PlaneShapeSettings, JPC_Quat, JPC_RRayCast, JPC_RShapeCast, JPC_RayCast, JPC_RayCastResult,
    JPC_ShapeCastResult, JPC_ShapeCastSettings, JPC_SixDOFConstraintSettings,
    JPC_SliderConstraintSettings, JPC_SphereShapeSettings, JPC_SpringSettings,
    JPC_StaticCompoundShapeSettings, JPC_SubShapeIDPair, JPC_SubShapeSettings,
    JPC_TriangleShapeSettings, JPC_Vec2, JPC_Vec3, JPC_Vec4,
};
pub use types::*;
mod bridge;
mod forward;
pub use forward::*;
#[cfg(feature = "double-precision")]
pub type Real = f64;

#[cfg(not(feature = "double-precision"))]
pub type Real = f32;

macro_rules! ffi_default {
    ($($c_struct:ident -> $default_fn:ident,)*) => {
        $(
            impl Default for $c_struct {
                fn default() -> Self {
                    unsafe {
                        let mut settings = std::mem::MaybeUninit::<$c_struct>::zeroed();
                        $default_fn(settings.as_mut_ptr());
                        settings.assume_init()
                    }
                }
            }
        )*
    };
}

ffi_default! {
    JPC_MassProperties -> JPC_MassProperties_default,
    JPC_BodyCreationSettings -> JPC_BodyCreationSettings_default,
    JPC_ShapeCastSettings -> JPC_ShapeCastSettings_default,
    JPC_CollideShapeSettings -> JPC_CollideShapeSettings_default,

    // All of the ShapeSettings types
    JPC_TriangleShapeSettings -> JPC_TriangleShapeSettings_default,
    JPC_MeshShapeSettings -> JPC_MeshShapeSettings_default,
    JPC_HeightFieldShapeSettings -> JPC_HeightFieldShapeSettings_default,
    JPC_BoxShapeSettings -> JPC_BoxShapeSettings_default,
    JPC_SphereShapeSettings -> JPC_SphereShapeSettings_default,
    JPC_CapsuleShapeSettings -> JPC_CapsuleShapeSettings_default,
    JPC_CylinderShapeSettings -> JPC_CylinderShapeSettings_default,
    JPC_PlaneShapeSettings -> JPC_PlaneShapeSettings_default,
    JPC_ConvexHullShapeSettings -> JPC_ConvexHullShapeSettings_default,
    JPC_SubShapeSettings -> JPC_SubShapeSettings_default,
    JPC_StaticCompoundShapeSettings -> JPC_StaticCompoundShapeSettings_default,
    JPC_MutableCompoundShapeSettings -> JPC_MutableCompoundShapeSettings_default,
}
