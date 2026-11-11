#pragma once

struct JPC_BodyCreationSettings;
struct JPC_BodyFilter;
struct JPC_BodyLockInterface;
struct JPC_BodyLockMultiRead;
struct JPC_BodyLockMultiWrite;
struct JPC_BodyLockRead;
struct JPC_BodyLockWrite;
struct JPC_BodyManager_DrawSettings;
struct JPC_BoxShapeSettings;
struct JPC_BroadPhaseLayerFilter;
struct JPC_CapsuleShapeSettings;
struct JPC_CollideShapeResult;
struct JPC_CollideShapeSettings;
struct JPC_CollisionEstimationResult;
struct JPC_CollisionGroup;
struct JPC_Color;
struct JPC_CompoundShape;
struct JPC_ConstraintSettings;
struct JPC_ContactManifold;
struct JPC_ContactPoints;
struct JPC_ContactSettings;
struct JPC_ConvexHullShapeSettings;
struct JPC_CylinderShapeSettings;
struct JPC_DMat44;
struct JPC_DVec3;
struct JPC_DebugRendererSimple;
struct JPC_DistanceConstraint;
struct JPC_DistanceConstraintSettings;
struct JPC_FixedConstraint;
struct JPC_FixedConstraintSettings;
struct JPC_Float3;
struct JPC_GroupFilter;
struct JPC_HeightFieldShapeSettings;
struct JPC_HingeConstraint;
struct JPC_HingeConstraintSettings;
struct JPC_IndexedTriangle;
struct JPC_IndexedTriangleList;
struct JPC_IndexedTriangleNoMaterial;
struct JPC_JobSystemSingleThreaded;
struct JPC_MassProperties;
struct JPC_Mat44;
struct JPC_MeshShapeSettings;
struct JPC_MotorSettings;
struct JPC_MutableCompoundShapeSettings;
struct JPC_NarrowPhaseQuery_CastRayArgs;
struct JPC_NarrowPhaseQuery_CastShapeArgs;
struct JPC_NarrowPhaseQuery_CollideShapeArgs;
struct JPC_ObjectLayerFilter;
struct JPC_PlaneShapeSettings;
struct JPC_Quat;
struct JPC_RRayCast;
struct JPC_RShapeCast;
struct JPC_RayCast;
struct JPC_RayCastResult;
struct JPC_ShapeCastResult;
struct JPC_ShapeCastSettings;
struct JPC_SixDOFConstraint;
struct JPC_SixDOFConstraintSettings;
struct JPC_SliderConstraint;
struct JPC_SliderConstraintSettings;
struct JPC_SphereShapeSettings;
struct JPC_SpringSettings;
struct JPC_StaticCompoundShapeSettings;
struct JPC_String;
struct JPC_SubShapeIDPair;
struct JPC_SubShapeSettings;
struct JPC_TriangleShapeSettings;
struct JPC_TwoBodyConstraint;
struct JPC_Vec2;
struct JPC_Vec3;
struct JPC_Vec4;
struct JPC_VertexList;

using uint = unsigned int;

typedef bool (*JPC_AssertFailedFunction)(const char *inExpression, const char *inMessage,
                                         const char *inFile, uint inLine);
typedef JPC_Vec3 JPC_RVec3;
typedef JPC_Mat44 JPC_RMat44;
typedef float Real;
typedef uint32_t JPC_BodyID;
typedef uint32_t JPC_SubShapeID;
typedef uint8_t JPC_BroadPhaseLayer;
typedef uint16_t JPC_ObjectLayer;
typedef uint32_t JPC_GroupID;
typedef uint32_t JPC_SubGroupID;

typedef struct JPC_GroupFilterFns {
    bool (*CanCollide)(const void *self, const JPC_CollisionGroup *inGroup1,
                       const JPC_CollisionGroup *inGroup2);
} JPC_GroupFilterFns;

typedef struct JPC_BroadPhaseLayerInterfaceFns {
    uint (*GetNumBroadPhaseLayers)(const void *self);
    JPC_BroadPhaseLayer (*GetBroadPhaseLayer)(const void *self, JPC_ObjectLayer inLayer);
} JPC_BroadPhaseLayerInterfaceFns;

