use super::*;
// Public constants, opaque handles and callback tables retained from rolt.
// Value/settings records are declared once in bridge.rs through cxx.

pub const JPC_VECTOR_ALIGNMENT: u32 = 16;
pub const JPC_DVECTOR_ALIGNMENT: u32 = 32;
pub const JPC_OBJECT_LAYER_BITS: u32 = 16;
pub const JPC_MAX_PHYSICS_JOBS: ::std::os::raw::c_int = 2048;
pub const JPC_MAX_PHYSICS_BARRIERS: ::std::os::raw::c_int = 8;
pub const JPC_DEFAULT_COLLISION_TOLERANCE: f32 = 0.00009999999747378752;
pub const JPC_DEFAULT_PENETRATION_TOLERANCE: f32 = 0.00009999999747378752;
pub const JPC_DEFAULT_CONVEX_RADIUS: f32 = 0.05000000074505806;
pub const JPC_CAPSULE_PROJECTION_SLOP: f32 = 0.019999999552965164;
pub const JPC_SHAPE_TYPE_CONVEX: JPC_ShapeType = 0;
pub const JPC_SHAPE_TYPE_COMPOUND: JPC_ShapeType = 1;
pub const JPC_SHAPE_TYPE_DECORATED: JPC_ShapeType = 2;
pub const JPC_SHAPE_TYPE_MESH: JPC_ShapeType = 3;
pub const JPC_SHAPE_TYPE_HEIGHT_FIELD: JPC_ShapeType = 4;
pub const JPC_SHAPE_TYPE_SOFTBODY: JPC_ShapeType = 5;
pub const JPC_SHAPE_TYPE_USER1: JPC_ShapeType = 6;
pub const JPC_SHAPE_TYPE_USER2: JPC_ShapeType = 7;
pub const JPC_SHAPE_TYPE_USER3: JPC_ShapeType = 8;
pub const JPC_SHAPE_TYPE_USER4: JPC_ShapeType = 9;
pub type JPC_ShapeType = u8;
pub const JPC_SHAPE_SUB_TYPE_SPHERE: JPC_ShapeSubType = 0;
pub const JPC_SHAPE_SUB_TYPE_BOX: JPC_ShapeSubType = 1;
pub const JPC_SHAPE_SUB_TYPE_TRIANGLE: JPC_ShapeSubType = 2;
pub const JPC_SHAPE_SUB_TYPE_CAPSULE: JPC_ShapeSubType = 3;
pub const JPC_SHAPE_SUB_TYPE_TAPEREDCAPSULE: JPC_ShapeSubType = 4;
pub const JPC_SHAPE_SUB_TYPE_CYLINDER: JPC_ShapeSubType = 5;
pub const JPC_SHAPE_SUB_TYPE_CONVEX_HULL: JPC_ShapeSubType = 6;
pub const JPC_SHAPE_SUB_TYPE_STATIC_COMPOUND: JPC_ShapeSubType = 7;
pub const JPC_SHAPE_SUB_TYPE_MUTABLE_COMPOUND: JPC_ShapeSubType = 8;
pub const JPC_SHAPE_SUB_TYPE_ROTATED_TRANSLATED: JPC_ShapeSubType = 9;
pub const JPC_SHAPE_SUB_TYPE_SCALED: JPC_ShapeSubType = 10;
pub const JPC_SHAPE_SUB_TYPE_OFFSET_CENTER_OF_MASS: JPC_ShapeSubType = 11;
pub const JPC_SHAPE_SUB_TYPE_MESH: JPC_ShapeSubType = 12;
pub const JPC_SHAPE_SUB_TYPE_HEIGHT_FIELD: JPC_ShapeSubType = 13;
pub const JPC_SHAPE_SUB_TYPE_SOFT_BODY: JPC_ShapeSubType = 14;
pub const JPC_SHAPE_SUB_TYPE_USER1: JPC_ShapeSubType = 15;
pub const JPC_SHAPE_SUB_TYPE_USER2: JPC_ShapeSubType = 16;
pub const JPC_SHAPE_SUB_TYPE_USER3: JPC_ShapeSubType = 17;
pub const JPC_SHAPE_SUB_TYPE_USER4: JPC_ShapeSubType = 18;
pub const JPC_SHAPE_SUB_TYPE_USER5: JPC_ShapeSubType = 19;
pub const JPC_SHAPE_SUB_TYPE_USER6: JPC_ShapeSubType = 20;
pub const JPC_SHAPE_SUB_TYPE_USER7: JPC_ShapeSubType = 21;
pub const JPC_SHAPE_SUB_TYPE_USER8: JPC_ShapeSubType = 22;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX1: JPC_ShapeSubType = 23;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX2: JPC_ShapeSubType = 24;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX3: JPC_ShapeSubType = 25;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX4: JPC_ShapeSubType = 26;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX5: JPC_ShapeSubType = 27;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX6: JPC_ShapeSubType = 28;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX7: JPC_ShapeSubType = 29;
pub const JPC_SHAPE_SUB_TYPE_USER_CONVEX8: JPC_ShapeSubType = 30;
pub type JPC_ShapeSubType = u8;
pub type JPC_PhysicsUpdateError = u32;
pub const JPC_PHYSICS_UPDATE_ERROR_NONE: JPC_PhysicsUpdateError = 0;
pub const JPC_PHYSICS_UPDATE_ERROR_MANIFOLD_CACHE_FULL: JPC_PhysicsUpdateError = 1;
pub const JPC_PHYSICS_UPDATE_ERROR_BODY_PAIR_CACHE_FULL: JPC_PhysicsUpdateError = 2;
pub const JPC_PHYSICS_UPDATE_ERROR_CONTACT_CONSTRAINTS_FULL: JPC_PhysicsUpdateError = 4;
pub const JPC_CONSTRAINT_TYPE_CONSTRAINT: JPC_ConstraintType = 0;
pub const JPC_CONSTRAINT_TYPE_TWO_BODY_CONSTRAINT: JPC_ConstraintType = 1;
pub type JPC_ConstraintType = u32;
pub const JPC_CONSTRAINT_SUB_TYPE_FIXED: JPC_ConstraintSubType = 0;
pub const JPC_CONSTRAINT_SUB_TYPE_POINT: JPC_ConstraintSubType = 1;
pub const JPC_CONSTRAINT_SUB_TYPE_HINGE: JPC_ConstraintSubType = 2;
pub const JPC_CONSTRAINT_SUB_TYPE_SLIDER: JPC_ConstraintSubType = 3;
pub const JPC_CONSTRAINT_SUB_TYPE_DISTANCE: JPC_ConstraintSubType = 4;
pub const JPC_CONSTRAINT_SUB_TYPE_CONE: JPC_ConstraintSubType = 5;
pub const JPC_CONSTRAINT_SUB_TYPE_SWING_TWIST: JPC_ConstraintSubType = 6;
pub const JPC_CONSTRAINT_SUB_TYPE_SIX_DOF: JPC_ConstraintSubType = 7;
pub const JPC_CONSTRAINT_SUB_TYPE_PATH: JPC_ConstraintSubType = 8;
pub const JPC_CONSTRAINT_SUB_TYPE_VEHICLE: JPC_ConstraintSubType = 9;
pub const JPC_CONSTRAINT_SUB_TYPE_RACK_AND_PINION: JPC_ConstraintSubType = 10;
pub const JPC_CONSTRAINT_SUB_TYPE_GEAR: JPC_ConstraintSubType = 11;
pub const JPC_CONSTRAINT_SUB_TYPE_PULLEY: JPC_ConstraintSubType = 12;
pub const JPC_CONSTRAINT_SUB_TYPE_USER1: JPC_ConstraintSubType = 13;
pub const JPC_CONSTRAINT_SUB_TYPE_USER2: JPC_ConstraintSubType = 14;
pub const JPC_CONSTRAINT_SUB_TYPE_USER3: JPC_ConstraintSubType = 15;
pub const JPC_CONSTRAINT_SUB_TYPE_USER4: JPC_ConstraintSubType = 16;
pub type JPC_ConstraintSubType = u32;
pub const JPC_CONSTRAINT_SPACE_LOCAL_TO_BODY_COM: JPC_ConstraintSpace = 0;
pub const JPC_CONSTRAINT_SPACE_WORLD_SPACE: JPC_ConstraintSpace = 1;
pub type JPC_ConstraintSpace = u32;
pub const JPC_MOTION_TYPE_STATIC: JPC_MotionType = 0;
pub const JPC_MOTION_TYPE_KINEMATIC: JPC_MotionType = 1;
pub const JPC_MOTION_TYPE_DYNAMIC: JPC_MotionType = 2;
pub type JPC_MotionType = u8;
pub const JPC_MOTION_QUALITY_DISCRETE: JPC_MotionQuality = 0;
pub const JPC_MOTION_QUALITY_LINEAR_CAST: JPC_MotionQuality = 1;
pub type JPC_MotionQuality = u8;
pub const JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA: JPC_OverrideMassProperties = 0;
pub const JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA: JPC_OverrideMassProperties = 1;
pub const JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED: JPC_OverrideMassProperties = 2;
pub type JPC_OverrideMassProperties = u8;
pub const JPC_CHARACTER_GROUND_STATE_ON_GROUND: JPC_GroundState = 0;
pub const JPC_CHARACTER_GROUND_STATE_ON_STEEP_GROUND: JPC_GroundState = 1;
pub const JPC_CHARACTER_GROUND_STATE_NOT_SUPPORTED: JPC_GroundState = 2;
pub const JPC_CHARACTER_GROUND_STATE_IN_AIR: JPC_GroundState = 3;
pub type JPC_GroundState = u32;
pub const JPC_ACTIVATION_ACTIVATE: JPC_Activation = 0;
pub const JPC_ACTIVATION_DONT_ACTIVATE: JPC_Activation = 1;
pub type JPC_Activation = u32;
pub const JPC_VALIDATE_RESULT_ACCEPT_ALL_CONTACTS: JPC_ValidateResult = 0;
pub const JPC_VALIDATE_RESULT_ACCEPT_CONTACT: JPC_ValidateResult = 1;
pub const JPC_VALIDATE_RESULT_REJECT_CONTACT: JPC_ValidateResult = 2;
pub const JPC_VALIDATE_RESULT_REJECT_ALL_CONTACTS: JPC_ValidateResult = 3;
pub type JPC_ValidateResult = u32;
pub type JPC_BackFaceMode = u8;
pub const JPC_BACK_FACE_MODE_IGNORE: JPC_BackFaceMode = 0;
pub const JPC_BACK_FACE_MODE_COLLIDE: JPC_BackFaceMode = 1;
pub const JPC_BODY_TYPE_RIGID_BODY: JPC_BodyType = 0;
pub const JPC_BODY_TYPE_SOFT_BODY: JPC_BodyType = 1;
pub type JPC_BodyType = u8;
pub const JPC_ALLOWED_DOFS_NONE: JPC_AllowedDOFs = 0;
pub const JPC_ALLOWED_DOFS_ALL: JPC_AllowedDOFs = 63;
pub const JPC_ALLOWED_DOFS_TRANSLATIONX: JPC_AllowedDOFs = 1;
pub const JPC_ALLOWED_DOFS_TRANSLATIONY: JPC_AllowedDOFs = 2;
pub const JPC_ALLOWED_DOFS_TRANSLATIONZ: JPC_AllowedDOFs = 4;
pub const JPC_ALLOWED_DOFS_ROTATIONX: JPC_AllowedDOFs = 8;
pub const JPC_ALLOWED_DOFS_ROTATIONY: JPC_AllowedDOFs = 16;
pub const JPC_ALLOWED_DOFS_ROTATIONZ: JPC_AllowedDOFs = 32;
pub const JPC_ALLOWED_DOFS_PLANE2D: JPC_AllowedDOFs = 35;
pub type JPC_AllowedDOFs = u8;
pub const JPC_FEATURE_DOUBLE_PRECISION: JPC_Features = 1;
pub const JPC_FEATURE_NEON: JPC_Features = 2;
pub const JPC_FEATURE_SSE: JPC_Features = 4;
pub const JPC_FEATURE_SSE4_1: JPC_Features = 8;
pub const JPC_FEATURE_SSE4_2: JPC_Features = 16;
pub const JPC_FEATURE_AVX: JPC_Features = 32;
pub const JPC_FEATURE_AVX2: JPC_Features = 64;
pub const JPC_FEATURE_AVX512: JPC_Features = 128;
pub const JPC_FEATURE_F16C: JPC_Features = 256;
pub const JPC_FEATURE_LZCNT: JPC_Features = 512;
pub const JPC_FEATURE_TZCNT: JPC_Features = 1024;
pub const JPC_FEATURE_FMADD: JPC_Features = 2048;
pub const JPC_FEATURE_PLATFORM_DETERMINISTIC: JPC_Features = 4096;
pub const JPC_FEATURE_FLOATING_POINT_EXCEPTIONS: JPC_Features = 8192;
pub const JPC_FEATURE_DEBUG: JPC_Features = 16384;
pub type JPC_Features = u32;
pub type JPC_ShapeColor = ::std::os::raw::c_int;
pub const JPC_SHAPE_COLOR_INSTANCE_COLOR: JPC_ShapeColor = 0;
pub const JPC_SHAPE_COLOR_SHAPE_TYPE_COLOR: JPC_ShapeColor = 1;
pub const JPC_SHAPE_COLOR_MOTION_TYPE_COLOR: JPC_ShapeColor = 2;
pub const JPC_SHAPE_COLOR_SLEEP_COLOR: JPC_ShapeColor = 3;
pub const JPC_SHAPE_COLOR_ISLAND_COLOR: JPC_ShapeColor = 4;
pub const JPC_SHAPE_COLOR_MATERIAL_COLOR: JPC_ShapeColor = 5;
pub type JPC_SoftBodyConstraintColor = ::std::os::raw::c_int;
pub const JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_TYPE: JPC_SoftBodyConstraintColor = 0;
pub const JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_GROUP: JPC_SoftBodyConstraintColor = 1;
pub const JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_ORDER: JPC_SoftBodyConstraintColor = 2;
pub type JPC_ActiveEdgeMode = u8;
pub const JPC_ACTIVE_EDGE_MODE_COLLIDE_ONLY_WITH_ACTIVE: JPC_ActiveEdgeMode = 0;
pub const JPC_ACTIVE_EDGE_MODE_COLLIDE_WITH_ALL: JPC_ActiveEdgeMode = 1;
pub type JPC_CollectFacesMode = u8;
pub const JPC_COLLECT_FACES_MODE_COLLECT_FACES: JPC_CollectFacesMode = 0;
pub const JPC_COLLECT_FACES_MODE_NO_FACES: JPC_CollectFacesMode = 1;
pub type JPC_SwingType = u8;
pub const JPC_SWING_TYPE_CONE: JPC_SwingType = 0;
pub const JPC_SWING_TYPE_PYRAMID: JPC_SwingType = 1;
pub type JPC_SixDOFConstraint_Axis = u32;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_TRANSLATION_X: JPC_SixDOFConstraint_Axis = 0;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_TRANSLATION_Y: JPC_SixDOFConstraint_Axis = 1;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_TRANSLATION_Z: JPC_SixDOFConstraint_Axis = 2;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_ROTATION_X: JPC_SixDOFConstraint_Axis = 3;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_ROTATION_Y: JPC_SixDOFConstraint_Axis = 4;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_ROTATION_Z: JPC_SixDOFConstraint_Axis = 5;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_NUM: JPC_SixDOFConstraint_Axis = 6;
pub const JPC_SIX_DOF_CONSTRAINT_AXIS_NUM_TRANSLATION: JPC_SixDOFConstraint_Axis = 3;
pub const JPC_SPRING_MODE_FREQUENCY_AND_DAMPING: JPC_SpringMode = 0;
pub const JPC_SPRING_MODE_STIFFNESS_AND_DAMPING: JPC_SpringMode = 1;
pub type JPC_SpringMode = u8;
pub const JPC_MOTOR_STATE_OFF: JPC_MotorState = 0;
pub const JPC_MOTOR_STATE_VELOCITY: JPC_MotorState = 1;
pub const JPC_MOTOR_STATE_POSITION: JPC_MotorState = 2;
pub type JPC_MotorState = u32;
pub const JPC_PI: f32 = 3.1415927410125732;
pub type JPC_AssertFailedFunction = ::std::option::Option<
    unsafe extern "C" fn(
        inExpression: *const ::std::os::raw::c_char,
        inMessage: *const ::std::os::raw::c_char,
        inFile: *const ::std::os::raw::c_char,
        inLine: ::std::os::raw::c_uint,
    ) -> bool,
