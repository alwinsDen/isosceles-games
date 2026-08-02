use bevy::{
    camera::{Camera3d, OrthographicProjection, Projection, ScalingMode},
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};

/// definition of the global camera that will be used across entirity of the game.
pub fn camera_global_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
