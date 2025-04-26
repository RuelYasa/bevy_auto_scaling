use bevy::{
    DefaultPlugins,
    app::{App, Startup, Update},
    asset::Assets,
    color::palettes::css::{BLUE, WHITE},
    core_pipeline::core_2d::Camera2d,
    ecs::{
        query::With,
        system::{Commands, Res, ResMut, Single},
    },
    math::primitives::Rectangle,
    render::{
        camera::{Camera, ClearColorConfig},
        mesh::{Mesh, Mesh2d},
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    text::{TextColor, TextFont},
    time::Time,
    transform::components::Transform,
    ui::{Node, widget::Text},
};

use bevy_auto_scaling::{AspectRatio, ScalePlugin, ScalingUI, fixed_size_2d};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScalePlugin)
        .insert_resource(ScalingUI {
            width: 300.0,
            height: 200.0,
        })
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
        Node {
            top: bevy::ui::Val::Px(10.0),
            left: bevy::ui::Val::Px(0.0),
            ..Default::default()
        },
        TextColor::BLACK,
        TextFont::from_font_size(10.0),
        Text::new("The quick fox jumps over a brown lazy dog."),
    ));

    cmd.spawn((
        Camera2d::default(),
        AspectRatio(1.5),
        fixed_size_2d(300.0, 200.0),
        Camera {
            clear_color: ClearColorConfig::Custom(WHITE.into()),
            ..Default::default()
        },
    ));
}

fn spin(time: Res<Time>, mut obj: Single<&mut Transform, With<Mesh2d>>) {
    obj.rotate_z(time.delta_secs());
}
