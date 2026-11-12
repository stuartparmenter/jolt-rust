#[allow(dead_code)]
mod framework;

use std::{ffi::CStr, ptr};

use framework::*;
use joltc_sys::*;

unsafe fn create(settings: &JPC_HeightFieldShapeSettings) -> Result<*mut JPC_Shape, String> {
    let mut shape = ptr::null_mut();
    let mut error = ptr::null_mut();
    if JPC_HeightFieldShapeSettings_Create(settings, &mut shape, &mut error) {
        assert!(!shape.is_null());
        assert!(error.is_null());
        Ok(shape)
    } else {
        assert!(shape.is_null());
        assert!(!error.is_null());
        let message = CStr::from_ptr(JPC_String_c_str(error))
            .to_string_lossy()
            .into_owned();
        JPC_String_delete(error);
        Err(message)
    }
}

struct Heightfield {
    terrain: JPC_BodyID,
    sphere: JPC_BodyID,
    frames: u32,
}

impl SmokeTest for Heightfield {
    unsafe fn setup(system: *mut JPC_PhysicsSystem) -> Self {
        let defaults = JPC_HeightFieldShapeSettings::default();
        assert_eq!(defaults.Scale.x, 1.0);
        assert_eq!(defaults.Scale.y, 1.0);
        assert_eq!(defaults.Scale.z, 1.0);
        assert_eq!(defaults.BlockSize, 2);
        assert!(defaults.HeightSamples.is_null());
        assert_eq!(defaults.HeightSamplesLen, 0);

        let mut heights: Vec<f32> = (0..64).map(|i| (i % 8 + 2 * (i / 8)) as f32).collect();
        let mut settings = JPC_HeightFieldShapeSettings {
            UserData: 42,
            Offset: vec3(-4.0, 3.0, -4.0),
            Scale: vec3(2.0, 0.5, 2.0),
            SampleCount: 8,
            HeightSamples: heights.as_ptr(),
            HeightSamplesLen: heights.len(),
            ..defaults
        };
        let shape = create(&settings).unwrap();
        // The shape must own its data after Create returns.
        heights.fill(1000.0);
        drop(heights);
        let heightfield = shape.cast::<JPC_HeightFieldShape>();
        assert_eq!(JPC_Shape_GetSubType(shape), JPC_SHAPE_SUB_TYPE_HEIGHT_FIELD);
        assert_eq!(JPC_Shape_GetUserData(shape), 42);
        assert_eq!(JPC_HeightFieldShape_GetSampleCount(heightfield), 8);
        assert_eq!(JPC_HeightFieldShape_GetBlockSize(heightfield), 2);
        let position = JPC_HeightFieldShape_GetPosition(heightfield, 3, 2);
        assert!((position.x - 2.0).abs() < 0.001);
        assert!((position.y - 6.5).abs() < 0.03);
        assert!(position.z.abs() < 0.001);
        assert!(!JPC_HeightFieldShape_IsNoCollision(heightfield, 3, 2));
        assert!(JPC_HeightFieldShape_GetMinHeightValue(heightfield) <= position.y);
        assert!(JPC_HeightFieldShape_GetMaxHeightValue(heightfield) >= position.y);
        JPC_Shape_Release(shape);

        let holes = [JPC_HeightFieldShape_GetNoCollisionValue(); 64];
        settings.HeightSamples = holes.as_ptr();
        let shape = create(&settings).unwrap();
        assert!(JPC_HeightFieldShape_IsNoCollision(shape.cast(), 3, 2));
        JPC_Shape_Release(shape);

        settings.HeightSamplesLen = 63;
        assert!(create(&settings).unwrap_err().contains("squared"));
        settings.HeightSamplesLen = 64;
        settings.HeightSamples = ptr::null();
        assert!(create(&settings).is_err());
        settings.HeightSamples = holes.as_ptr();
        settings.BlockSize = 0;
        assert!(create(&settings).unwrap_err().contains("block size"));
        settings.BlockSize = 2;
        settings.SampleCount = 0;
        assert!(create(&settings).is_err());
        settings.SampleCount = 8;
        settings.BitsPerSample = 0;
        assert!(create(&settings).is_err());
        // Error reporting is optional, but failure still clears the shape.
        let mut failed_shape = ptr::null_mut();
        assert!(!JPC_HeightFieldShapeSettings_Create(
            &settings,
            &mut failed_shape,
            ptr::null_mut()
        ));
        assert!(failed_shape.is_null());

        let heights = [2.0; 64];
        let terrain_shape = create(&JPC_HeightFieldShapeSettings {
            Offset: vec3(-4.0, 0.0, -4.0),
            SampleCount: 8,
            HeightSamples: heights.as_ptr(),
            HeightSamplesLen: heights.len(),
            ..Default::default()
        })
        .unwrap();
        let interface = JPC_PhysicsSystem_GetBodyInterface(system);
        let terrain = JPC_BodyInterface_CreateBody(
            interface,
            &JPC_BodyCreationSettings {
                Shape: terrain_shape,
                MotionType: JPC_MOTION_TYPE_STATIC,
                ObjectLayer: OL_NON_MOVING,
                ..Default::default()
            },
        );
        JPC_Shape_Release(terrain_shape);
        let terrain = JPC_Body_GetID(terrain);
        JPC_BodyInterface_AddBody(interface, terrain, JPC_ACTIVATION_DONT_ACTIVATE);

        let sphere_shape = create_sphere(&JPC_SphereShapeSettings {
            Radius: 0.5,
            ..Default::default()
        })
        .unwrap();
        let sphere = JPC_BodyInterface_CreateBody(
            interface,
            &JPC_BodyCreationSettings {
                Shape: sphere_shape,
                Position: rvec3(0.0, 6.0, 0.0),
                MotionType: JPC_MOTION_TYPE_DYNAMIC,
                ObjectLayer: OL_MOVING,
                ..Default::default()
            },
        );
        JPC_Shape_Release(sphere_shape);
        let sphere = JPC_Body_GetID(sphere);
        JPC_BodyInterface_AddBody(interface, sphere, JPC_ACTIVATION_ACTIVATE);
        Self {
            terrain,
            sphere,
            frames: 0,
        }
    }

    unsafe fn post_update(&mut self, _system: *mut JPC_PhysicsSystem) -> bool {
        self.frames += 1;
        self.frames < 240
    }

    unsafe fn teardown(&mut self, system: *mut JPC_PhysicsSystem) {
        let interface = JPC_PhysicsSystem_GetBodyInterface(system);
        let position = JPC_BodyInterface_GetPosition(interface, self.sphere);
        assert!((position.y - 2.5).abs() < 0.05, "sphere at {position:?}");
        for body in [self.sphere, self.terrain] {
            JPC_BodyInterface_RemoveBody(interface, body);
            JPC_BodyInterface_DestroyBody(interface, body);
        }
    }
}

#[test]
fn heightfield_api() {
    run_test::<Heightfield>();
}
