// C++ virtual subclasses for the existing Rust callback traits. Not generated.
// cxx does not implement C++ virtual interfaces on behalf of Rust.
#include "conversions.h"
namespace compat {
#define ADAPTER(Name, Mut)                                                                         \
    struct Name##Adapter final : JPH::Name {                                                       \
        Mut void *self;                                                                            \
        JPC_##Name##Fns f;                                                                         \
        Name##Adapter(Mut void *s, JPC_##Name##Fns v) : self(s), f(v) {}
#define ADAPTER_END(Name, Mut)                                                                     \
    }                                                                                              \
    ;                                                                                              \
    JPC_##Name *JPC_##Name##_new(uintptr_t s, JPC_##Name##Fns f) {                                 \
        return foreign<JPC_##Name>(                                                                \
            static_cast<JPH::Name *>(new Name##Adapter(reinterpret_cast<Mut void *>(s), f)));      \
    }                                                                                              \
    void JPC_##Name##_delete(JPC_##Name *p) { delete native<JPH::Name>(p); }
ADAPTER(BroadPhaseLayerInterface, const)
unsigned GetNumBroadPhaseLayers() const override { return f.GetNumBroadPhaseLayers(self); }
JPH::BroadPhaseLayer GetBroadPhaseLayer(JPH::ObjectLayer l) const override {
    return JPH::BroadPhaseLayer(f.GetBroadPhaseLayer(self, l));
}
ADAPTER_END(BroadPhaseLayerInterface, const)
ADAPTER(ObjectVsBroadPhaseLayerFilter, const)
bool ShouldCollide(JPH::ObjectLayer a, JPH::BroadPhaseLayer b) const override {
    return f.ShouldCollide(self, a, b.GetValue());
}
ADAPTER_END(ObjectVsBroadPhaseLayerFilter, const)
ADAPTER(ObjectLayerPairFilter, const)
bool ShouldCollide(JPH::ObjectLayer a, JPH::ObjectLayer b) const override {
    return f.ShouldCollide(self, a, b);
}
ADAPTER_END(ObjectLayerPairFilter, const)
ADAPTER(BroadPhaseLayerFilter, const)
bool ShouldCollide(JPH::BroadPhaseLayer b) const override {
    return f.ShouldCollide(self, b.GetValue());
}
ADAPTER_END(BroadPhaseLayerFilter, const)
ADAPTER(ObjectLayerFilter, const)
bool ShouldCollide(JPH::ObjectLayer b) const override { return f.ShouldCollide(self, b); }
ADAPTER_END(ObjectLayerFilter, const)
ADAPTER(BodyFilter, const)
bool ShouldCollide(const JPH::BodyID &b) const override {
    return f.ShouldCollide(self, b.GetIndexAndSequenceNumber());
}
bool ShouldCollideLocked(const JPH::Body &b) const override {
    return f.ShouldCollideLocked(self, foreign<const JPC_Body>(&b));
}
ADAPTER_END(BodyFilter, const)
ADAPTER(ShapeFilter, const)
bool ShouldCollide(const JPH::Shape *s, const JPH::SubShapeID &id) const override {
    return f.ShouldCollide(
        self, foreign<const JPC_ShapeFilter>(static_cast<const JPH::ShapeFilter *>(this)),
        foreign<const JPC_Shape>(s), id.GetValue());
}
bool ShouldCollide(const JPH::Shape *a, const JPH::SubShapeID &ai, const JPH::Shape *b,
                   const JPH::SubShapeID &bi) const override {
    return f.ShouldCollideTwoShapes(
        self, foreign<const JPC_ShapeFilter>(static_cast<const JPH::ShapeFilter *>(this)),
        foreign<const JPC_Shape>(a), ai.GetValue(), foreign<const JPC_Shape>(b), bi.GetValue());
}
ADAPTER_END(ShapeFilter, const)
JPC_BodyID JPC_ShapeFilter_getBodyID2(const JPC_ShapeFilter *p) {
    return native<JPH::ShapeFilter>(p)->mBodyID2.GetIndexAndSequenceNumber();
}
ADAPTER(SimShapeFilter, const)
bool ShouldCollide(const JPH::Body &a, const JPH::Shape *as, const JPH::SubShapeID &ai,
                   const JPH::Body &b, const JPH::Shape *bs,
                   const JPH::SubShapeID &bi) const override {
    return f.ShouldCollide(self, foreign<const JPC_Body>(&a), foreign<const JPC_Shape>(as),
                           ai.GetValue(), foreign<const JPC_Body>(&b), foreign<const JPC_Shape>(bs),
                           bi.GetValue());
}
ADAPTER_END(SimShapeFilter, const)
ADAPTER(GroupFilter, const)
bool CanCollide(const JPH::CollisionGroup &a, const JPH::CollisionGroup &b) const override {
    JPC_CollisionGroup ga{foreign<const JPC_GroupFilter>(a.GetGroupFilter()), a.GetGroupID(),
                          a.GetSubGroupID()};
    JPC_CollisionGroup gb{foreign<const JPC_GroupFilter>(b.GetGroupFilter()), b.GetGroupID(),
                          b.GetSubGroupID()};
    return f.CanCollide(self, &ga, &gb);
}
ADAPTER_END(GroupFilter, const)
ADAPTER(CastShapeCollector, )
void Reset() override {
    JPH::CastShapeCollector::Reset();
    f.Reset(self);
}
void AddHit(const JPH::ShapeCastResult &h) override {
    auto r = result(h);
    f.AddHit(self, foreign<JPC_CastShapeCollector>(static_cast<JPH::CastShapeCollector *>(this)),
             &r);
}
ADAPTER_END(CastShapeCollector, )
ADAPTER(CollideShapeCollector, )
void Reset() override {
    JPH::CollideShapeCollector::Reset();
    f.Reset(self);
}
void AddHit(const JPH::CollideShapeResult &h) override {
    auto r = result(h);
    f.AddHit(self,
             foreign<JPC_CollideShapeCollector>(static_cast<JPH::CollideShapeCollector *>(this)),
             &r);
}
ADAPTER_END(CollideShapeCollector, )

