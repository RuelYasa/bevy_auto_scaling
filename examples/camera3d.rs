use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::Assets,
    camera::{Camera, Camera3d, ClearColorConfig},
    color::palettes::css::{BLUE, WHITE},
    ecs::{
        query::With,
        system::{Commands, Res, ResMut, Single},
    },
    light::PointLight,
    math::{Vec3, primitives::Cuboid},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    time::Time,
    transform::components::Transform,
};

use bevy_auto_scaling::{AspectRatio, ScalePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScalePlugin)
        .add_systems(Startup, init)
        .add_systems(Update, spin)
        .run();
}

fn init(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(StandardMaterial::from_color(BLUE));
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    cmd.spawn((Mesh3d(mesh), MeshMaterial3d(material)));

    cmd.spawn((
        Camera3d::default(),
        PointLight::default(),
        AspectRatio(1.5),
        Transform::from_translation(Vec3::new(4.0, 1.0, 2.0)).looking_at(Vec3::ZERO, Vec3::Z),
        Camera {
            clear_color: ClearColorConfig::Custom(WHITE.into()),
            ..Default::default()
        },
    ));
}

fn spin(time: Res<Time>, mut obj: Single<&mut Transform, With<Mesh3d>>) {
    obj.rotate_x(time.delta_secs());
    obj.rotate_z(time.delta_secs() * 1.3);
}