typedef struct JPC_BroadPhaseLayerFilterFns {
    bool (*ShouldCollide)(const void *self, JPC_BroadPhaseLayer inLayer);
} JPC_BroadPhaseLayerFilterFns;

typedef struct JPC_ObjectLayerFilterFns {
    bool (*ShouldCollide)(const void *self, JPC_ObjectLayer inLayer);
} JPC_ObjectLayerFilterFns;

typedef struct JPC_BodyFilterFns {
    bool (*ShouldCollide)(const void *self, JPC_BodyID inBodyID);
    bool (*ShouldCollideLocked)(const void *self, const JPC_Body *inBodyID);
} JPC_BodyFilterFns;

typedef struct JPC_ShapeFilterFns {
    bool (*ShouldCollide)(const void *self, const JPC_ShapeFilter *filter,
                          const JPC_Shape *inShape2, JPC_SubShapeID inSubShapeIDOfShape2);

    bool (*ShouldCollideTwoShapes)(const void *self, const JPC_ShapeFilter *filter,
                                   const JPC_Shape *inShape1, JPC_SubShapeID inSubShapeIDOfShape1,
                                   const JPC_Shape *inShape2, JPC_SubShapeID inSubShapeIDOfShape2);
} JPC_ShapeFilterFns;

typedef struct JPC_SimShapeFilterFns {
    bool (*ShouldCollide)(const void *self, const JPC_Body *inBody1, const JPC_Shape *inShape1,
                          JPC_SubShapeID inSubShapeIDOfShape1, const JPC_Body *inBody2,
                          const JPC_Shape *inShape2, JPC_SubShapeID inSubShapeIDOfShape2);
} JPC_SimShapeFilterFns;

typedef struct JPC_ObjectVsBroadPhaseLayerFilterFns {
    bool (*ShouldCollide)(const void *self, JPC_ObjectLayer inLayer1, JPC_BroadPhaseLayer inLayer2);
} JPC_ObjectVsBroadPhaseLayerFilterFns;

typedef struct JPC_ObjectLayerPairFilterFns {
    bool (*ShouldCollide)(const void *self, JPC_ObjectLayer inLayer1, JPC_ObjectLayer inLayer2);
} JPC_ObjectLayerPairFilterFns;

typedef struct JPC_ContactListenerFns {
    JPC_ValidateResult (*OnContactValidate)(void *self, const JPC_Body *inBody1,
                                            const JPC_Body *inBody2, JPC_RVec3 inBaseOffset,
                                            const JPC_CollideShapeResult *inCollisionResult);

    void (*OnContactAdded)(void *self, const JPC_Body *inBody1, const JPC_Body *inBody2,
                           const JPC_ContactManifold *inManifold, JPC_ContactSettings *ioSettings);

    void (*OnContactPersisted)(void *self, const JPC_Body *inBody1, const JPC_Body *inBody2,
                               const JPC_ContactManifold *inManifold,
                               JPC_ContactSettings *ioSettings);

    void (*OnContactRemoved)(void *self, const JPC_SubShapeIDPair *inSubShapePair);
} JPC_ContactListenerFns;

typedef struct JPC_CastShapeCollectorFns {
    void (*Reset)(void *self);
    void (*AddHit)(void *self, JPC_CastShapeCollector *base, const JPC_ShapeCastResult *Result);
} JPC_CastShapeCollectorFns;

typedef struct JPC_CollideShapeCollectorFns {
    void (*Reset)(void *self);
    void (*AddHit)(void *self, JPC_CollideShapeCollector *base,
                   const JPC_CollideShapeResult *Result);
} JPC_CollideShapeCollectorFns;

typedef struct JPC_DebugRendererSimpleFns {
    void (*DrawLine)(const void *self, JPC_RVec3 inFrom, JPC_RVec3 inTo, JPC_Color inColor);
} JPC_DebugRendererSimpleFns;

