use crate::jolt;
use std::marker::PhantomData;

use crate::raw::*;
use glam::Quat;

use crate::{Body, BodyId, ObjectLayer, RVec3, Vec3};

/// See also: Jolt's [`BodyInterface`](https://jrouwe.github.io/JoltPhysicsDocs/5.1.0/class_body_interface.html) class.
pub struct BodyInterface<'physics_system> {
    raw: *mut JPC_BodyInterface,
    _phantom: PhantomData<&'physics_system ()>,
}

impl<'physics_system> BodyInterface<'physics_system> {
    pub(crate) fn new(raw: *mut JPC_BodyInterface) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    /// # Safety
    /// `settings` must be initialized and valid, with a valid `Shape` pointer.
    pub unsafe fn create_body(&self, settings: &JPC_BodyCreationSettings) -> Option<Body<'_>> {
        let settings = jolt::ffi::native_body_settings(settings).expect("native body settings");
        let raw = jolt::pin(self.raw).CreateBody(&settings);

        if raw.is_null() {
            None
        } else {
            Some(Body::<'physics_system>::new(raw))
        }
    }

    pub fn add_body(&self, body_id: BodyId, activation_mode: JPC_Activation) {
        unsafe {
            jolt::pin(self.raw).AddBody(
                &jolt::BodyID(body_id.raw()),
                jolt::EActivation(activation_mode),
            );
        }
    }

    pub fn activate_body(&self, body_id: BodyId) {
        unsafe {
            jolt::pin(self.raw).ActivateBody(&jolt::BodyID(body_id.raw()));
        }
    }

    pub fn activate_bodies(&self, body_ids: &[BodyId]) {
        unsafe {
            jolt::pin(self.raw).ActivateBodies(
                (BodyId::slice_raw(body_ids).as_ptr()).cast(),
                body_ids.len() as i32,
            );
        }
    }

    pub fn remove_body(&self, body_id: BodyId) {
        unsafe { jolt::pin(self.raw).RemoveBody(&jolt::BodyID(body_id.raw())) }
    }

    pub fn destroy_body(&self, body_id: BodyId) {
        unsafe { jolt::pin(self.raw).DestroyBody(&jolt::BodyID(body_id.raw())) }
    }

    /// # Safety
    /// `shape` must be a valid shape.
    pub unsafe fn set_shape(
        &self,
        body_id: BodyId,
        shape: *const JPC_Shape,
        update_mass_properties: bool,
        activation: JPC_Activation,
    ) {
        (&*self.raw).SetShape(
            &jolt::BodyID(body_id.raw()),
            shape,
            update_mass_properties,
            jolt::EActivation(activation),
        )
    }

    pub fn is_active(&self, body_id: BodyId) -> bool {
        unsafe { (&*self.raw).IsActive(&jolt::BodyID(body_id.raw())) }
    }

    pub fn user_data(&self, body_id: BodyId) -> u64 {
        unsafe { (&*self.raw).GetUserData(&jolt::BodyID(body_id.raw())) }
    }

    pub fn set_user_data(&self, body_id: BodyId, user_data: u64) {
        unsafe { (&*self.raw).SetUserData(&jolt::BodyID(body_id.raw()), user_data) }
    }

    pub fn center_of_mass_position(&self, body_id: BodyId) -> RVec3 {
        unsafe {
            (&*self.raw)
                .GetCenterOfMassPosition(&jolt::BodyID(body_id.raw()))
                .into()
        }
    }

    pub fn linear_velocity(&self, body_id: BodyId) -> Vec3 {
        unsafe {
            (&*self.raw)
                .GetLinearVelocity(&jolt::BodyID(body_id.raw()))
                .into()
        }
    }

    pub fn set_linear_velocity(&self, body_id: BodyId, velocity: Vec3) {
        unsafe {
            jolt::pin(self.raw).SetLinearVelocity(&jolt::BodyID(body_id.raw()), (velocity).into());
        }
    }

    pub fn set_object_layer(&self, body_id: BodyId, object_layer: ObjectLayer) {
        unsafe {
            jolt::pin(self.raw).SetObjectLayer(&jolt::BodyID(body_id.raw()), object_layer.raw())
        }
    }

    pub fn notify_shape_changed(
        &self,
        body_id: BodyId,
        old_com: Vec3,
        update_mass_properties: bool,
        activation: JPC_Activation,
    ) {
        unsafe {
            (&*self.raw).NotifyShapeChanged(
                &jolt::BodyID(body_id.raw()),
                (old_com).into(),
                update_mass_properties,
                jolt::EActivation(activation),
            )
        }
    }

    pub fn set_position_and_rotation_when_changed(
        &self,
        body_id: BodyId,
        pos: RVec3,
        rot: Quat,
        activation: JPC_Activation,
    ) {
        unsafe {
            jolt::pin(self.raw).SetPositionAndRotationWhenChanged(
                &jolt::BodyID(body_id.raw()),
                (pos).into(),
                (rot).into(),
                jolt::EActivation(activation),
            )
        }
    }

    pub fn raw(&self) -> *mut JPC_BodyInterface {
        self.raw
    }
}

unsafe impl Sync for BodyInterface<'_> {}