ADAPTER(ContactListener, )
JPH::ValidateResult OnContactValidate(const JPH::Body &a, const JPH::Body &b, JPH::RVec3Arg offset,
                                      const JPH::CollideShapeResult &h) override {
    auto r = result(h);
    return JPH::ValidateResult(f.OnContactValidate(self, foreign<const JPC_Body>(&a),
                                                   foreign<const JPC_Body>(&b), vec(offset), &r));
}
void event(bool added, const JPH::Body &a, const JPH::Body &b, const JPH::ContactManifold &m,
           JPH::ContactSettings &s) {
    JPC_ContactManifold cm{};
    cm.BaseOffset = vec(m.mBaseOffset);
    cm.WorldSpaceNormal = vec(m.mWorldSpaceNormal);
    cm.PenetrationDepth = m.mPenetrationDepth;
    cm.SubShapeID1 = m.mSubShapeID1.GetValue();
    cm.SubShapeID2 = m.mSubShapeID2.GetValue();
    cm.RelativeContactPointsOn1.length = m.mRelativeContactPointsOn1.size();
    cm.RelativeContactPointsOn2.length = m.mRelativeContactPointsOn2.size();
    for (unsigned i = 0; i < cm.RelativeContactPointsOn1.length; ++i)
        cm.RelativeContactPointsOn1.points[i] = vec(m.mRelativeContactPointsOn1[i]);
    for (unsigned i = 0; i < cm.RelativeContactPointsOn2.length; ++i)
        cm.RelativeContactPointsOn2.points[i] = vec(m.mRelativeContactPointsOn2[i]);
    JPC_ContactSettings cs{s.mCombinedFriction,
                           s.mCombinedRestitution,
                           s.mInvMassScale1,
                           s.mInvInertiaScale1,
                           s.mInvMassScale2,
                           s.mInvInertiaScale2,
                           s.mIsSensor,
                           vec(s.mRelativeLinearSurfaceVelocity),
                           vec(s.mRelativeAngularSurfaceVelocity)};
    auto cb = added ? f.OnContactAdded : f.OnContactPersisted;
    cb(self, foreign<const JPC_Body>(&a), foreign<const JPC_Body>(&b), &cm, &cs);
    s.mCombinedFriction = cs.CombinedFriction;
    s.mCombinedRestitution = cs.CombinedRestitution;
    s.mInvMassScale1 = cs.InvMassScale1;
    s.mInvInertiaScale1 = cs.InvInertiaScale1;
    s.mInvMassScale2 = cs.InvMassScale2;
    s.mInvInertiaScale2 = cs.InvInertiaScale2;
    s.mIsSensor = cs.IsSensor;
    s.mRelativeLinearSurfaceVelocity = vec(cs.RelativeLinearSurfaceVelocity);
    s.mRelativeAngularSurfaceVelocity = vec(cs.RelativeAngularSurfaceVelocity);
}
void OnContactAdded(const JPH::Body &a, const JPH::Body &b, const JPH::ContactManifold &m,
                    JPH::ContactSettings &s) override {
    event(true, a, b, m, s);
}
void OnContactPersisted(const JPH::Body &a, const JPH::Body &b, const JPH::ContactManifold &m,
                        JPH::ContactSettings &s) override {
    event(false, a, b, m, s);
}
void OnContactRemoved(const JPH::SubShapeIDPair &p) override {
    JPC_SubShapeIDPair pair{
        p.GetBody1ID().GetIndexAndSequenceNumber(), p.GetSubShapeID1().GetValue(),
        p.GetBody2ID().GetIndexAndSequenceNumber(), p.GetSubShapeID2().GetValue()};
    f.OnContactRemoved(self, &pair);
}
ADAPTER_END(ContactListener, )

struct Renderer final : JPH::DebugRendererSimple {
    const void *self;
    JPC_DebugRendererSimpleFns f;
    Renderer(const void *s, JPC_DebugRendererSimpleFns v) : self(s), f(v) {}
    void DrawLine(JPH::RVec3Arg a, JPH::RVec3Arg b, JPH::ColorArg color) override {
        f.DrawLine(self, vec(a), vec(b), {color.r, color.g, color.b, color.a});
    }
    // The retained callback contract only exposes lines, as in JoltC.
    void DrawText3D(JPH::RVec3Arg, const JPH::string_view &, JPH::ColorArg, float) override {}
};
JPC_DebugRendererSimple *JPC_DebugRendererSimple_new(uintptr_t s, JPC_DebugRendererSimpleFns f) {
    return foreign<JPC_DebugRendererSimple>(static_cast<JPH::DebugRendererSimple *>(
        new Renderer(reinterpret_cast<const void *>(s), f)));
}
void JPC_DebugRendererSimple_delete(JPC_DebugRendererSimple *p) {
    delete native<JPH::DebugRendererSimple>(p);
}

} // namespace compat
