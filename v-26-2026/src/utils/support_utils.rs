#[cfg(debug_assertions)]
use bevy::{
    camera::{Camera, Camera3d},
    ecs::{query::With, system::Single},
    math::Vec2,
    transform::components::GlobalTransform,
    window::Window,
};
use bevy::{
    color::Color,
    gizmos::gizmos::Gizmos,
    math::{Isometry3d, UVec3, Vec3},
};

#[allow(unused_imports)]
use bevy::transform::components::Transform;

#[cfg(debug_assertions)]
pub fn dev_pointer_grip(
    mut gismoz: Gizmos,
    camera_q: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    window: Single<&Window>,
) {
    /*
    this code created a non-interferring gizmo cube in the scene.
    its a sample wireframe cube.

    gismoz.cube(
        Transform::from_translation(Vec3::X * 0.).with_scale(Vec3::splat(0.)),
        Color::WHITE,
    );
    */

    let (camera, camera_transform) = camera_q.into_inner();
    let ray = match camera.viewport_to_world(
        camera_transform,
        Vec2::new(window.width() * 0.95, window.height() * 0.1),
    ) {
        Ok(r) => r,
        Err(_) => return,
    };
    let anchor = ray.origin + ray.direction * 8.0;
    gismoz
        .grid_3d(
            Isometry3d::from_translation(anchor),
            UVec3::new(1, 1, 1),
            Vec3::splat(0.5),
            Color::srgb(0.71, 0.37, 0.21),
        )
        .outer_edges();
}