>;

pub type JPC_RVec3 = JPC_Vec3;
pub type JPC_RMat44 = JPC_Mat44;
pub type JPC_BodyID = u32;
pub type JPC_SubShapeID = u32;
pub type JPC_BroadPhaseLayer = u8;
pub type JPC_ObjectLayer = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_Body {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_VertexList {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_IndexedTriangleList {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_Shape {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_CompoundShape {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_TempAllocatorImpl {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_JobSystem {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_JobSystemThreadPool {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_JobSystemSingleThreaded {
    _unused: [u8; 0],
}
pub type JPC_GroupID = u32;
pub type JPC_SubGroupID = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_GroupFilter {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_GroupFilterFns {
    pub CanCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inGroup1: *const JPC_CollisionGroup,
            inGroup2: *const JPC_CollisionGroup,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BroadPhaseLayerInterfaceFns {
    pub GetNumBroadPhaseLayers: ::std::option::Option<
        unsafe extern "C" fn(self_: *const ::std::os::raw::c_void) -> ::std::os::raw::c_uint,
    >,
    pub GetBroadPhaseLayer: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inLayer: JPC_ObjectLayer,
        ) -> JPC_BroadPhaseLayer,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BroadPhaseLayerInterface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BroadPhaseLayerFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inLayer: JPC_BroadPhaseLayer,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BroadPhaseLayerFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectLayerFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inLayer: JPC_ObjectLayer,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectLayerFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(self_: *const ::std::os::raw::c_void, inBodyID: JPC_BodyID) -> bool,
    >,
    pub ShouldCollideLocked: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inBodyID: *const JPC_Body,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ShapeFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ShapeFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            filter: *const JPC_ShapeFilter,
            inShape2: *const JPC_Shape,
            inSubShapeIDOfShape2: JPC_SubShapeID,
        ) -> bool,
    >,
    pub ShouldCollideTwoShapes: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            filter: *const JPC_ShapeFilter,
            inShape1: *const JPC_Shape,
            inSubShapeIDOfShape1: JPC_SubShapeID,
            inShape2: *const JPC_Shape,
            inSubShapeIDOfShape2: JPC_SubShapeID,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_SimShapeFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inBody1: *const JPC_Body,
            inShape1: *const JPC_Shape,
            inSubShapeIDOfShape1: JPC_SubShapeID,
            inBody2: *const JPC_Body,
            inShape2: *const JPC_Shape,
            inSubShapeIDOfShape2: JPC_SubShapeID,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_SimShapeFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectVsBroadPhaseLayerFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inLayer1: JPC_ObjectLayer,
            inLayer2: JPC_BroadPhaseLayer,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectVsBroadPhaseLayerFilter {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectLayerPairFilterFns {
    pub ShouldCollide: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inLayer1: JPC_ObjectLayer,
            inLayer2: JPC_ObjectLayer,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ObjectLayerPairFilter {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ContactListenerFns {
    pub OnContactValidate: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            inBody1: *const JPC_Body,
            inBody2: *const JPC_Body,
            inBaseOffset: JPC_RVec3,
            inCollisionResult: *const JPC_CollideShapeResult,
        ) -> JPC_ValidateResult,
    >,
    pub OnContactAdded: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            inBody1: *const JPC_Body,
            inBody2: *const JPC_Body,
            inManifold: *const JPC_ContactManifold,
            ioSettings: *mut JPC_ContactSettings,
        ),
    >,
    pub OnContactPersisted: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            inBody1: *const JPC_Body,
            inBody2: *const JPC_Body,
            inManifold: *const JPC_ContactManifold,
            ioSettings: *mut JPC_ContactSettings,
        ),
    >,
    pub OnContactRemoved: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            inSubShapePair: *const JPC_SubShapeIDPair,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_ContactListener {
    _unused: [u8; 0],
}
pub const JPC_ContactPointsCapacity: ::std::os::raw::c_uint = 64;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_CastShapeCollector {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_CastShapeCollectorFns {
    pub Reset: ::std::option::Option<unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void)>,
    pub AddHit: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            base: *mut JPC_CastShapeCollector,
            Result: *const JPC_ShapeCastResult,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_CollideShapeCollector {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_CollideShapeCollectorFns {
    pub Reset: ::std::option::Option<unsafe extern "C" fn(self_: *mut ::std::os::raw::c_void)>,
    pub AddHit: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *mut ::std::os::raw::c_void,
            base: *mut JPC_CollideShapeCollector,
            Result: *const JPC_CollideShapeResult,
        ),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_DebugRendererSimpleFns {
    pub DrawLine: ::std::option::Option<
        unsafe extern "C" fn(
            self_: *const ::std::os::raw::c_void,
            inFrom: JPC_RVec3,
            inTo: JPC_RVec3,
            inColor: JPC_Color,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_DebugRendererSimple {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_String {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_Constraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_TwoBodyConstraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_FixedConstraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_DistanceConstraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_SixDOFConstraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_HingeConstraint {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_SliderConstraint {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_MutableCompoundShape {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyLockInterface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyLockRead {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyLockWrite {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyLockMultiRead {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyLockMultiWrite {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_BodyInterface {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_NarrowPhaseQuery {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_PhysicsSystem {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JPC_HeightFieldShape {
    _unused: [u8; 0],
}
