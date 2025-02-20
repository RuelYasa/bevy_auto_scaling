use bevy::{
    app::{App, Startup, Update}, asset::Assets, color::palettes::css::{BLUE, WHITE}, core_pipeline::core_2d::Camera2d, ecs::{query::With, system::{Commands, Res, ResMut, Single}
    }, math::primitives::Rectangle, render::{
        camera::{Camera, ClearColorConfig},
        mesh::{Mesh, Mesh2d},
    }, sprite::{ColorMaterial, MeshMaterial2d}, time::Time, transform::components::Transform, DefaultPlugins
};

use bevy_auto_scaling::{FixedSize, Scale2dPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Scale2dPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, spin)
        .run();
}

fn init(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = materials.add(ColorMaterial::from_color(BLUE));
    let mesh = meshes.add(Rectangle::new(100.0, 100.0));
    cmd.spawn((Mesh2d(mesh), MeshMaterial2d(material)));

    cmd.spawn((
        Camera2d::default(),
        FixedSize {
            width: 200.0,
            height: 200.0,
        },
        Camera {
            clear_color: ClearColorConfig::Custom(WHITE.into()),
            ..Default::default()
        },
    ));
}

fn spin(time:Res<Time>,mut obj:Single<&mut Transform, With<Mesh2d>>){
    obj.rotate_z(time.delta_secs());
}