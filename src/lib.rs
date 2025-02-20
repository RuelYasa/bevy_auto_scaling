use bevy::{
    app::{Plugin, Update}, core_pipeline::{core_2d::Camera2d, core_3d::Camera3d}, ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::Query,
    }, math::UVec2, render::camera::{Camera, OrthographicProjection}, window::{Window, WindowResized}
};

#[derive(Component)]
pub struct FixedSize {
    pub width: f32,
    pub height: f32,
}

pub struct Scale2dPlugin;

impl Plugin for Scale2dPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, adjust_camera_2d);
    }
}

pub struct Scale3dPlugin;

impl Plugin for Scale3dPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, adjust_camera_3d);
    }
}

fn adjust_camera_2d(
    mut e: EventReader<WindowResized>,
    mut cam: Query<(&FixedSize, &mut Camera, &mut OrthographicProjection), With<Camera2d>>,
    windows: Query<&Window>,
) {
    for (size, mut camera, mut projection) in cam.iter_mut() {
        for event in e.read() {
            let window = windows.get(event.window).unwrap();
            let (window_height, window_width) = (
                window.physical_height() as f32,
                window.physical_width() as f32,
            );
            if window_height / window_width > size.height / size.width {
                projection.scale = size.width / window_width * window.scale_factor();
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    window_width as u32,
                    (window_width / size.width * size.height) as u32,
                );
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            } else {
                projection.scale = size.height / window_height * window.scale_factor();
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    (window_height / size.height * size.width) as u32,
                    window_height as u32,
                );
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            }
        }
    }
}

fn adjust_camera_3d(
    mut e: EventReader<WindowResized>,
    mut cam: Query<(&FixedSize, &mut Camera), With<Camera3d>>,
    windows: Query<&Window>,
) {
    for (size, mut camera) in cam.iter_mut() {
        for event in e.read() {
            let window = windows.get(event.window).unwrap();
            let (window_height, window_width) = (
                window.physical_height() as f32,
                window.physical_width() as f32,
            );
            if window_height / window_width > size.height / size.width {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    window_width as u32,
                    (window_width / size.width * size.height) as u32,
                );
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            } else {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    (window_height / size.height * size.width) as u32,
                    window_height as u32,
                );
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            }
        }
    }
}
