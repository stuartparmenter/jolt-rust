#pragma once
#include "jolt.h"
#include <cstdint>
#include <cstddef>
#include <memory>
namespace compat {

using JPC_Body = JPH::Body;
using JPC_BodyInterface = JPH::BodyInterface;
using JPC_PhysicsSystem = JPH::PhysicsSystem;
using JPC_Shape = JPH::Shape;
using JPC_MutableCompoundShape = JPH::MutableCompoundShape;
using JPC_HeightFieldShape = JPH::HeightFieldShape;
using JPC_NarrowPhaseQuery = JPH::NarrowPhaseQuery;
using JPC_Constraint = JPH::Constraint;
using JPC_BroadPhaseLayerInterface = JPH::BroadPhaseLayerInterface;
using JPC_ObjectVsBroadPhaseLayerFilter = JPH::ObjectVsBroadPhaseLayerFilter;
using JPC_ObjectLayerPairFilter = JPH::ObjectLayerPairFilter;
using JPC_ContactListener = JPH::ContactListener;
using JPC_SimShapeFilter = JPH::SimShapeFilter;
using JPC_CastShapeCollector = JPH::CastShapeCollector;
using JPC_CollideShapeCollector = JPH::CollideShapeCollector;
using JPC_ShapeFilter = JPH::ShapeFilter;
using JPC_TempAllocatorImpl = JPH::TempAllocatorImpl;
using JPC_JobSystem = JPH::JobSystem;
using JPC_JobSystemThreadPool = JPH::JobSystemThreadPool;

#include "api/Enums.h"
#include "types.h"
std::unique_ptr<JPH::BodyCreationSettings> native_body_settings(const JPC_BodyCreationSettings &);
} // namespace compat
static_assert(static_cast<unsigned long long>(compat::JPC_MOTION_TYPE_STATIC) ==
                  static_cast<unsigned long long>(JPH::EMotionType::Static),
              "Native enum changed: JPC_MOTION_TYPE_STATIC");
static_assert(static_cast<unsigned long long>(compat::JPC_MOTION_TYPE_KINEMATIC) ==
                  static_cast<unsigned long long>(JPH::EMotionType::Kinematic),
              "Native enum changed: JPC_MOTION_TYPE_KINEMATIC");
static_assert(static_cast<unsigned long long>(compat::JPC_MOTION_TYPE_DYNAMIC) ==
                  static_cast<unsigned long long>(JPH::EMotionType::Dynamic),
              "Native enum changed: JPC_MOTION_TYPE_DYNAMIC");
static_assert(static_cast<unsigned long long>(compat::JPC_MOTION_QUALITY_DISCRETE) ==
                  static_cast<unsigned long long>(JPH::EMotionQuality::Discrete),
              "Native enum changed: JPC_MOTION_QUALITY_DISCRETE");
static_assert(static_cast<unsigned long long>(compat::JPC_MOTION_QUALITY_LINEAR_CAST) ==
                  static_cast<unsigned long long>(JPH::EMotionQuality::LinearCast),
              "Native enum changed: JPC_MOTION_QUALITY_LINEAR_CAST");
static_assert(
    static_cast<unsigned long long>(compat::JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA) ==
        static_cast<unsigned long long>(JPH::EOverrideMassProperties::CalculateMassAndInertia),
    "Native enum changed: JPC_OVERRIDE_MASS_PROPS_CALC_MASS_INERTIA");
static_assert(static_cast<unsigned long long>(compat::JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA) ==
                  static_cast<unsigned long long>(JPH::EOverrideMassProperties::CalculateInertia),
              "Native enum changed: JPC_OVERRIDE_MASS_PROPS_CALC_INERTIA");
static_assert(
    static_cast<unsigned long long>(compat::JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED) ==
        static_cast<unsigned long long>(JPH::EOverrideMassProperties::MassAndInertiaProvided),
    "Native enum changed: JPC_OVERRIDE_MASS_PROPS_MASS_INERTIA_PROVIDED");
static_assert(
    static_cast<unsigned long long>(compat::JPC_VALIDATE_RESULT_ACCEPT_ALL_CONTACTS) ==
        static_cast<unsigned long long>(JPH::ValidateResult::AcceptAllContactsForThisBodyPair),
    "Native enum changed: JPC_VALIDATE_RESULT_ACCEPT_ALL_CONTACTS");
static_assert(static_cast<unsigned long long>(compat::JPC_VALIDATE_RESULT_ACCEPT_CONTACT) ==
                  static_cast<unsigned long long>(JPH::ValidateResult::AcceptContact),
              "Native enum changed: JPC_VALIDATE_RESULT_ACCEPT_CONTACT");
static_assert(static_cast<unsigned long long>(compat::JPC_VALIDATE_RESULT_REJECT_CONTACT) ==
                  static_cast<unsigned long long>(JPH::ValidateResult::RejectContact),
              "Native enum changed: JPC_VALIDATE_RESULT_REJECT_CONTACT");
static_assert(
    static_cast<unsigned long long>(compat::JPC_VALIDATE_RESULT_REJECT_ALL_CONTACTS) ==
        static_cast<unsigned long long>(JPH::ValidateResult::RejectAllContactsForThisBodyPair),
    "Native enum changed: JPC_VALIDATE_RESULT_REJECT_ALL_CONTACTS");