void JPC_FactoryInit();
void JPC_FactoryDelete();
void JPC_SetAssertFailed(uintptr_t fn);
JPC_TempAllocatorImpl *JPC_TempAllocatorImpl_new(uint size);
void JPC_TempAllocatorImpl_delete(JPC_TempAllocatorImpl *object);
JPC_JobSystemThreadPool *JPC_JobSystemThreadPool_new2(uint inMaxJobs, uint inMaxBarriers);
void JPC_JobSystemThreadPool_delete(JPC_JobSystemThreadPool *object);
JPC_GroupFilter *JPC_GroupFilter_new(uintptr_t self, JPC_GroupFilterFns fns);
void JPC_GroupFilter_delete(JPC_GroupFilter *object);
JPC_BroadPhaseLayerInterface *JPC_BroadPhaseLayerInterface_new(uintptr_t self,
                                                               JPC_BroadPhaseLayerInterfaceFns fns);
void JPC_BroadPhaseLayerInterface_delete(JPC_BroadPhaseLayerInterface *object);
JPC_BroadPhaseLayerFilter *JPC_BroadPhaseLayerFilter_new(uintptr_t self,
                                                         JPC_BroadPhaseLayerFilterFns fns);
void JPC_BroadPhaseLayerFilter_delete(JPC_BroadPhaseLayerFilter *object);
JPC_ObjectLayerFilter *JPC_ObjectLayerFilter_new(uintptr_t self, JPC_ObjectLayerFilterFns fns);
void JPC_ObjectLayerFilter_delete(JPC_ObjectLayerFilter *object);
JPC_BodyFilter *JPC_BodyFilter_new(uintptr_t self, JPC_BodyFilterFns fns);
void JPC_BodyFilter_delete(JPC_BodyFilter *object);
JPC_ShapeFilter *JPC_ShapeFilter_new(uintptr_t self, JPC_ShapeFilterFns fns);
JPC_BodyID JPC_ShapeFilter_getBodyID2(const JPC_ShapeFilter *self);
void JPC_ShapeFilter_delete(JPC_ShapeFilter *object);
JPC_SimShapeFilter *JPC_SimShapeFilter_new(uintptr_t self, JPC_SimShapeFilterFns fns);
void JPC_SimShapeFilter_delete(JPC_SimShapeFilter *object);
JPC_ObjectVsBroadPhaseLayerFilter *
JPC_ObjectVsBroadPhaseLayerFilter_new(uintptr_t self, JPC_ObjectVsBroadPhaseLayerFilterFns fns);
void JPC_ObjectVsBroadPhaseLayerFilter_delete(JPC_ObjectVsBroadPhaseLayerFilter *object);
JPC_ObjectLayerPairFilter *JPC_ObjectLayerPairFilter_new(uintptr_t self,
                                                         JPC_ObjectLayerPairFilterFns fns);
void JPC_ObjectLayerPairFilter_delete(JPC_ObjectLayerPairFilter *object);
JPC_ContactListener *JPC_ContactListener_new(uintptr_t self, JPC_ContactListenerFns fns);
void JPC_ContactListener_delete(JPC_ContactListener *object);
JPC_CastShapeCollector *JPC_CastShapeCollector_new(uintptr_t self, JPC_CastShapeCollectorFns fns);
void JPC_CastShapeCollector_delete(JPC_CastShapeCollector *object);
JPC_CollideShapeCollector *JPC_CollideShapeCollector_new(uintptr_t self,
                                                         JPC_CollideShapeCollectorFns fns);
void JPC_CollideShapeCollector_delete(JPC_CollideShapeCollector *object);
void JPC_BodyManager_DrawSettings_default(JPC_BodyManager_DrawSettings *object);
JPC_DebugRendererSimple *JPC_DebugRendererSimple_new(uintptr_t self,
                                                     JPC_DebugRendererSimpleFns fns);
void JPC_DebugRendererSimple_delete(JPC_DebugRendererSimple *object);
void JPC_String_delete(JPC_String *self);
const int8_t *JPC_String_c_str(JPC_String *self);
void JPC_TriangleShapeSettings_default(JPC_TriangleShapeSettings *object);
bool JPC_TriangleShapeSettings_Create(const JPC_TriangleShapeSettings *self, JPC_Shape **outShape,
                                      JPC_String **outError);
