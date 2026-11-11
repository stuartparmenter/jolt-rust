use rolt::{raw::*, *};
use std::ptr;

struct Layers;
impl BroadPhaseLayerInterface for Layers {
    fn get_num_broad_phase_layers(&self) -> u32 {
        1
    }
    fn get_broad_phase_layer(&self, _: ObjectLayer) -> BroadPhaseLayer {
        BroadPhaseLayer::new(0)
    }
}
struct Pairs;
impl ObjectLayerPairFilter for Pairs {
    fn should_collide(&self, _: ObjectLayer, _: ObjectLayer) -> bool {
        true
    }
}
struct Vs;
impl ObjectVsBroadPhaseLayerFilter for Vs {
    fn should_collide(&self, _: ObjectLayer, _: BroadPhaseLayer) -> bool {
        true
    }
}

#[test]
fn sphere_settles_on_heightfield() {
    register_default_allocator();
    factory_init();
    register_types();
    unsafe {
        let heights = [0.0; 64];
        let settings = JPC_HeightFieldShapeSettings {
            SampleCount: 8,
            HeightSamples: heights.as_ptr(),
            HeightSamplesLen: heights.len(),
            ..Default::default()
        };
        let mut terrain = ptr::null_mut();
        let mut error = ptr::null_mut();
        assert!(JPC_HeightFieldShapeSettings_Create(
            &settings,
            &mut terrain,
            &mut error
        ));
        assert!(error.is_null());
        let mut sphere = ptr::null_mut();
        assert!(JPC_SphereShapeSettings_Create(
            &JPC_SphereShapeSettings {
                Radius: 0.5,
                ..Default::default()
            },
            &mut sphere,
            &mut error,
        ));
        let mut world = PhysicsSystem::new();
        world.init(128, 0, 128, 128, Layers, Vs, Pairs);
        let bodies = world.body_interface();
        let floor = bodies
            .create_body(&JPC_BodyCreationSettings {
                Shape: terrain.cast(),
                MotionType: JPC_MOTION_TYPE_STATIC,
                ..Default::default()
            })
            .unwrap()
            .id();
        let ball = bodies
            .create_body(&JPC_BodyCreationSettings {
                Shape: sphere.cast(),
                Position: Vec3::new(3., 3., 3.).into_jolt(),
                MotionType: JPC_MOTION_TYPE_DYNAMIC,
                ..Default::default()
            })
            .unwrap()
            .id();
        bodies.add_body(floor, JPC_ACTIVATION_DONT_ACTIVATE);
        bodies.add_body(ball, JPC_ACTIVATION_ACTIVATE);
        let allocator = JPC_TempAllocatorImpl_new(10 * 1024 * 1024);
        let jobs = JPC_JobSystemThreadPool_new2(2048, 8);
        for _ in 0..240 {
            world.update(1. / 60., 1, allocator, jobs);
        }
        let y = bodies.center_of_mass_position(ball).y;
        assert!((0.4..0.6).contains(&y), "settled y={y}");
        for id in [ball, floor] {
            bodies.remove_body(id);
            bodies.destroy_body(id);
        }
        drop(world);
        JPC_Shape::release(sphere.cast());
        JPC_Shape::release(terrain.cast());
        JPC_JobSystemThreadPool_delete(jobs);
        JPC_TempAllocatorImpl_delete(allocator);
    }
    unregister_types();
    factory_delete();
}
