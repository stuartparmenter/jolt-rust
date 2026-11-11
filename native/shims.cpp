// Shims for constructors, public settings and query templates that cxx cannot
// express directly. Ordinary body/world operations bind directly in src/jolt.rs.
#include "conversions.h"
namespace compat {
void JPC_FactoryInit() { JPH::Factory::sInstance = new JPH::Factory(); }
void JPC_FactoryDelete() {
    delete JPH::Factory::sInstance;
    JPH::Factory::sInstance = nullptr;
}

void JPC_SetAssertFailed(uintptr_t f) {
#ifdef JPH_ENABLE_ASSERTS
    JPH::AssertFailed = reinterpret_cast<JPC_AssertFailedFunction>(f);
#else
    (void)f;
#endif
}
JPC_TempAllocatorImpl *JPC_TempAllocatorImpl_new(unsigned n) {
    return new JPH::TempAllocatorImpl(n);
}
void JPC_TempAllocatorImpl_delete(JPC_TempAllocatorImpl *p) { delete p; }
JPC_JobSystemThreadPool *JPC_JobSystemThreadPool_new2(unsigned jobs, unsigned barriers) {
    return new JPH::JobSystemThreadPool(jobs, barriers);
}
void JPC_JobSystemThreadPool_delete(JPC_JobSystemThreadPool *p) { delete p; }
const int8_t *JPC_String_c_str(JPC_String *p) {
    return reinterpret_cast<const int8_t *>(native<std::string>(p)->c_str());
}
void JPC_String_delete(JPC_String *p) { delete native<std::string>(p); }

template <class T> bool create_shape(const JPH::ShapeSettings &s, T **out, JPC_String **error) {
    *out = nullptr;
    if (error)
        *error = nullptr;
    auto r = s.Create();
    if (r.HasError()) {
        if (error)
            *error = foreign<JPC_String>(new std::string(r.GetError().c_str()));
        return false;
    }
    auto *p = r.Get().GetPtr();
    p->AddRef();
    *out = foreign<T>(p);
    return true;
}
// Public settings are cxx shared records, separate from Jolt's native classes.
#include "settings.inc"

float JPC_HeightFieldShape_GetNoCollisionValue() {
    return JPH::HeightFieldShapeConstants::cNoCollisionValue;
}
JPC_PhysicsSystem *JPC_PhysicsSystem_new() { return new JPH::PhysicsSystem(); }
void JPC_PhysicsSystem_delete(JPC_PhysicsSystem *p) { delete p; }

void JPC_PhysicsSystem_DrawBodies(JPC_PhysicsSystem *p, JPC_BodyManager_DrawSettings *s,
                                  JPC_DebugRendererSimple *r, uintptr_t f) {
    p->DrawBodies(draw_settings(*s), native<JPH::DebugRendererSimple>(r),
                  reinterpret_cast<const JPH::BodyDrawFilter *>(f));
}
std::unique_ptr<JPH::BodyCreationSettings> native_body_settings(const JPC_BodyCreationSettings &s) {
    return std::make_unique<JPH::BodyCreationSettings>(body_settings(s));
}

// ShapeFilter carries mutable per-query state, so defaults must be local to
// each invocation, including queries nested inside callbacks.
struct QueryFilters {
    JPH::BroadPhaseLayerFilter broad;
    JPH::ObjectLayerFilter object;
    JPH::BodyFilter body;
    JPH::ShapeFilter shape;
};
template <class T, class P> const T &filter(P *p, const T &fallback) {
    return p ? *native<T>(p) : fallback;
}
bool JPC_NarrowPhaseQuery_CastRay(const JPC_NarrowPhaseQuery *p,
                                  JPC_NarrowPhaseQuery_CastRayArgs *a) {
    const QueryFilters defaults;
    JPH::ClosestHitCollisionCollector<JPH::CastRayCollector> collector;
    p->CastRay(JPH::RRayCast(vec(a->Ray.Origin), vec(a->Ray.Direction)), JPH::RayCastSettings(),
               collector, filter(a->BroadPhaseLayerFilter, defaults.broad),
               filter(a->ObjectLayerFilter, defaults.object), filter(a->BodyFilter, defaults.body),
               filter(a->ShapeFilter, defaults.shape));
    if (!collector.HadHit())
        return false;
    auto &r = collector.mHit;
    a->Result = {r.mBodyID.GetIndexAndSequenceNumber(), r.mFraction, r.mSubShapeID2.GetValue()};
    return true;
}
void JPC_NarrowPhaseQuery_CastShape(const JPC_NarrowPhaseQuery *p,
                                    JPC_NarrowPhaseQuery_CastShapeArgs *a) {
    const QueryFilters defaults;
    auto &s = a->ShapeCast;
    JPH::RShapeCast cast(s.Shape, vec(s.Scale), mat(s.CenterOfMassStart), vec(s.Direction));
    p->CastShape(cast, settings(a->Settings), vec(a->BaseOffset), *a->Collector,
                 filter(a->BroadPhaseLayerFilter, defaults.broad),
                 filter(a->ObjectLayerFilter, defaults.object),
                 filter(a->BodyFilter, defaults.body), filter(a->ShapeFilter, defaults.shape));
}
void JPC_NarrowPhaseQuery_CollideShape(const JPC_NarrowPhaseQuery *p,
                                       JPC_NarrowPhaseQuery_CollideShapeArgs *a) {
    const QueryFilters defaults;
    p->CollideShape(a->Shape, vec(a->ShapeScale), mat(a->CenterOfMassTransform),
                    settings(a->Settings), vec(a->BaseOffset), *a->Collector,
                    filter(a->BroadPhaseLayerFilter, defaults.broad),
                    filter(a->ObjectLayerFilter, defaults.object),
                    filter(a->BodyFilter, defaults.body), filter(a->ShapeFilter, defaults.shape));
}

} // namespace compat
