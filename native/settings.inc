// Convert the retained public settings to/from native Jolt settings.
// Field accesses are checked against Jolt headers by the C++ compiler.
void JPC_MassProperties_default(JPC_MassProperties *p) {
    JPH::MassProperties d;
    *p = {};
    p->Mass = d.mMass;
    p->Inertia = mat(d.mInertia);
}
static JPH::MassProperties mass(const JPC_MassProperties &s) {
    JPH::MassProperties d;
    d.mMass = s.Mass;
    d.mInertia = mat(s.Inertia);
    return d;
}
static JPC_MassProperties mass(const JPH::MassProperties &s) {
    JPC_MassProperties p{};
    p.Mass = s.mMass;
    p.Inertia = mat(s.mInertia);
    return p;
}
void JPC_BodyCreationSettings_default(JPC_BodyCreationSettings *p) {
    JPH::BodyCreationSettings d;
    *p = {};
    p->Position = vec(d.mPosition);
    p->Rotation = quat(d.mRotation);
    p->LinearVelocity = vec(d.mLinearVelocity);
    p->AngularVelocity = vec(d.mAngularVelocity);
    p->UserData = d.mUserData;
    p->ObjectLayer = d.mObjectLayer;
    p->MotionType = JPC_MotionType(d.mMotionType);
    p->AllowedDOFs = JPC_AllowedDOFs(d.mAllowedDOFs);
    p->AllowDynamicOrKinematic = d.mAllowDynamicOrKinematic;
    p->IsSensor = d.mIsSensor;
    p->CollideKinematicVsNonDynamic = d.mCollideKinematicVsNonDynamic;
    p->UseManifoldReduction = d.mUseManifoldReduction;
    p->ApplyGyroscopicForce = d.mApplyGyroscopicForce;
    p->MotionQuality = JPC_MotionQuality(d.mMotionQuality);
    p->EnhancedInternalEdgeRemoval = d.mEnhancedInternalEdgeRemoval;
    p->AllowSleeping = d.mAllowSleeping;
    p->Friction = d.mFriction;
    p->Restitution = d.mRestitution;
    p->LinearDamping = d.mLinearDamping;
    p->AngularDamping = d.mAngularDamping;
    p->MaxLinearVelocity = d.mMaxLinearVelocity;
    p->MaxAngularVelocity = d.mMaxAngularVelocity;
    p->GravityFactor = d.mGravityFactor;
    p->NumVelocityStepsOverride = d.mNumVelocityStepsOverride;
    p->NumPositionStepsOverride = d.mNumPositionStepsOverride;
    p->OverrideMassProperties = JPC_OverrideMassProperties(d.mOverrideMassProperties);
    p->MassPropertiesOverride = mass(d.mMassPropertiesOverride);
    p->InertiaMultiplier = d.mInertiaMultiplier;
}
static JPH::BodyCreationSettings body_settings(const JPC_BodyCreationSettings &s) {
    JPH::BodyCreationSettings d;
    d.mPosition = vec(s.Position);
    d.mRotation = quat(s.Rotation);
    d.mLinearVelocity = vec(s.LinearVelocity);
    d.mAngularVelocity = vec(s.AngularVelocity);
    d.mUserData = s.UserData;
    d.mObjectLayer = s.ObjectLayer;
    d.mMotionType = JPH::EMotionType(s.MotionType);
    d.mAllowedDOFs = JPH::EAllowedDOFs(s.AllowedDOFs);
    d.mAllowDynamicOrKinematic = s.AllowDynamicOrKinematic;
    d.mIsSensor = s.IsSensor;
    d.mCollideKinematicVsNonDynamic = s.CollideKinematicVsNonDynamic;
    d.mUseManifoldReduction = s.UseManifoldReduction;
    d.mApplyGyroscopicForce = s.ApplyGyroscopicForce;
    d.mMotionQuality = JPH::EMotionQuality(s.MotionQuality);
    d.mEnhancedInternalEdgeRemoval = s.EnhancedInternalEdgeRemoval;
    d.mAllowSleeping = s.AllowSleeping;
    d.mFriction = s.Friction;
    d.mRestitution = s.Restitution;
    d.mLinearDamping = s.LinearDamping;
    d.mAngularDamping = s.AngularDamping;
    d.mMaxLinearVelocity = s.MaxLinearVelocity;
    d.mMaxAngularVelocity = s.MaxAngularVelocity;
    d.mGravityFactor = s.GravityFactor;
    d.mNumVelocityStepsOverride = s.NumVelocityStepsOverride;
    d.mNumPositionStepsOverride = s.NumPositionStepsOverride;
    d.mOverrideMassProperties = JPH::EOverrideMassProperties(s.OverrideMassProperties);
    d.mMassPropertiesOverride = mass(s.MassPropertiesOverride);
    d.mInertiaMultiplier = s.InertiaMultiplier;
    d.SetShape(s.Shape);
    return d;
}
void JPC_BoxShapeSettings_default(JPC_BoxShapeSettings *p) {
    JPH::BoxShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->HalfExtent = vec(d.mHalfExtent);
    p->ConvexRadius = d.mConvexRadius;
}
bool JPC_BoxShapeSettings_Create(const JPC_BoxShapeSettings *s, JPC_Shape **out, JPC_String **err) {
    JPH::BoxShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mHalfExtent = vec(s->HalfExtent);
    d.mConvexRadius = s->ConvexRadius;
    return create_shape(d, out, err);
}
void JPC_SphereShapeSettings_default(JPC_SphereShapeSettings *p) {
    JPH::SphereShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->Radius = d.mRadius;
}
bool JPC_SphereShapeSettings_Create(const JPC_SphereShapeSettings *s, JPC_Shape **out,
                                    JPC_String **err) {
    JPH::SphereShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mRadius = s->Radius;
    return create_shape(d, out, err);
}
void JPC_CapsuleShapeSettings_default(JPC_CapsuleShapeSettings *p) {
    JPH::CapsuleShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->Radius = d.mRadius;
    p->HalfHeightOfCylinder = d.mHalfHeightOfCylinder;
}
bool JPC_CapsuleShapeSettings_Create(const JPC_CapsuleShapeSettings *s, JPC_Shape **out,
                                     JPC_String **err) {
    JPH::CapsuleShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mRadius = s->Radius;
    d.mHalfHeightOfCylinder = s->HalfHeightOfCylinder;
    return create_shape(d, out, err);
}
void JPC_CylinderShapeSettings_default(JPC_CylinderShapeSettings *p) {
    JPH::CylinderShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->HalfHeight = d.mHalfHeight;
    p->Radius = d.mRadius;
    p->ConvexRadius = d.mConvexRadius;
}
bool JPC_CylinderShapeSettings_Create(const JPC_CylinderShapeSettings *s, JPC_Shape **out,
                                      JPC_String **err) {
    JPH::CylinderShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mHalfHeight = s->HalfHeight;
    d.mRadius = s->Radius;
    d.mConvexRadius = s->ConvexRadius;
    return create_shape(d, out, err);
}
void JPC_TriangleShapeSettings_default(JPC_TriangleShapeSettings *p) {
    JPH::TriangleShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->V1 = vec(d.mV1);
    p->V2 = vec(d.mV2);
    p->V3 = vec(d.mV3);
    p->ConvexRadius = d.mConvexRadius;
}
bool JPC_TriangleShapeSettings_Create(const JPC_TriangleShapeSettings *s, JPC_Shape **out,
                                      JPC_String **err) {
    JPH::TriangleShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mV1 = vec(s->V1);
    d.mV2 = vec(s->V2);
    d.mV3 = vec(s->V3);
    d.mConvexRadius = s->ConvexRadius;
    return create_shape(d, out, err);
}
void JPC_PlaneShapeSettings_default(JPC_PlaneShapeSettings *p) {
    JPH::PlaneShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->HalfExtent = d.mHalfExtent;
    p->Normal = vec(d.mPlane.GetNormal());
    p->Constant = d.mPlane.GetConstant();
}
bool JPC_PlaneShapeSettings_Create(const JPC_PlaneShapeSettings *s, JPC_Shape **out,
                                   JPC_String **err) {
    JPH::PlaneShapeSettings d;
    d.mUserData = s->UserData;
    d.mHalfExtent = s->HalfExtent;
    d.mPlane = JPH::Plane(vec(s->Normal), s->Constant);
    return create_shape(d, out, err);
}
void JPC_ConvexHullShapeSettings_default(JPC_ConvexHullShapeSettings *p) {
    JPH::ConvexHullShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Density = d.mDensity;
    p->MaxConvexRadius = d.mMaxConvexRadius;
    p->MaxErrorConvexRadius = d.mMaxErrorConvexRadius;
    p->HullTolerance = d.mHullTolerance;
}
bool JPC_ConvexHullShapeSettings_Create(const JPC_ConvexHullShapeSettings *s, JPC_Shape **out,
                                        JPC_String **err) {
    JPH::ConvexHullShapeSettings d;
    d.mUserData = s->UserData;
    d.mDensity = s->Density;
    d.mMaxConvexRadius = s->MaxConvexRadius;
    d.mMaxErrorConvexRadius = s->MaxErrorConvexRadius;
    d.mHullTolerance = s->HullTolerance;
    for (size_t i = 0; i < s->PointsLen; ++i)
        d.mPoints.push_back(vec(s->Points[i]));
    return create_shape(d, out, err);
}
void JPC_MeshShapeSettings_default(JPC_MeshShapeSettings *p) {
    JPH::MeshShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
}
bool JPC_MeshShapeSettings_Create(const JPC_MeshShapeSettings *s, JPC_Shape **out,
                                  JPC_String **err) {
    JPH::MeshShapeSettings d;
    d.mUserData = s->UserData;
    for (size_t i = 0; i < s->TriangleVerticesLen; ++i) {
        auto v = s->TriangleVertices[i];
        d.mTriangleVertices.emplace_back(v.x, v.y, v.z);
    }
    for (size_t i = 0; i < s->IndexedTrianglesLen; ++i) {
        auto t = s->IndexedTriangles[i];
        d.mIndexedTriangles.emplace_back(t.idx[0], t.idx[1], t.idx[2], t.materialIndex, t.userData);
    }
    return create_shape(d, out, err);
}
void JPC_StaticCompoundShapeSettings_default(JPC_StaticCompoundShapeSettings *p) {
    JPH::StaticCompoundShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
}
bool JPC_StaticCompoundShapeSettings_Create(const JPC_StaticCompoundShapeSettings *s,
                                            JPC_Shape **out, JPC_String **err) {
    JPH::StaticCompoundShapeSettings d;
    d.mUserData = s->UserData;
    for (size_t i = 0; i < s->SubShapesLen; ++i) {
        auto c = s->SubShapes[i];
        d.AddShape(vec(c.Position), quat(c.Rotation), c.Shape, c.UserData);
    }
    return create_shape(d, out, err);
}
void JPC_MutableCompoundShapeSettings_default(JPC_MutableCompoundShapeSettings *p) {
    JPH::MutableCompoundShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
}
bool JPC_MutableCompoundShapeSettings_Create(const JPC_MutableCompoundShapeSettings *s,
                                             JPC_MutableCompoundShape **out, JPC_String **err) {
    JPH::MutableCompoundShapeSettings d;
    d.mUserData = s->UserData;
    for (size_t i = 0; i < s->SubShapesLen; ++i) {
        auto c = s->SubShapes[i];
        d.AddShape(vec(c.Position), quat(c.Rotation), c.Shape, c.UserData);
    }
    return create_shape(d, out, err);
}
void JPC_HeightFieldShapeSettings_default(JPC_HeightFieldShapeSettings *p) {
    JPH::HeightFieldShapeSettings d;
    *p = {};
    p->UserData = d.mUserData;
    p->Offset = vec(d.mOffset);
    p->Scale = vec(d.mScale);
    p->SampleCount = d.mSampleCount;
    p->MinHeightValue = d.mMinHeightValue;
    p->MaxHeightValue = d.mMaxHeightValue;
    p->MaterialsCapacity = d.mMaterialsCapacity;
    p->BlockSize = d.mBlockSize;
    p->BitsPerSample = d.mBitsPerSample;
    p->ActiveEdgeCosThresholdAngle = d.mActiveEdgeCosThresholdAngle;
}
bool JPC_HeightFieldShapeSettings_Create(const JPC_HeightFieldShapeSettings *s, JPC_Shape **out,
                                         JPC_String **err) {
    JPH::HeightFieldShapeSettings d;
    d.mUserData = s->UserData;
    d.mOffset = vec(s->Offset);
    d.mScale = vec(s->Scale);
    d.mSampleCount = s->SampleCount;
    d.mMinHeightValue = s->MinHeightValue;
    d.mMaxHeightValue = s->MaxHeightValue;
    d.mMaterialsCapacity = s->MaterialsCapacity;
    d.mBlockSize = s->BlockSize;
    d.mBitsPerSample = s->BitsPerSample;
    d.mActiveEdgeCosThresholdAngle = s->ActiveEdgeCosThresholdAngle;
    if (!s->HeightSamples || uint64_t(s->SampleCount) * s->SampleCount != s->HeightSamplesLen) {
        *out = nullptr;
        if (err)
            *err = foreign<JPC_String>(
                new std::string("Expected sample_count squared height samples"));
        return false;
    }
    d.mHeightSamples.assign(s->HeightSamples, s->HeightSamples + s->HeightSamplesLen);
    return create_shape(d, out, err);
}
void JPC_ShapeCastSettings_default(JPC_ShapeCastSettings *p) {
    JPH::ShapeCastSettings d;
    *p = {};
    p->ActiveEdgeMode = JPC_ActiveEdgeMode(d.mActiveEdgeMode);
    p->CollectFacesMode = JPC_CollectFacesMode(d.mCollectFacesMode);
    p->CollisionTolerance = d.mCollisionTolerance;
    p->PenetrationTolerance = d.mPenetrationTolerance;
    p->ActiveEdgeMovementDirection = vec(d.mActiveEdgeMovementDirection);
    p->BackFaceModeTriangles = JPC_BackFaceMode(d.mBackFaceModeTriangles);
    p->BackFaceModeConvex = JPC_BackFaceMode(d.mBackFaceModeConvex);
    p->UseShrunkenShapeAndConvexRadius = d.mUseShrunkenShapeAndConvexRadius;
    p->ReturnDeepestPoint = d.mReturnDeepestPoint;
}
static JPH::ShapeCastSettings settings(const JPC_ShapeCastSettings &s) {
    JPH::ShapeCastSettings d;
    d.mActiveEdgeMode = JPH::EActiveEdgeMode(s.ActiveEdgeMode);
    d.mCollectFacesMode = JPH::ECollectFacesMode(s.CollectFacesMode);
    d.mCollisionTolerance = s.CollisionTolerance;
    d.mPenetrationTolerance = s.PenetrationTolerance;
    d.mActiveEdgeMovementDirection = vec(s.ActiveEdgeMovementDirection);
    d.mBackFaceModeTriangles = JPH::EBackFaceMode(s.BackFaceModeTriangles);
    d.mBackFaceModeConvex = JPH::EBackFaceMode(s.BackFaceModeConvex);
    d.mUseShrunkenShapeAndConvexRadius = s.UseShrunkenShapeAndConvexRadius;
    d.mReturnDeepestPoint = s.ReturnDeepestPoint;
    if (s.ExtraConvexRadius != 0)
        throw std::invalid_argument(
            "ExtraConvexRadius was removed from Jolt; nonzero value is unsupported");
    return d;
}
void JPC_CollideShapeSettings_default(JPC_CollideShapeSettings *p) {
    JPH::CollideShapeSettings d;
    *p = {};
    p->ActiveEdgeMode = JPC_ActiveEdgeMode(d.mActiveEdgeMode);
    p->CollectFacesMode = JPC_CollectFacesMode(d.mCollectFacesMode);
    p->CollisionTolerance = d.mCollisionTolerance;
    p->PenetrationTolerance = d.mPenetrationTolerance;
    p->ActiveEdgeMovementDirection = vec(d.mActiveEdgeMovementDirection);
    p->MaxSeparationDistance = d.mMaxSeparationDistance;
    p->BackFaceMode = JPC_BackFaceMode(d.mBackFaceMode);
}
static JPH::CollideShapeSettings settings(const JPC_CollideShapeSettings &s) {
    JPH::CollideShapeSettings d;
    d.mActiveEdgeMode = JPH::EActiveEdgeMode(s.ActiveEdgeMode);
    d.mCollectFacesMode = JPH::ECollectFacesMode(s.CollectFacesMode);
    d.mCollisionTolerance = s.CollisionTolerance;
    d.mPenetrationTolerance = s.PenetrationTolerance;
    d.mActiveEdgeMovementDirection = vec(s.ActiveEdgeMovementDirection);
    d.mMaxSeparationDistance = s.MaxSeparationDistance;
    d.mBackFaceMode = JPH::EBackFaceMode(s.BackFaceMode);
    return d;
}
void JPC_BodyManager_DrawSettings_default(JPC_BodyManager_DrawSettings *p) {
    JPH::BodyManager::DrawSettings d;
    *p = {};
    p->mDrawGetSupportFunction = d.mDrawGetSupportFunction;
    p->mDrawSupportDirection = d.mDrawSupportDirection;
    p->mDrawGetSupportingFace = d.mDrawGetSupportingFace;
    p->mDrawShape = d.mDrawShape;
    p->mDrawShapeWireframe = d.mDrawShapeWireframe;
    p->mDrawShapeColor = JPC_ShapeColor(d.mDrawShapeColor);
    p->mDrawBoundingBox = d.mDrawBoundingBox;
    p->mDrawCenterOfMassTransform = d.mDrawCenterOfMassTransform;
    p->mDrawWorldTransform = d.mDrawWorldTransform;
    p->mDrawVelocity = d.mDrawVelocity;
    p->mDrawMassAndInertia = d.mDrawMassAndInertia;
    p->mDrawSleepStats = d.mDrawSleepStats;
    p->mDrawSoftBodyVertices = d.mDrawSoftBodyVertices;
    p->mDrawSoftBodyVertexVelocities = d.mDrawSoftBodyVertexVelocities;
    p->mDrawSoftBodyEdgeConstraints = d.mDrawSoftBodyEdgeConstraints;
    p->mDrawSoftBodyBendConstraints = d.mDrawSoftBodyBendConstraints;
    p->mDrawSoftBodyVolumeConstraints = d.mDrawSoftBodyVolumeConstraints;
    p->mDrawSoftBodySkinConstraints = d.mDrawSoftBodySkinConstraints;
    p->mDrawSoftBodyLRAConstraints = d.mDrawSoftBodyLRAConstraints;
    p->mDrawSoftBodyRods = d.mDrawSoftBodyRods;
    p->mDrawSoftBodyRodStates = d.mDrawSoftBodyRodStates;
    p->mDrawSoftBodyRodBendTwistConstraints = d.mDrawSoftBodyRodBendTwistConstraints;
    p->mDrawSoftBodyPredictedBounds = d.mDrawSoftBodyPredictedBounds;
    p->DrawSoftBodyConstraintColor = JPC_SoftBodyConstraintColor(d.mDrawSoftBodyConstraintColor);
}
static JPH::BodyManager::DrawSettings draw_settings(const JPC_BodyManager_DrawSettings &s) {
    JPH::BodyManager::DrawSettings d;
    d.mDrawGetSupportFunction = s.mDrawGetSupportFunction;
    d.mDrawSupportDirection = s.mDrawSupportDirection;
    d.mDrawGetSupportingFace = s.mDrawGetSupportingFace;
    d.mDrawShape = s.mDrawShape;
    d.mDrawShapeWireframe = s.mDrawShapeWireframe;
    d.mDrawShapeColor = JPH::BodyManager::EShapeColor(s.mDrawShapeColor);
    d.mDrawBoundingBox = s.mDrawBoundingBox;
    d.mDrawCenterOfMassTransform = s.mDrawCenterOfMassTransform;
    d.mDrawWorldTransform = s.mDrawWorldTransform;
    d.mDrawVelocity = s.mDrawVelocity;
    d.mDrawMassAndInertia = s.mDrawMassAndInertia;
    d.mDrawSleepStats = s.mDrawSleepStats;
    d.mDrawSoftBodyVertices = s.mDrawSoftBodyVertices;
    d.mDrawSoftBodyVertexVelocities = s.mDrawSoftBodyVertexVelocities;
    d.mDrawSoftBodyEdgeConstraints = s.mDrawSoftBodyEdgeConstraints;
    d.mDrawSoftBodyBendConstraints = s.mDrawSoftBodyBendConstraints;
    d.mDrawSoftBodyVolumeConstraints = s.mDrawSoftBodyVolumeConstraints;
    d.mDrawSoftBodySkinConstraints = s.mDrawSoftBodySkinConstraints;
    d.mDrawSoftBodyLRAConstraints = s.mDrawSoftBodyLRAConstraints;
    d.mDrawSoftBodyRods = s.mDrawSoftBodyRods;
    d.mDrawSoftBodyRodStates = s.mDrawSoftBodyRodStates;
    d.mDrawSoftBodyRodBendTwistConstraints = s.mDrawSoftBodyRodBendTwistConstraints;
    d.mDrawSoftBodyPredictedBounds = s.mDrawSoftBodyPredictedBounds;
    d.mDrawSoftBodyConstraintColor = JPH::ESoftBodyConstraintColor(s.DrawSoftBodyConstraintColor);
    return d;
}
void JPC_SubShapeSettings_default(JPC_SubShapeSettings *p) {
    *p = {};
    p->Rotation.w = 1;
}