static_assert(static_cast<unsigned long long>(compat::JPC_BACK_FACE_MODE_IGNORE) ==
                  static_cast<unsigned long long>(JPH::EBackFaceMode::IgnoreBackFaces),
              "Native enum changed: JPC_BACK_FACE_MODE_IGNORE");
static_assert(static_cast<unsigned long long>(compat::JPC_BACK_FACE_MODE_COLLIDE) ==
                  static_cast<unsigned long long>(JPH::EBackFaceMode::CollideWithBackFaces),
              "Native enum changed: JPC_BACK_FACE_MODE_COLLIDE");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_NONE) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::None),
              "Native enum changed: JPC_ALLOWED_DOFS_NONE");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_ALL) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::All),
              "Native enum changed: JPC_ALLOWED_DOFS_ALL");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_TRANSLATIONX) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::TranslationX),
              "Native enum changed: JPC_ALLOWED_DOFS_TRANSLATIONX");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_TRANSLATIONY) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::TranslationY),
              "Native enum changed: JPC_ALLOWED_DOFS_TRANSLATIONY");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_TRANSLATIONZ) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::TranslationZ),
              "Native enum changed: JPC_ALLOWED_DOFS_TRANSLATIONZ");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_ROTATIONX) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::RotationX),
              "Native enum changed: JPC_ALLOWED_DOFS_ROTATIONX");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_ROTATIONY) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::RotationY),
              "Native enum changed: JPC_ALLOWED_DOFS_ROTATIONY");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_ROTATIONZ) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::RotationZ),
              "Native enum changed: JPC_ALLOWED_DOFS_ROTATIONZ");
static_assert(static_cast<unsigned long long>(compat::JPC_ALLOWED_DOFS_PLANE2D) ==
                  static_cast<unsigned long long>(JPH::EAllowedDOFs::Plane2D),
              "Native enum changed: JPC_ALLOWED_DOFS_PLANE2D");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_INSTANCE_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::InstanceColor),
              "Native enum changed: JPC_SHAPE_COLOR_INSTANCE_COLOR");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_SHAPE_TYPE_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::ShapeTypeColor),
              "Native enum changed: JPC_SHAPE_COLOR_SHAPE_TYPE_COLOR");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_MOTION_TYPE_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::MotionTypeColor),
              "Native enum changed: JPC_SHAPE_COLOR_MOTION_TYPE_COLOR");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_SLEEP_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::SleepColor),
              "Native enum changed: JPC_SHAPE_COLOR_SLEEP_COLOR");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_ISLAND_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::IslandColor),
              "Native enum changed: JPC_SHAPE_COLOR_ISLAND_COLOR");
static_assert(static_cast<unsigned long long>(compat::JPC_SHAPE_COLOR_MATERIAL_COLOR) ==
                  static_cast<unsigned long long>(JPH::BodyManager::EShapeColor::MaterialColor),
              "Native enum changed: JPC_SHAPE_COLOR_MATERIAL_COLOR");
static_assert(
    static_cast<unsigned long long>(compat::JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_TYPE) ==
        static_cast<unsigned long long>(JPH::ESoftBodyConstraintColor::ConstraintType),
    "Native enum changed: JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_TYPE");
static_assert(
    static_cast<unsigned long long>(compat::JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_GROUP) ==
        static_cast<unsigned long long>(JPH::ESoftBodyConstraintColor::ConstraintGroup),
    "Native enum changed: JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_GROUP");
static_assert(
    static_cast<unsigned long long>(compat::JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_ORDER) ==
        static_cast<unsigned long long>(JPH::ESoftBodyConstraintColor::ConstraintOrder),
    "Native enum changed: JPC_SOFT_BODY_CONSTRAINT_COLOR_CONSTRAINT_ORDER");
static_assert(
    static_cast<unsigned long long>(compat::JPC_ACTIVE_EDGE_MODE_COLLIDE_ONLY_WITH_ACTIVE) ==
        static_cast<unsigned long long>(JPH::EActiveEdgeMode::CollideOnlyWithActive),
    "Native enum changed: JPC_ACTIVE_EDGE_MODE_COLLIDE_ONLY_WITH_ACTIVE");
static_assert(static_cast<unsigned long long>(compat::JPC_ACTIVE_EDGE_MODE_COLLIDE_WITH_ALL) ==
                  static_cast<unsigned long long>(JPH::EActiveEdgeMode::CollideWithAll),
              "Native enum changed: JPC_ACTIVE_EDGE_MODE_COLLIDE_WITH_ALL");
static_assert(static_cast<unsigned long long>(compat::JPC_COLLECT_FACES_MODE_COLLECT_FACES) ==
                  static_cast<unsigned long long>(JPH::ECollectFacesMode::CollectFaces),
              "Native enum changed: JPC_COLLECT_FACES_MODE_COLLECT_FACES");
static_assert(static_cast<unsigned long long>(compat::JPC_COLLECT_FACES_MODE_NO_FACES) ==
                  static_cast<unsigned long long>(JPH::ECollectFacesMode::NoFaces),
              "Native enum changed: JPC_COLLECT_FACES_MODE_NO_FACES");
