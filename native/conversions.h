// Conversions required by public rolt value/callback types. Not generated.
#pragma once
#include <Jolt/Jolt.h>
#include "bridge.h"
#include "rolt/src/raw/bridge.rs.h"
#include <Jolt/RegisterTypes.h>
#include <Jolt/Core/Factory.h>
#include <Jolt/Core/JobSystemThreadPool.h>
#include <Jolt/Core/TempAllocator.h>
#include <Jolt/Physics/PhysicsSystem.h>
#include <Jolt/Physics/Body/BodyCreationSettings.h>
#include <Jolt/Physics/Collision/Shape/BoxShape.h>
#include <Jolt/Physics/Collision/Shape/SphereShape.h>
#include <Jolt/Physics/Collision/Shape/CapsuleShape.h>
#include <Jolt/Physics/Collision/Shape/CylinderShape.h>
#include <Jolt/Physics/Collision/Shape/TriangleShape.h>
#include <Jolt/Physics/Collision/Shape/PlaneShape.h>
#include <Jolt/Physics/Collision/Shape/ConvexHullShape.h>
#include <Jolt/Physics/Collision/Shape/MeshShape.h>
#include <Jolt/Physics/Collision/Shape/StaticCompoundShape.h>
#include <Jolt/Physics/Collision/Shape/MutableCompoundShape.h>
#include <Jolt/Physics/Collision/Shape/HeightFieldShape.h>
#include <Jolt/Physics/Collision/RayCast.h>
#include <Jolt/Physics/Collision/CastResult.h>
#include <Jolt/Physics/Collision/CollisionCollectorImpl.h>
#include <Jolt/Physics/Collision/ShapeCast.h>
#include <Jolt/Physics/Collision/SimShapeFilter.h>
#include <Jolt/Renderer/DebugRendererSimple.h>
#include <string>
#include <stdexcept>

namespace compat {
// Only opaque pointers cross by address. All public value types are converted field by field.
template <class T, class U> T *native(U *p) { return reinterpret_cast<T *>(p); }
template <class T, class U> const T *native(const U *p) { return reinterpret_cast<const T *>(p); }
template <class T, class U> T *foreign(U *p) { return reinterpret_cast<T *>(p); }
inline JPH::Vec3 vec(JPC_Vec3 v) { return {v.x, v.y, v.z}; }
inline JPC_Vec3 vec(JPH::Vec3Arg v) { return {v.GetX(), v.GetY(), v.GetZ(), v.GetZ()}; }
inline JPH::Quat quat(JPC_Quat q) { return {q.x, q.y, q.z, q.w}; }
inline JPC_Quat quat(JPH::QuatArg q) { return {q.GetX(), q.GetY(), q.GetZ(), q.GetW()}; }
inline JPH::Mat44 mat(JPC_Mat44 m) {
    return {JPH::Vec4(m.col[0].x, m.col[0].y, m.col[0].z, m.col[0].w),
            JPH::Vec4(m.col[1].x, m.col[1].y, m.col[1].z, m.col[1].w),
            JPH::Vec4(m.col[2].x, m.col[2].y, m.col[2].z, m.col[2].w),
            JPH::Vec4(m.col3.x, m.col3.y, m.col3.z, 1)};
}
inline JPC_Mat44 mat(JPH::Mat44Arg m) {
    JPC_Mat44 r{};
    for (int i = 0; i < 3; ++i) {
        auto c = m.GetColumn4(i);
        r.col[i] = {c.GetX(), c.GetY(), c.GetZ(), c.GetW()};
    }
    r.col3 = vec(m.GetTranslation());
    return r;
}

inline JPC_CollideShapeResult result(const JPH::CollideShapeResult &h) {
    return {vec(h.mContactPointOn1),
            vec(h.mContactPointOn2),
            vec(h.mPenetrationAxis),
            h.mPenetrationDepth,
            h.mSubShapeID1.GetValue(),
            h.mSubShapeID2.GetValue(),
            h.mBodyID2.GetIndexAndSequenceNumber()};
}
inline JPC_ShapeCastResult result(const JPH::ShapeCastResult &h) {
    JPC_ShapeCastResult r{};
    r.ContactPointOn1 = vec(h.mContactPointOn1);
    r.ContactPointOn2 = vec(h.mContactPointOn2);
    r.PenetrationAxis = vec(h.mPenetrationAxis);
    r.PenetrationDepth = h.mPenetrationDepth;
    r.SubShapeID1 = h.mSubShapeID1.GetValue();
    r.SubShapeID2 = h.mSubShapeID2.GetValue();
    r.BodyID2 = h.mBodyID2.GetIndexAndSequenceNumber();
    r.Fraction = h.mFraction;
    r.IsBackFaceHit = h.mIsBackFaceHit;
    return r;
}

} // namespace compat
