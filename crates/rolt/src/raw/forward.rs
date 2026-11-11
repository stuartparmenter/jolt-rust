// Preserve the raw call signatures used by rolt while handling cxx exceptions.
use super::bridge::ffi;
use crate::raw::*;
pub unsafe fn JPC_BodyCreationSettings_default(settings: *mut JPC_BodyCreationSettings) {
    ffi::JPC_BodyCreationSettings_default(settings)
        .unwrap_or_else(|e| panic!("JPC_BodyCreationSettings_default: {e}"))
}
pub unsafe fn JPC_BodyFilter_delete(object: *mut JPC_BodyFilter) {
    ffi::JPC_BodyFilter_delete(object).unwrap_or_else(|e| panic!("JPC_BodyFilter_delete: {e}"))
}
pub unsafe fn JPC_BodyFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_BodyFilterFns,
) -> *mut JPC_BodyFilter {
    ffi::JPC_BodyFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_BodyFilter_new: {e}"))
}
pub unsafe fn JPC_BodyManager_DrawSettings_default(object: *mut JPC_BodyManager_DrawSettings) {
    ffi::JPC_BodyManager_DrawSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_BodyManager_DrawSettings_default: {e}"))
}
pub unsafe fn JPC_BoxShapeSettings_Create(
    self_: *const JPC_BoxShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_BoxShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_BoxShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_BoxShapeSettings_default(object: *mut JPC_BoxShapeSettings) {
    ffi::JPC_BoxShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_BoxShapeSettings_default: {e}"))
}
pub unsafe fn JPC_BroadPhaseLayerFilter_delete(object: *mut JPC_BroadPhaseLayerFilter) {
    ffi::JPC_BroadPhaseLayerFilter_delete(object)
        .unwrap_or_else(|e| panic!("JPC_BroadPhaseLayerFilter_delete: {e}"))
}
pub unsafe fn JPC_BroadPhaseLayerFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_BroadPhaseLayerFilterFns,
) -> *mut JPC_BroadPhaseLayerFilter {
    ffi::JPC_BroadPhaseLayerFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_BroadPhaseLayerFilter_new: {e}"))
}
pub unsafe fn JPC_BroadPhaseLayerInterface_delete(object: *mut JPC_BroadPhaseLayerInterface) {
    ffi::JPC_BroadPhaseLayerInterface_delete(object)
        .unwrap_or_else(|e| panic!("JPC_BroadPhaseLayerInterface_delete: {e}"))
}
pub unsafe fn JPC_BroadPhaseLayerInterface_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_BroadPhaseLayerInterfaceFns,
) -> *mut JPC_BroadPhaseLayerInterface {
    ffi::JPC_BroadPhaseLayerInterface_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_BroadPhaseLayerInterface_new: {e}"))
}
pub unsafe fn JPC_CapsuleShapeSettings_Create(
    self_: *const JPC_CapsuleShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_CapsuleShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_CapsuleShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_CapsuleShapeSettings_default(object: *mut JPC_CapsuleShapeSettings) {
    ffi::JPC_CapsuleShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_CapsuleShapeSettings_default: {e}"))
}
pub unsafe fn JPC_CastShapeCollector_delete(object: *mut JPC_CastShapeCollector) {
    ffi::JPC_CastShapeCollector_delete(object)
        .unwrap_or_else(|e| panic!("JPC_CastShapeCollector_delete: {e}"))
}
pub unsafe fn JPC_CastShapeCollector_new(
    self_: *mut ::std::os::raw::c_void,
    fns: JPC_CastShapeCollectorFns,
) -> *mut JPC_CastShapeCollector {
    ffi::JPC_CastShapeCollector_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_CastShapeCollector_new: {e}"))
}
pub unsafe fn JPC_CollideShapeCollector_delete(object: *mut JPC_CollideShapeCollector) {
    ffi::JPC_CollideShapeCollector_delete(object)
        .unwrap_or_else(|e| panic!("JPC_CollideShapeCollector_delete: {e}"))
}
pub unsafe fn JPC_CollideShapeCollector_new(
    self_: *mut ::std::os::raw::c_void,
    fns: JPC_CollideShapeCollectorFns,
) -> *mut JPC_CollideShapeCollector {
    ffi::JPC_CollideShapeCollector_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_CollideShapeCollector_new: {e}"))
}
pub unsafe fn JPC_CollideShapeSettings_default(object: *mut JPC_CollideShapeSettings) {
    ffi::JPC_CollideShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_CollideShapeSettings_default: {e}"))
}
pub unsafe fn JPC_ContactListener_delete(object: *mut JPC_ContactListener) {
    ffi::JPC_ContactListener_delete(object)
        .unwrap_or_else(|e| panic!("JPC_ContactListener_delete: {e}"))
}
pub unsafe fn JPC_ContactListener_new(
    self_: *mut ::std::os::raw::c_void,
    fns: JPC_ContactListenerFns,
) -> *mut JPC_ContactListener {
    ffi::JPC_ContactListener_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_ContactListener_new: {e}"))
}
pub unsafe fn JPC_ConvexHullShapeSettings_Create(
    self_: *const JPC_ConvexHullShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_ConvexHullShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_ConvexHullShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_ConvexHullShapeSettings_default(object: *mut JPC_ConvexHullShapeSettings) {
    ffi::JPC_ConvexHullShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_ConvexHullShapeSettings_default: {e}"))
}
pub unsafe fn JPC_CylinderShapeSettings_Create(
    self_: *const JPC_CylinderShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_CylinderShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_CylinderShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_CylinderShapeSettings_default(object: *mut JPC_CylinderShapeSettings) {
    ffi::JPC_CylinderShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_CylinderShapeSettings_default: {e}"))
}
pub unsafe fn JPC_DebugRendererSimple_delete(object: *mut JPC_DebugRendererSimple) {
    ffi::JPC_DebugRendererSimple_delete(object)
        .unwrap_or_else(|e| panic!("JPC_DebugRendererSimple_delete: {e}"))
}
pub unsafe fn JPC_DebugRendererSimple_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_DebugRendererSimpleFns,
) -> *mut JPC_DebugRendererSimple {
    ffi::JPC_DebugRendererSimple_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_DebugRendererSimple_new: {e}"))
}
pub unsafe fn JPC_FactoryDelete() {
    ffi::JPC_FactoryDelete().unwrap_or_else(|e| panic!("JPC_FactoryDelete: {e}"))
}
pub unsafe fn JPC_FactoryInit() {
    ffi::JPC_FactoryInit().unwrap_or_else(|e| panic!("JPC_FactoryInit: {e}"))
}
pub unsafe fn JPC_GroupFilter_delete(object: *mut JPC_GroupFilter) {
    ffi::JPC_GroupFilter_delete(object).unwrap_or_else(|e| panic!("JPC_GroupFilter_delete: {e}"))
}
pub unsafe fn JPC_GroupFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_GroupFilterFns,
) -> *mut JPC_GroupFilter {
    ffi::JPC_GroupFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_GroupFilter_new: {e}"))
}
pub unsafe fn JPC_HeightFieldShapeSettings_Create(
    self_: *const JPC_HeightFieldShapeSettings,
    arg1: *mut *mut JPC_Shape,
    arg2: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_HeightFieldShapeSettings_Create(self_, arg1, arg2)
        .unwrap_or_else(|e| panic!("JPC_HeightFieldShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_HeightFieldShapeSettings_default(self_: *mut JPC_HeightFieldShapeSettings) {
    ffi::JPC_HeightFieldShapeSettings_default(self_)
        .unwrap_or_else(|e| panic!("JPC_HeightFieldShapeSettings_default: {e}"))
}
pub unsafe fn JPC_HeightFieldShape_GetNoCollisionValue() -> f32 {
    ffi::JPC_HeightFieldShape_GetNoCollisionValue()
        .unwrap_or_else(|e| panic!("JPC_HeightFieldShape_GetNoCollisionValue: {e}"))
}
pub unsafe fn JPC_JobSystemThreadPool_delete(object: *mut JPC_JobSystemThreadPool) {
    ffi::JPC_JobSystemThreadPool_delete(object)
        .unwrap_or_else(|e| panic!("JPC_JobSystemThreadPool_delete: {e}"))
}
pub unsafe fn JPC_JobSystemThreadPool_new2(
    inMaxJobs: ::std::os::raw::c_uint,
    inMaxBarriers: ::std::os::raw::c_uint,
) -> *mut JPC_JobSystemThreadPool {
    ffi::JPC_JobSystemThreadPool_new2(inMaxJobs, inMaxBarriers)
        .unwrap_or_else(|e| panic!("JPC_JobSystemThreadPool_new2: {e}"))
}
pub unsafe fn JPC_MassProperties_default(properties: *mut JPC_MassProperties) {
    ffi::JPC_MassProperties_default(properties)
        .unwrap_or_else(|e| panic!("JPC_MassProperties_default: {e}"))
}
pub unsafe fn JPC_MeshShapeSettings_Create(
    self_: *const JPC_MeshShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_MeshShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_MeshShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_MeshShapeSettings_default(object: *mut JPC_MeshShapeSettings) {
    ffi::JPC_MeshShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_MeshShapeSettings_default: {e}"))
}
pub unsafe fn JPC_MutableCompoundShapeSettings_Create(
    self_: *const JPC_MutableCompoundShapeSettings,
    outShape: *mut *mut JPC_MutableCompoundShape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_MutableCompoundShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_MutableCompoundShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_MutableCompoundShapeSettings_default(
    object: *mut JPC_MutableCompoundShapeSettings,
) {
    ffi::JPC_MutableCompoundShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_MutableCompoundShapeSettings_default: {e}"))
}
pub unsafe fn JPC_NarrowPhaseQuery_CastRay(
    self_: *const JPC_NarrowPhaseQuery,
    args: *mut JPC_NarrowPhaseQuery_CastRayArgs,
) -> bool {
    ffi::JPC_NarrowPhaseQuery_CastRay(self_, args)
        .unwrap_or_else(|e| panic!("JPC_NarrowPhaseQuery_CastRay: {e}"))
}
pub unsafe fn JPC_NarrowPhaseQuery_CastShape(
    self_: *const JPC_NarrowPhaseQuery,
    args: *mut JPC_NarrowPhaseQuery_CastShapeArgs,
) {
    ffi::JPC_NarrowPhaseQuery_CastShape(self_, args)
        .unwrap_or_else(|e| panic!("JPC_NarrowPhaseQuery_CastShape: {e}"))
}
pub unsafe fn JPC_NarrowPhaseQuery_CollideShape(
    self_: *const JPC_NarrowPhaseQuery,
    args: *mut JPC_NarrowPhaseQuery_CollideShapeArgs,
) {
    ffi::JPC_NarrowPhaseQuery_CollideShape(self_, args)
        .unwrap_or_else(|e| panic!("JPC_NarrowPhaseQuery_CollideShape: {e}"))
}
pub unsafe fn JPC_ObjectLayerFilter_delete(object: *mut JPC_ObjectLayerFilter) {
    ffi::JPC_ObjectLayerFilter_delete(object)
        .unwrap_or_else(|e| panic!("JPC_ObjectLayerFilter_delete: {e}"))
}
pub unsafe fn JPC_ObjectLayerFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_ObjectLayerFilterFns,
) -> *mut JPC_ObjectLayerFilter {
    ffi::JPC_ObjectLayerFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_ObjectLayerFilter_new: {e}"))
}
pub unsafe fn JPC_ObjectLayerPairFilter_delete(object: *mut JPC_ObjectLayerPairFilter) {
    ffi::JPC_ObjectLayerPairFilter_delete(object)
        .unwrap_or_else(|e| panic!("JPC_ObjectLayerPairFilter_delete: {e}"))
}
pub unsafe fn JPC_ObjectLayerPairFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_ObjectLayerPairFilterFns,
) -> *mut JPC_ObjectLayerPairFilter {
    ffi::JPC_ObjectLayerPairFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_ObjectLayerPairFilter_new: {e}"))
}
pub unsafe fn JPC_ObjectVsBroadPhaseLayerFilter_delete(
    object: *mut JPC_ObjectVsBroadPhaseLayerFilter,
) {
    ffi::JPC_ObjectVsBroadPhaseLayerFilter_delete(object)
        .unwrap_or_else(|e| panic!("JPC_ObjectVsBroadPhaseLayerFilter_delete: {e}"))
}
pub unsafe fn JPC_ObjectVsBroadPhaseLayerFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_ObjectVsBroadPhaseLayerFilterFns,
) -> *mut JPC_ObjectVsBroadPhaseLayerFilter {
    ffi::JPC_ObjectVsBroadPhaseLayerFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_ObjectVsBroadPhaseLayerFilter_new: {e}"))
}
pub unsafe fn JPC_PhysicsSystem_DrawBodies(
    self_: *mut JPC_PhysicsSystem,
    inSettings: *mut JPC_BodyManager_DrawSettings,
    inRenderer: *mut JPC_DebugRendererSimple,
    inBodyFilter: *const ::std::os::raw::c_void,
) {
    ffi::JPC_PhysicsSystem_DrawBodies(self_, inSettings, inRenderer, inBodyFilter as usize)
        .unwrap_or_else(|e| panic!("JPC_PhysicsSystem_DrawBodies: {e}"))
}
pub unsafe fn JPC_PhysicsSystem_delete(object: *mut JPC_PhysicsSystem) {
    ffi::JPC_PhysicsSystem_delete(object)
        .unwrap_or_else(|e| panic!("JPC_PhysicsSystem_delete: {e}"))
}
pub unsafe fn JPC_PhysicsSystem_new() -> *mut JPC_PhysicsSystem {
    ffi::JPC_PhysicsSystem_new().unwrap_or_else(|e| panic!("JPC_PhysicsSystem_new: {e}"))
}
pub unsafe fn JPC_PlaneShapeSettings_Create(
    self_: *const JPC_PlaneShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_PlaneShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_PlaneShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_PlaneShapeSettings_default(object: *mut JPC_PlaneShapeSettings) {
    ffi::JPC_PlaneShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_PlaneShapeSettings_default: {e}"))
}
pub unsafe fn JPC_SetAssertFailed(fn_: JPC_AssertFailedFunction) {
    ffi::JPC_SetAssertFailed(fn_.map_or(0, |f| f as usize))
        .unwrap_or_else(|e| panic!("JPC_SetAssertFailed: {e}"))
}
pub unsafe fn JPC_ShapeCastSettings_default(object: *mut JPC_ShapeCastSettings) {
    ffi::JPC_ShapeCastSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_ShapeCastSettings_default: {e}"))
}
pub unsafe fn JPC_ShapeFilter_delete(object: *mut JPC_ShapeFilter) {
    ffi::JPC_ShapeFilter_delete(object).unwrap_or_else(|e| panic!("JPC_ShapeFilter_delete: {e}"))
}
pub unsafe fn JPC_ShapeFilter_getBodyID2(self_: *const JPC_ShapeFilter) -> JPC_BodyID {
    ffi::JPC_ShapeFilter_getBodyID2(self_)
        .unwrap_or_else(|e| panic!("JPC_ShapeFilter_getBodyID2: {e}"))
}
pub unsafe fn JPC_ShapeFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_ShapeFilterFns,
) -> *mut JPC_ShapeFilter {
    ffi::JPC_ShapeFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_ShapeFilter_new: {e}"))
}
pub unsafe fn JPC_SimShapeFilter_delete(object: *mut JPC_SimShapeFilter) {
    ffi::JPC_SimShapeFilter_delete(object)
        .unwrap_or_else(|e| panic!("JPC_SimShapeFilter_delete: {e}"))
}
pub unsafe fn JPC_SimShapeFilter_new(
    self_: *const ::std::os::raw::c_void,
    fns: JPC_SimShapeFilterFns,
) -> *mut JPC_SimShapeFilter {
    ffi::JPC_SimShapeFilter_new(self_ as usize, fns)
        .unwrap_or_else(|e| panic!("JPC_SimShapeFilter_new: {e}"))
}
pub unsafe fn JPC_SphereShapeSettings_Create(
    self_: *const JPC_SphereShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_SphereShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_SphereShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_SphereShapeSettings_default(object: *mut JPC_SphereShapeSettings) {
    ffi::JPC_SphereShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_SphereShapeSettings_default: {e}"))
}
pub unsafe fn JPC_StaticCompoundShapeSettings_Create(
    self_: *const JPC_StaticCompoundShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_StaticCompoundShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_StaticCompoundShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_StaticCompoundShapeSettings_default(
    object: *mut JPC_StaticCompoundShapeSettings,
) {
    ffi::JPC_StaticCompoundShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_StaticCompoundShapeSettings_default: {e}"))
}
pub unsafe fn JPC_String_c_str(self_: *mut JPC_String) -> *const ::std::os::raw::c_char {
    ffi::JPC_String_c_str(self_).unwrap_or_else(|e| panic!("JPC_String_c_str: {e}")) as _
}
pub unsafe fn JPC_String_delete(self_: *mut JPC_String) {
    ffi::JPC_String_delete(self_).unwrap_or_else(|e| panic!("JPC_String_delete: {e}"))
}
pub unsafe fn JPC_SubShapeSettings_default(object: *mut JPC_SubShapeSettings) {
    ffi::JPC_SubShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_SubShapeSettings_default: {e}"))
}
pub unsafe fn JPC_TempAllocatorImpl_delete(object: *mut JPC_TempAllocatorImpl) {
    ffi::JPC_TempAllocatorImpl_delete(object)
        .unwrap_or_else(|e| panic!("JPC_TempAllocatorImpl_delete: {e}"))
}
pub unsafe fn JPC_TempAllocatorImpl_new(
    size: ::std::os::raw::c_uint,
) -> *mut JPC_TempAllocatorImpl {
    ffi::JPC_TempAllocatorImpl_new(size)
        .unwrap_or_else(|e| panic!("JPC_TempAllocatorImpl_new: {e}"))
}
pub unsafe fn JPC_TriangleShapeSettings_Create(
    self_: *const JPC_TriangleShapeSettings,
    outShape: *mut *mut JPC_Shape,
    outError: *mut *mut JPC_String,
) -> bool {
    ffi::JPC_TriangleShapeSettings_Create(self_, outShape, outError)
        .unwrap_or_else(|e| panic!("JPC_TriangleShapeSettings_Create: {e}"))
}
pub unsafe fn JPC_TriangleShapeSettings_default(object: *mut JPC_TriangleShapeSettings) {
    ffi::JPC_TriangleShapeSettings_default(object)
        .unwrap_or_else(|e| panic!("JPC_TriangleShapeSettings_default: {e}"))
}
unsafe impl cxx::ExternType for JPC_Body {
    type Id = cxx::type_id!("JPH::Body");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_BodyFilter {
    type Id = cxx::type_id!("compat::JPC_BodyFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_BodyFilterFns {
    type Id = cxx::type_id!("compat::JPC_BodyFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_BodyInterface {
    type Id = cxx::type_id!("JPH::BodyInterface");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_BroadPhaseLayerFilter {
    type Id = cxx::type_id!("compat::JPC_BroadPhaseLayerFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_BroadPhaseLayerFilterFns {
    type Id = cxx::type_id!("compat::JPC_BroadPhaseLayerFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_BroadPhaseLayerInterface {
    type Id = cxx::type_id!("JPH::BroadPhaseLayerInterface");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_BroadPhaseLayerInterfaceFns {
    type Id = cxx::type_id!("compat::JPC_BroadPhaseLayerInterfaceFns");
    type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for JPC_CastShapeCollector {
    type Id = cxx::type_id!("JPH::CastShapeCollector");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_CastShapeCollectorFns {
    type Id = cxx::type_id!("compat::JPC_CastShapeCollectorFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_CollideShapeCollector {
    type Id = cxx::type_id!("JPH::CollideShapeCollector");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_CollideShapeCollectorFns {
    type Id = cxx::type_id!("compat::JPC_CollideShapeCollectorFns");
    type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for JPC_Constraint {
    type Id = cxx::type_id!("JPH::Constraint");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ContactListener {
    type Id = cxx::type_id!("JPH::ContactListener");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ContactListenerFns {
    type Id = cxx::type_id!("compat::JPC_ContactListenerFns");
    type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for JPC_DebugRendererSimple {
    type Id = cxx::type_id!("compat::JPC_DebugRendererSimple");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_DebugRendererSimpleFns {
    type Id = cxx::type_id!("compat::JPC_DebugRendererSimpleFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_GroupFilter {
    type Id = cxx::type_id!("compat::JPC_GroupFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_GroupFilterFns {
    type Id = cxx::type_id!("compat::JPC_GroupFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_HeightFieldShape {
    type Id = cxx::type_id!("JPH::HeightFieldShape");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_JobSystem {
    type Id = cxx::type_id!("JPH::JobSystem");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_JobSystemThreadPool {
    type Id = cxx::type_id!("JPH::JobSystemThreadPool");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_MutableCompoundShape {
    type Id = cxx::type_id!("JPH::MutableCompoundShape");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_NarrowPhaseQuery {
    type Id = cxx::type_id!("JPH::NarrowPhaseQuery");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_ObjectLayerFilter {
    type Id = cxx::type_id!("compat::JPC_ObjectLayerFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ObjectLayerFilterFns {
    type Id = cxx::type_id!("compat::JPC_ObjectLayerFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_ObjectLayerPairFilter {
    type Id = cxx::type_id!("JPH::ObjectLayerPairFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ObjectLayerPairFilterFns {
    type Id = cxx::type_id!("compat::JPC_ObjectLayerPairFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_ObjectVsBroadPhaseLayerFilter {
    type Id = cxx::type_id!("JPH::ObjectVsBroadPhaseLayerFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ObjectVsBroadPhaseLayerFilterFns {
    type Id = cxx::type_id!("compat::JPC_ObjectVsBroadPhaseLayerFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_PhysicsSystem {
    type Id = cxx::type_id!("JPH::PhysicsSystem");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_Shape {
    type Id = cxx::type_id!("JPH::Shape");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_ShapeFilter {
    type Id = cxx::type_id!("JPH::ShapeFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_ShapeFilterFns {
    type Id = cxx::type_id!("compat::JPC_ShapeFilterFns");
    type Kind = cxx::kind::Trivial;
}
unsafe impl cxx::ExternType for JPC_SimShapeFilter {
    type Id = cxx::type_id!("JPH::SimShapeFilter");
    type Kind = cxx::kind::Opaque;
}
unsafe impl cxx::ExternType for JPC_SimShapeFilterFns {
    type Id = cxx::type_id!("compat::JPC_SimShapeFilterFns");
    type Kind = cxx::kind::Trivial;
}

unsafe impl cxx::ExternType for JPC_String {
    type Id = cxx::type_id!("compat::JPC_String");
    type Kind = cxx::kind::Opaque;
}

unsafe impl cxx::ExternType for JPC_TempAllocatorImpl {
    type Id = cxx::type_id!("JPH::TempAllocatorImpl");
    type Kind = cxx::kind::Opaque;
}