void JPC_MeshShapeSettings_default(JPC_MeshShapeSettings *object);
bool JPC_MeshShapeSettings_Create(const JPC_MeshShapeSettings *self, JPC_Shape **outShape,
                                  JPC_String **outError);
void JPC_BoxShapeSettings_default(JPC_BoxShapeSettings *object);
bool JPC_BoxShapeSettings_Create(const JPC_BoxShapeSettings *self, JPC_Shape **outShape,
                                 JPC_String **outError);
void JPC_SphereShapeSettings_default(JPC_SphereShapeSettings *object);
bool JPC_SphereShapeSettings_Create(const JPC_SphereShapeSettings *self, JPC_Shape **outShape,
                                    JPC_String **outError);
void JPC_CapsuleShapeSettings_default(JPC_CapsuleShapeSettings *object);
bool JPC_CapsuleShapeSettings_Create(const JPC_CapsuleShapeSettings *self, JPC_Shape **outShape,
                                     JPC_String **outError);
void JPC_CylinderShapeSettings_default(JPC_CylinderShapeSettings *object);
bool JPC_CylinderShapeSettings_Create(const JPC_CylinderShapeSettings *self, JPC_Shape **outShape,
                                      JPC_String **outError);
void JPC_PlaneShapeSettings_default(JPC_PlaneShapeSettings *object);
bool JPC_PlaneShapeSettings_Create(const JPC_PlaneShapeSettings *self, JPC_Shape **outShape,
                                   JPC_String **outError);
void JPC_ConvexHullShapeSettings_default(JPC_ConvexHullShapeSettings *object);
bool JPC_ConvexHullShapeSettings_Create(const JPC_ConvexHullShapeSettings *self,
                                        JPC_Shape **outShape, JPC_String **outError);
void JPC_SubShapeSettings_default(JPC_SubShapeSettings *object);
void JPC_StaticCompoundShapeSettings_default(JPC_StaticCompoundShapeSettings *object);
bool JPC_StaticCompoundShapeSettings_Create(const JPC_StaticCompoundShapeSettings *self,
                                            JPC_Shape **outShape, JPC_String **outError);
void JPC_MutableCompoundShapeSettings_default(JPC_MutableCompoundShapeSettings *object);
bool JPC_MutableCompoundShapeSettings_Create(const JPC_MutableCompoundShapeSettings *self,
                                             JPC_MutableCompoundShape **outShape,
                                             JPC_String **outError);
void JPC_MassProperties_default(JPC_MassProperties *properties);
void JPC_BodyCreationSettings_default(JPC_BodyCreationSettings *settings);
bool JPC_NarrowPhaseQuery_CastRay(const JPC_NarrowPhaseQuery *self,
                                  JPC_NarrowPhaseQuery_CastRayArgs *args);
void JPC_ShapeCastSettings_default(JPC_ShapeCastSettings *object);
void JPC_NarrowPhaseQuery_CastShape(const JPC_NarrowPhaseQuery *self,
                                    JPC_NarrowPhaseQuery_CastShapeArgs *args);
void JPC_CollideShapeSettings_default(JPC_CollideShapeSettings *object);
void JPC_NarrowPhaseQuery_CollideShape(const JPC_NarrowPhaseQuery *self,
                                       JPC_NarrowPhaseQuery_CollideShapeArgs *args);
JPC_PhysicsSystem *JPC_PhysicsSystem_new();
void JPC_PhysicsSystem_delete(JPC_PhysicsSystem *object);
void JPC_PhysicsSystem_DrawBodies(JPC_PhysicsSystem *self, JPC_BodyManager_DrawSettings *inSettings,
                                  JPC_DebugRendererSimple *inRenderer, // FIXME: un-specialize
                                  uintptr_t inBodyFilter);
void JPC_HeightFieldShapeSettings_default(JPC_HeightFieldShapeSettings *self_);
bool JPC_HeightFieldShapeSettings_Create(const JPC_HeightFieldShapeSettings *self_,
                                         JPC_Shape **arg1, JPC_String **arg2);
float JPC_HeightFieldShape_GetNoCollisionValue(void);
