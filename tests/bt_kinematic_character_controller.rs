//extern crate bullet_rs;
extern crate bullet_sys;

use bullet_sys::bullet::*;

#[test()]
fn kinematic_character_controller() {
    // GTEST_TEST(BulletDynamics, KinematicCharacterController)
    unsafe {
        let mut ghost_object = btPairCachingGhostObject::new();
        let box_half_extents = btVector3::new1(&1f64, &1f64, &1f64);
        let mut convex_shape = btBoxShape::new(&box_half_extents);

        let up = btVector3::new1(&1f64,&0f64,&0f64);
        let tested = btKinematicCharacterController::new(&mut ghost_object, &mut convex_shape._base._base._base, 1f64, &up);

        assert_eq!(-9.8 * 3.0, *tested.getGravity().x());
        assert_eq!(0.0, *tested.getGravity().y());
        assert_eq!(0.0, *tested.getGravity().z());
    }
}
