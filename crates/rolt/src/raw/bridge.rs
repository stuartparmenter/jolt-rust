//! Shared public records and explicit cxx declarations for the native adapters.
#[cxx::bridge(namespace = "compat")]
pub(crate) mod ffi {
    #[derive(Debug, Copy, Clone)]
    struct JPC_Float3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_Vec2 {
        pub x: f32,
        pub y: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub _w: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_Vec4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    #[repr(align(32))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_DVec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub _w: f64,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_Quat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_Mat44 {
        pub col: [JPC_Vec4; 3],
        pub col3: JPC_Vec3,
    }
    #[repr(align(32))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_DMat44 {
        pub col: [JPC_Vec4; 3],
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: [u64; 2],
        pub col3: JPC_DVec3,
    }
    #[repr(align(4))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_IndexedTriangleNoMaterial {
        pub idx: [u32; 3],
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_IndexedTriangle {
        pub idx: [u32; 3],
        pub materialIndex: u32,
        pub userData: u32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_RayCast {
        pub Origin: JPC_Vec3,
        pub Direction: JPC_Vec3,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_RRayCast {
        pub Origin: JPC_Vec3,
        pub Direction: JPC_Vec3,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_RayCastResult {
        pub BodyID: u32,
        pub Fraction: f32,
        pub SubShapeID2: u32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_ShapeCastResult {
        pub ContactPointOn1: JPC_Vec3,
        pub ContactPointOn2: JPC_Vec3,
        pub PenetrationAxis: JPC_Vec3,
        pub PenetrationDepth: f32,
        pub SubShapeID1: u32,
        pub SubShapeID2: u32,
        pub BodyID2: u32,
        pub Fraction: f32,
        pub IsBackFaceHit: bool,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_CollideShapeResult {
        pub ContactPointOn1: JPC_Vec3,
        pub ContactPointOn2: JPC_Vec3,
        pub PenetrationAxis: JPC_Vec3,
        pub PenetrationDepth: f32,
        pub SubShapeID1: u32,
        pub SubShapeID2: u32,
        pub BodyID2: u32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_CollisionGroup {
        pub GroupFilter: *const JPC_GroupFilter,
        pub GroupID: u32,
        pub SubGroupID: u32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_ContactPoints {
        pub length: u32,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub points: [JPC_Vec3; 64],
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_ContactManifold {
        pub BaseOffset: JPC_Vec3,
        pub WorldSpaceNormal: JPC_Vec3,
        pub PenetrationDepth: f32,
        pub SubShapeID1: u32,
        pub SubShapeID2: u32,
        pub RelativeContactPointsOn1: JPC_ContactPoints,
        pub RelativeContactPointsOn2: JPC_ContactPoints,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_ContactSettings {
        pub CombinedFriction: f32,
        pub CombinedRestitution: f32,
        pub InvMassScale1: f32,
        pub InvInertiaScale1: f32,
        pub InvMassScale2: f32,
        pub InvInertiaScale2: f32,
        pub IsSensor: bool,
        pub RelativeLinearSurfaceVelocity: JPC_Vec3,
        pub RelativeAngularSurfaceVelocity: JPC_Vec3,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_SubShapeIDPair {
        pub Body1ID: u32,
        pub SubShapeID1: u32,
        pub Body2ID: u32,
        pub SubShapeID2: u32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_ShapeCastSettings {
        pub ActiveEdgeMode: u8,
        pub CollectFacesMode: u8,
        pub CollisionTolerance: f32,
        pub PenetrationTolerance: f32,
        pub ActiveEdgeMovementDirection: JPC_Vec3,
        pub ExtraConvexRadius: f32,
        pub BackFaceModeTriangles: u8,
        pub BackFaceModeConvex: u8,
        pub UseShrunkenShapeAndConvexRadius: bool,
        pub ReturnDeepestPoint: bool,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_CollideShapeSettings {
        pub ActiveEdgeMode: u8,
        pub CollectFacesMode: u8,
        pub CollisionTolerance: f32,
        pub PenetrationTolerance: f32,
        pub ActiveEdgeMovementDirection: JPC_Vec3,
        pub MaxSeparationDistance: f32,
        pub BackFaceMode: u8,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_CollisionEstimationResult {
        pub LinearVelocity1: JPC_Vec3,
        pub AngularVelocity1: JPC_Vec3,
        pub LinearVelocity2: JPC_Vec3,
        pub AngularVelocity2: JPC_Vec3,
        pub FrictionPoint: JPC_Vec3,
        pub Tangent1: JPC_Vec3,
        pub Tangent2: JPC_Vec3,
        pub FrictionImpulse1: f32,
        pub FrictionImpulse2: f32,
        pub AngularFrictionImpulse: f32,
        pub NumImpulses: u32,
        pub Impulses: [f32; 64],
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_BodyManager_DrawSettings {
        pub mDrawGetSupportFunction: bool,
        pub mDrawSupportDirection: bool,
        pub mDrawGetSupportingFace: bool,
        pub mDrawShape: bool,
        pub mDrawShapeWireframe: bool,
        pub mDrawShapeColor: i32,
        pub mDrawBoundingBox: bool,
        pub mDrawCenterOfMassTransform: bool,
        pub mDrawWorldTransform: bool,
        pub mDrawVelocity: bool,
        pub mDrawMassAndInertia: bool,
        pub mDrawSleepStats: bool,
        pub mDrawSoftBodyVertices: bool,
        pub mDrawSoftBodyVertexVelocities: bool,
        pub mDrawSoftBodyEdgeConstraints: bool,
        pub mDrawSoftBodyBendConstraints: bool,
        pub mDrawSoftBodyVolumeConstraints: bool,
        pub mDrawSoftBodySkinConstraints: bool,
        pub mDrawSoftBodyLRAConstraints: bool,
        pub mDrawSoftBodyRods: bool,
        pub mDrawSoftBodyRodStates: bool,
        pub mDrawSoftBodyRodBendTwistConstraints: bool,
        pub mDrawSoftBodyPredictedBounds: bool,
        pub DrawSoftBodyConstraintColor: i32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_ConstraintSettings {
        pub Enabled: bool,
        pub ConstraintPriority: u32,
        pub NumVelocityStepsOverride: u32,
        pub NumPositionStepsOverride: u32,
        pub DrawConstraintSize: f32,
        pub UserData: u64,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_SpringSettings {
        pub Mode: u8,
        pub FrequencyOrStiffness: f32,
        pub Damping: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_MotorSettings {
        pub SpringSettings: JPC_SpringSettings,
        pub MinForceLimit: f32,
        pub MaxForceLimit: f32,
        pub MinTorqueLimit: f32,
        pub MaxTorqueLimit: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_FixedConstraintSettings {
        pub ConstraintSettings: JPC_ConstraintSettings,
        pub Space: u32,
        pub AutoDetectPoint: bool,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Point1: JPC_Vec3,
        pub AxisX1: JPC_Vec3,
        pub AxisY1: JPC_Vec3,
        pub Point2: JPC_Vec3,
        pub AxisX2: JPC_Vec3,
        pub AxisY2: JPC_Vec3,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_SixDOFConstraintSettings {
        pub ConstraintSettings: JPC_ConstraintSettings,
        pub Space: u32,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Position1: JPC_Vec3,
        pub AxisX1: JPC_Vec3,
        pub AxisY1: JPC_Vec3,
        pub Position2: JPC_Vec3,
        pub AxisX2: JPC_Vec3,
        pub AxisY2: JPC_Vec3,
        pub MaxFriction: [f32; 6],
        pub LimitMin: [f32; 6],
        pub LimitMax: [f32; 6],
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_HingeConstraintSettings {
        pub ConstraintSettings: JPC_ConstraintSettings,
        pub Space: u32,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Point1: JPC_Vec3,
        pub HingeAxis1: JPC_Vec3,
        pub NormalAxis1: JPC_Vec3,
        pub Point2: JPC_Vec3,
        pub HingeAxis2: JPC_Vec3,
        pub NormalAxis2: JPC_Vec3,
        pub LimitsMin: f32,
        pub LimitsMax: f32,
        pub LimitsSpringSettings: JPC_SpringSettings,
        pub MaxFrictionTorque: f32,
        pub MotorSettings: JPC_MotorSettings,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_DistanceConstraintSettings {
        pub ConstraintSettings: JPC_ConstraintSettings,
        pub Space: u32,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Point1: JPC_Vec3,
        pub Point2: JPC_Vec3,
        pub MinDistance: f32,
        pub MaxDistance: f32,
        pub LimitsSpringSettings: JPC_SpringSettings,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_SliderConstraintSettings {
        pub ConstraintSettings: JPC_ConstraintSettings,
        pub Space: u32,
        pub AutoDetectPoint: bool,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Point1: JPC_Vec3,
        pub SliderAxis1: JPC_Vec3,
        pub NormalAxis1: JPC_Vec3,
        pub Point2: JPC_Vec3,
        pub SliderAxis2: JPC_Vec3,
        pub NormalAxis2: JPC_Vec3,
        pub LimitsMin: f32,
        pub LimitsMax: f32,
        pub LimitsSpringSettings: JPC_SpringSettings,
        pub MaxFrictionForce: f32,
        pub MotorSettings: JPC_MotorSettings,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_TriangleShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub V1: JPC_Vec3,
        pub V2: JPC_Vec3,
        pub V3: JPC_Vec3,
        pub ConvexRadius: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_MeshShapeSettings {
        pub UserData: u64,
        pub TriangleVertices: *mut JPC_Float3,
        pub TriangleVerticesLen: usize,
        pub IndexedTriangles: *mut JPC_IndexedTriangle,
        pub IndexedTrianglesLen: usize,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_BoxShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub HalfExtent: JPC_Vec3,
        pub ConvexRadius: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_SphereShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub Radius: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_CapsuleShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub Radius: f32,
        pub HalfHeightOfCylinder: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_CylinderShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub HalfHeight: f32,
        pub Radius: f32,
        pub ConvexRadius: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_PlaneShapeSettings {
        pub UserData: u64,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Normal: JPC_Vec3,
        pub Constant: f32,
        pub HalfExtent: f32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_ConvexHullShapeSettings {
        pub UserData: u64,
        pub Density: f32,
        pub Points: *const JPC_Vec3,
        pub PointsLen: usize,
        pub MaxConvexRadius: f32,
        pub MaxErrorConvexRadius: f32,
        pub HullTolerance: f32,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_SubShapeSettings {
        pub Shape: *const JPC_Shape,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Position: JPC_Vec3,
        pub Rotation: JPC_Quat,
        pub UserData: u32,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_StaticCompoundShapeSettings {
        pub UserData: u64,
        pub SubShapes: *const JPC_SubShapeSettings,
        pub SubShapesLen: usize,
    }
    #[derive(Debug, Copy, Clone)]
    struct JPC_MutableCompoundShapeSettings {
        pub UserData: u64,
        pub SubShapes: *const JPC_SubShapeSettings,
        pub SubShapesLen: usize,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_MassProperties {
        pub Mass: f32,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Inertia: JPC_Mat44,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_BodyCreationSettings {
        pub Position: JPC_Vec3,
        pub Rotation: JPC_Quat,
        pub LinearVelocity: JPC_Vec3,
        pub AngularVelocity: JPC_Vec3,
        pub UserData: u64,
        pub ObjectLayer: u16,
        pub MotionType: u8,
        pub AllowedDOFs: u8,
        pub AllowDynamicOrKinematic: bool,
        pub IsSensor: bool,
        pub CollideKinematicVsNonDynamic: bool,
        pub UseManifoldReduction: bool,
        pub ApplyGyroscopicForce: bool,
        pub MotionQuality: u8,
        pub EnhancedInternalEdgeRemoval: bool,
        pub AllowSleeping: bool,
        pub Friction: f32,
        pub Restitution: f32,
        pub LinearDamping: f32,
        pub AngularDamping: f32,
        pub MaxLinearVelocity: f32,
        pub MaxAngularVelocity: f32,
        pub GravityFactor: f32,
        pub NumVelocityStepsOverride: u32,
        pub NumPositionStepsOverride: u32,
        pub OverrideMassProperties: u8,
        pub MassPropertiesOverride: JPC_MassProperties,
        pub InertiaMultiplier: f32,
        pub Shape: *const JPC_Shape,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_NarrowPhaseQuery_CastRayArgs {
        pub Ray: JPC_RRayCast,
        pub Result: JPC_RayCastResult,
        pub BroadPhaseLayerFilter: *const JPC_BroadPhaseLayerFilter,
        pub ObjectLayerFilter: *const JPC_ObjectLayerFilter,
        pub BodyFilter: *const JPC_BodyFilter,
        pub ShapeFilter: *const JPC_ShapeFilter,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_RShapeCast {
        pub Shape: *const JPC_Shape,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Scale: JPC_Vec3,
        pub CenterOfMassStart: JPC_Mat44,
        pub Direction: JPC_Vec3,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_NarrowPhaseQuery_CastShapeArgs {
        pub ShapeCast: JPC_RShapeCast,
        pub Settings: JPC_ShapeCastSettings,
        pub BaseOffset: JPC_Vec3,
        pub Collector: *mut JPC_CastShapeCollector,
        pub BroadPhaseLayerFilter: *const JPC_BroadPhaseLayerFilter,
        pub ObjectLayerFilter: *const JPC_ObjectLayerFilter,
        pub BodyFilter: *const JPC_BodyFilter,
        pub ShapeFilter: *const JPC_ShapeFilter,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_NarrowPhaseQuery_CollideShapeArgs {
        pub Shape: *const JPC_Shape,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub ShapeScale: JPC_Vec3,
        pub CenterOfMassTransform: JPC_Mat44,
        pub Settings: JPC_CollideShapeSettings,
        pub BaseOffset: JPC_Vec3,
        pub Collector: *mut JPC_CollideShapeCollector,
        pub BroadPhaseLayerFilter: *const JPC_BroadPhaseLayerFilter,
        pub ObjectLayerFilter: *const JPC_ObjectLayerFilter,
        pub BodyFilter: *const JPC_BodyFilter,
        pub ShapeFilter: *const JPC_ShapeFilter,
    }
    #[repr(align(16))]
    #[derive(Debug, Copy, Clone)]
    struct JPC_HeightFieldShapeSettings {
        pub UserData: u64,
        #[cxx_name = "padding"]
        pub __bindgen_padding_0: u64,
        pub Offset: JPC_Vec3,
        pub Scale: JPC_Vec3,
        pub SampleCount: u32,
        pub MinHeightValue: f32,
        pub MaxHeightValue: f32,
        pub MaterialsCapacity: u32,
        pub BlockSize: u32,
        pub BitsPerSample: u32,
        pub HeightSamples: *const f32,
        pub HeightSamplesLen: usize,
        pub ActiveEdgeCosThresholdAngle: f32,
    }
    unsafe extern "C++" {
        include!("bridge.h");
        #[namespace = "JPH"]
        #[cxx_name = "Body"]
        type JPC_Body = crate::raw::JPC_Body;

        #[namespace = "compat"]
        #[cxx_name = "JPC_BodyFilter"]
        type JPC_BodyFilter = crate::raw::JPC_BodyFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_BodyFilterFns"]
        type JPC_BodyFilterFns = crate::raw::JPC_BodyFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "BodyInterface"]
        type JPC_BodyInterface = crate::raw::JPC_BodyInterface;

        #[namespace = "compat"]
        #[cxx_name = "JPC_BroadPhaseLayerFilter"]
        type JPC_BroadPhaseLayerFilter = crate::raw::JPC_BroadPhaseLayerFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_BroadPhaseLayerFilterFns"]
        type JPC_BroadPhaseLayerFilterFns = crate::raw::JPC_BroadPhaseLayerFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "BroadPhaseLayerInterface"]
        type JPC_BroadPhaseLayerInterface = crate::raw::JPC_BroadPhaseLayerInterface;
        #[namespace = "compat"]
        #[cxx_name = "JPC_BroadPhaseLayerInterfaceFns"]
        type JPC_BroadPhaseLayerInterfaceFns = crate::raw::JPC_BroadPhaseLayerInterfaceFns;

        #[namespace = "JPH"]
        #[cxx_name = "CastShapeCollector"]
        type JPC_CastShapeCollector = crate::raw::JPC_CastShapeCollector;
        #[namespace = "compat"]
        #[cxx_name = "JPC_CastShapeCollectorFns"]
        type JPC_CastShapeCollectorFns = crate::raw::JPC_CastShapeCollectorFns;
        #[namespace = "JPH"]
        #[cxx_name = "CollideShapeCollector"]
        type JPC_CollideShapeCollector = crate::raw::JPC_CollideShapeCollector;
        #[namespace = "compat"]
        #[cxx_name = "JPC_CollideShapeCollectorFns"]
        type JPC_CollideShapeCollectorFns = crate::raw::JPC_CollideShapeCollectorFns;

        #[namespace = "JPH"]
        #[cxx_name = "Constraint"]
        type JPC_Constraint = crate::raw::JPC_Constraint;
        #[namespace = "JPH"]
        #[cxx_name = "ContactListener"]
        type JPC_ContactListener = crate::raw::JPC_ContactListener;
        #[namespace = "compat"]
        #[cxx_name = "JPC_ContactListenerFns"]
        type JPC_ContactListenerFns = crate::raw::JPC_ContactListenerFns;

        #[namespace = "compat"]
        #[cxx_name = "JPC_DebugRendererSimple"]
        type JPC_DebugRendererSimple = crate::raw::JPC_DebugRendererSimple;
        #[namespace = "compat"]
        #[cxx_name = "JPC_DebugRendererSimpleFns"]
        type JPC_DebugRendererSimpleFns = crate::raw::JPC_DebugRendererSimpleFns;
        #[namespace = "compat"]
        #[cxx_name = "JPC_GroupFilter"]
        type JPC_GroupFilter = crate::raw::JPC_GroupFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_GroupFilterFns"]
        type JPC_GroupFilterFns = crate::raw::JPC_GroupFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "HeightFieldShape"]
        type JPC_HeightFieldShape = crate::raw::JPC_HeightFieldShape;

        #[namespace = "JPH"]
        #[cxx_name = "JobSystem"]
        type JPC_JobSystem = crate::raw::JPC_JobSystem;
        #[namespace = "JPH"]
        #[cxx_name = "JobSystemThreadPool"]
        type JPC_JobSystemThreadPool = crate::raw::JPC_JobSystemThreadPool;

        #[namespace = "JPH"]
        #[cxx_name = "MutableCompoundShape"]
        type JPC_MutableCompoundShape = crate::raw::JPC_MutableCompoundShape;

        #[namespace = "JPH"]
        #[cxx_name = "NarrowPhaseQuery"]
        type JPC_NarrowPhaseQuery = crate::raw::JPC_NarrowPhaseQuery;

        #[namespace = "compat"]
        #[cxx_name = "JPC_ObjectLayerFilter"]
        type JPC_ObjectLayerFilter = crate::raw::JPC_ObjectLayerFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_ObjectLayerFilterFns"]
        type JPC_ObjectLayerFilterFns = crate::raw::JPC_ObjectLayerFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "ObjectLayerPairFilter"]
        type JPC_ObjectLayerPairFilter = crate::raw::JPC_ObjectLayerPairFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_ObjectLayerPairFilterFns"]
        type JPC_ObjectLayerPairFilterFns = crate::raw::JPC_ObjectLayerPairFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "ObjectVsBroadPhaseLayerFilter"]
        type JPC_ObjectVsBroadPhaseLayerFilter = crate::raw::JPC_ObjectVsBroadPhaseLayerFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_ObjectVsBroadPhaseLayerFilterFns"]
        type JPC_ObjectVsBroadPhaseLayerFilterFns =
            crate::raw::JPC_ObjectVsBroadPhaseLayerFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "PhysicsSystem"]
        type JPC_PhysicsSystem = crate::raw::JPC_PhysicsSystem;

        #[namespace = "JPH"]
        #[cxx_name = "Shape"]
        type JPC_Shape = crate::raw::JPC_Shape;

        #[namespace = "JPH"]
        #[cxx_name = "ShapeFilter"]
        type JPC_ShapeFilter = crate::raw::JPC_ShapeFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_ShapeFilterFns"]
        type JPC_ShapeFilterFns = crate::raw::JPC_ShapeFilterFns;
        #[namespace = "JPH"]
        #[cxx_name = "SimShapeFilter"]
        type JPC_SimShapeFilter = crate::raw::JPC_SimShapeFilter;
        #[namespace = "compat"]
        #[cxx_name = "JPC_SimShapeFilterFns"]
        type JPC_SimShapeFilterFns = crate::raw::JPC_SimShapeFilterFns;

        #[namespace = "compat"]
        #[cxx_name = "JPC_String"]
        type JPC_String = crate::raw::JPC_String;

        #[namespace = "JPH"]
        #[cxx_name = "TempAllocatorImpl"]
        type JPC_TempAllocatorImpl = crate::raw::JPC_TempAllocatorImpl;

        unsafe fn JPC_BodyCreationSettings_default(
            settings: *mut JPC_BodyCreationSettings,
        ) -> Result<()>;
        unsafe fn JPC_BodyFilter_delete(object: *mut JPC_BodyFilter) -> Result<()>;
        unsafe fn JPC_BodyFilter_new(
            self_: usize,
            fns: JPC_BodyFilterFns,
        ) -> Result<*mut JPC_BodyFilter>;
        unsafe fn JPC_BodyManager_DrawSettings_default(
            object: *mut JPC_BodyManager_DrawSettings,
        ) -> Result<()>;
        unsafe fn JPC_BoxShapeSettings_Create(
            self_: *const JPC_BoxShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_BoxShapeSettings_default(object: *mut JPC_BoxShapeSettings) -> Result<()>;
        unsafe fn JPC_BroadPhaseLayerFilter_delete(
            object: *mut JPC_BroadPhaseLayerFilter,
        ) -> Result<()>;
        unsafe fn JPC_BroadPhaseLayerFilter_new(
            self_: usize,
            fns: JPC_BroadPhaseLayerFilterFns,
        ) -> Result<*mut JPC_BroadPhaseLayerFilter>;
        unsafe fn JPC_BroadPhaseLayerInterface_delete(
            object: *mut JPC_BroadPhaseLayerInterface,
        ) -> Result<()>;
        unsafe fn JPC_BroadPhaseLayerInterface_new(
            self_: usize,
            fns: JPC_BroadPhaseLayerInterfaceFns,
        ) -> Result<*mut JPC_BroadPhaseLayerInterface>;
        unsafe fn JPC_CapsuleShapeSettings_Create(
            self_: *const JPC_CapsuleShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_CapsuleShapeSettings_default(
            object: *mut JPC_CapsuleShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_CastShapeCollector_delete(object: *mut JPC_CastShapeCollector) -> Result<()>;
        unsafe fn JPC_CastShapeCollector_new(
            self_: usize,
            fns: JPC_CastShapeCollectorFns,
        ) -> Result<*mut JPC_CastShapeCollector>;
        unsafe fn JPC_CollideShapeCollector_delete(
            object: *mut JPC_CollideShapeCollector,
        ) -> Result<()>;
        unsafe fn JPC_CollideShapeCollector_new(
            self_: usize,
            fns: JPC_CollideShapeCollectorFns,
        ) -> Result<*mut JPC_CollideShapeCollector>;
        unsafe fn JPC_CollideShapeSettings_default(
            object: *mut JPC_CollideShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_ContactListener_delete(object: *mut JPC_ContactListener) -> Result<()>;
        unsafe fn JPC_ContactListener_new(
            self_: usize,
            fns: JPC_ContactListenerFns,
        ) -> Result<*mut JPC_ContactListener>;
        unsafe fn JPC_ConvexHullShapeSettings_Create(
            self_: *const JPC_ConvexHullShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_ConvexHullShapeSettings_default(
            object: *mut JPC_ConvexHullShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_CylinderShapeSettings_Create(
            self_: *const JPC_CylinderShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_CylinderShapeSettings_default(
            object: *mut JPC_CylinderShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_DebugRendererSimple_delete(
            object: *mut JPC_DebugRendererSimple,
        ) -> Result<()>;
        unsafe fn JPC_DebugRendererSimple_new(
            self_: usize,
            fns: JPC_DebugRendererSimpleFns,
        ) -> Result<*mut JPC_DebugRendererSimple>;
        unsafe fn JPC_FactoryDelete() -> Result<()>;
        unsafe fn JPC_FactoryInit() -> Result<()>;
        unsafe fn JPC_GroupFilter_delete(object: *mut JPC_GroupFilter) -> Result<()>;
        unsafe fn JPC_GroupFilter_new(
            self_: usize,
            fns: JPC_GroupFilterFns,
        ) -> Result<*mut JPC_GroupFilter>;
        unsafe fn JPC_HeightFieldShapeSettings_Create(
            self_: *const JPC_HeightFieldShapeSettings,
            arg1: *mut *mut JPC_Shape,
            arg2: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_HeightFieldShapeSettings_default(
            self_: *mut JPC_HeightFieldShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_HeightFieldShape_GetNoCollisionValue() -> Result<f32>;
        unsafe fn JPC_JobSystemThreadPool_delete(
            object: *mut JPC_JobSystemThreadPool,
        ) -> Result<()>;
        unsafe fn JPC_JobSystemThreadPool_new2(
            inMaxJobs: u32,
            inMaxBarriers: u32,
        ) -> Result<*mut JPC_JobSystemThreadPool>;
        unsafe fn JPC_MassProperties_default(properties: *mut JPC_MassProperties) -> Result<()>;
        unsafe fn JPC_MeshShapeSettings_Create(
            self_: *const JPC_MeshShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_MeshShapeSettings_default(object: *mut JPC_MeshShapeSettings) -> Result<()>;
        unsafe fn JPC_MutableCompoundShapeSettings_Create(
            self_: *const JPC_MutableCompoundShapeSettings,
            outShape: *mut *mut JPC_MutableCompoundShape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_MutableCompoundShapeSettings_default(
            object: *mut JPC_MutableCompoundShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_NarrowPhaseQuery_CastRay(
            self_: *const JPC_NarrowPhaseQuery,
            args: *mut JPC_NarrowPhaseQuery_CastRayArgs,
        ) -> Result<bool>;
        unsafe fn JPC_NarrowPhaseQuery_CastShape(
            self_: *const JPC_NarrowPhaseQuery,
            args: *mut JPC_NarrowPhaseQuery_CastShapeArgs,
        ) -> Result<()>;
        unsafe fn JPC_NarrowPhaseQuery_CollideShape(
            self_: *const JPC_NarrowPhaseQuery,
            args: *mut JPC_NarrowPhaseQuery_CollideShapeArgs,
        ) -> Result<()>;
        unsafe fn JPC_ObjectLayerFilter_delete(object: *mut JPC_ObjectLayerFilter) -> Result<()>;
        unsafe fn JPC_ObjectLayerFilter_new(
            self_: usize,
            fns: JPC_ObjectLayerFilterFns,
        ) -> Result<*mut JPC_ObjectLayerFilter>;
        unsafe fn JPC_ObjectLayerPairFilter_delete(
            object: *mut JPC_ObjectLayerPairFilter,
        ) -> Result<()>;
        unsafe fn JPC_ObjectLayerPairFilter_new(
            self_: usize,
            fns: JPC_ObjectLayerPairFilterFns,
        ) -> Result<*mut JPC_ObjectLayerPairFilter>;
        unsafe fn JPC_ObjectVsBroadPhaseLayerFilter_delete(
            object: *mut JPC_ObjectVsBroadPhaseLayerFilter,
        ) -> Result<()>;
        unsafe fn JPC_ObjectVsBroadPhaseLayerFilter_new(
            self_: usize,
            fns: JPC_ObjectVsBroadPhaseLayerFilterFns,
        ) -> Result<*mut JPC_ObjectVsBroadPhaseLayerFilter>;
        unsafe fn JPC_PhysicsSystem_DrawBodies(
            self_: *mut JPC_PhysicsSystem,
            inSettings: *mut JPC_BodyManager_DrawSettings,
            inRenderer: *mut JPC_DebugRendererSimple,
            inBodyFilter: usize,
        ) -> Result<()>;
        unsafe fn JPC_PhysicsSystem_delete(object: *mut JPC_PhysicsSystem) -> Result<()>;
        unsafe fn JPC_PhysicsSystem_new() -> Result<*mut JPC_PhysicsSystem>;
        unsafe fn JPC_PlaneShapeSettings_Create(
            self_: *const JPC_PlaneShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_PlaneShapeSettings_default(object: *mut JPC_PlaneShapeSettings)
            -> Result<()>;
        unsafe fn JPC_SetAssertFailed(fn_: usize) -> Result<()>;
        unsafe fn JPC_ShapeCastSettings_default(object: *mut JPC_ShapeCastSettings) -> Result<()>;
        unsafe fn JPC_ShapeFilter_delete(object: *mut JPC_ShapeFilter) -> Result<()>;
        unsafe fn JPC_ShapeFilter_getBodyID2(self_: *const JPC_ShapeFilter) -> Result<u32>;
        unsafe fn JPC_ShapeFilter_new(
            self_: usize,
            fns: JPC_ShapeFilterFns,
        ) -> Result<*mut JPC_ShapeFilter>;
        unsafe fn JPC_SimShapeFilter_delete(object: *mut JPC_SimShapeFilter) -> Result<()>;
        unsafe fn JPC_SimShapeFilter_new(
            self_: usize,
            fns: JPC_SimShapeFilterFns,
        ) -> Result<*mut JPC_SimShapeFilter>;
        unsafe fn JPC_SphereShapeSettings_Create(
            self_: *const JPC_SphereShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_SphereShapeSettings_default(
            object: *mut JPC_SphereShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_StaticCompoundShapeSettings_Create(
            self_: *const JPC_StaticCompoundShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_StaticCompoundShapeSettings_default(
            object: *mut JPC_StaticCompoundShapeSettings,
        ) -> Result<()>;
        unsafe fn JPC_String_c_str(self_: *mut JPC_String) -> Result<*const i8>;
        unsafe fn JPC_String_delete(self_: *mut JPC_String) -> Result<()>;
        unsafe fn JPC_SubShapeSettings_default(object: *mut JPC_SubShapeSettings) -> Result<()>;
        unsafe fn JPC_TempAllocatorImpl_delete(object: *mut JPC_TempAllocatorImpl) -> Result<()>;
        unsafe fn JPC_TempAllocatorImpl_new(size: u32) -> Result<*mut JPC_TempAllocatorImpl>;
        unsafe fn JPC_TriangleShapeSettings_Create(
            self_: *const JPC_TriangleShapeSettings,
            outShape: *mut *mut JPC_Shape,
            outError: *mut *mut JPC_String,
        ) -> Result<bool>;
        unsafe fn JPC_TriangleShapeSettings_default(
            object: *mut JPC_TriangleShapeSettings,
        ) -> Result<()>;
    }
}
