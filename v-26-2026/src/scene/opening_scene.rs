use bevy::{
    asset::Assets,
    color::Color,
    ecs::system::{Commands, ResMut},
    light::PointLight,
    math::{
        Quat,
        primitives::{Circle, Cuboid},
    },
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

pub fn scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.))),
        MeshMaterial3d::<StandardMaterial>(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(1.0, 0.5, 4.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d::<StandardMaterial>(materials.add(Color::srgb_u8(255, 0, 0))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    /*
    below is the sample creation of assets using the bsn_List similar to the above code.
    */
    // bsn_list![
    //     (
    //         #CircularBase
    //         Mesh3d(asset_value(Circle::new(4.0)))
    //         MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
    //         Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
    //     ),
    //     (
    //         #Cube
    //         Mesh3d(asset_value(Cuboid::new(1.0,1.0,1.0)))
    //         MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
    //         Transform::from_xyz(0.0, 0.5, 0.0)
    //     ),
    //     (
    //      PointLight{
    //          shadow_maps_enabled: true,
    //      }
    //          Transform::from_xyz(1.0, 0.5, 4.0)
    //          )
    //      ]
}
