#pragma once
// Actual Jolt headers used by the handwritten #[cxx::bridge] declarations.
#include <Jolt/Jolt.h>
#include <Jolt/RegisterTypes.h>
#include <Jolt/Core/TempAllocator.h>
#include <Jolt/Core/JobSystemThreadPool.h>
#include <Jolt/Physics/PhysicsSystem.h>
#include <Jolt/Physics/Body/BodyCreationSettings.h>
#include <Jolt/Physics/Collision/Shape/MutableCompoundShape.h>
#include <Jolt/Physics/Collision/Shape/HeightFieldShape.h>
#include <Jolt/Physics/Collision/ShapeCast.h>
#include <Jolt/Physics/Collision/SimShapeFilter.h>

// cxx cannot express C++ inheritance conversions.
namespace rolt_native {
inline JPH::TempAllocator *allocator_base(JPH::TempAllocatorImpl *p) { return p; }
} // namespace rolt_native
// Verify the Rust trivial native-value representations on every compilation.
static_assert(sizeof(JPH::BodyID) == 4 && alignof(JPH::BodyID) == 4);
static_assert(sizeof(JPH::EActivation) == 4 && alignof(JPH::EActivation) == 4);
static_assert(sizeof(JPH::EPhysicsUpdateError) == 4 && alignof(JPH::EPhysicsUpdateError) == 4);
static_assert(sizeof(JPH::Vec3) == 16 && alignof(JPH::Vec3) == 16);
static_assert(sizeof(JPH::Quat) == 16 && alignof(JPH::Quat) == 16);
static_assert(static_cast<unsigned>(JPH::EActivation::Activate) == 0);
static_assert(static_cast<unsigned>(JPH::EActivation::DontActivate) == 1);
