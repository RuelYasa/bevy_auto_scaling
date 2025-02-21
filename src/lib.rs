use bevy::{
    app::{Plugin, Update},
    ecs::{component::Component, event::EventReader,  system::Query},
    math::UVec2,
    render::camera::{Camera, OrthographicProjection, ScalingMode},
    window::{Window, WindowResized},
};

/// Fixed aspect ratio of the camera.
/// ratio=width/height
#[derive(Component)]
pub struct AspectRatio(pub f32);

/// The plugin of the plugin.
/// Scale the view of cameras with AspectRatio component to fit the window.
pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, adjust_camera);
    }
}

fn adjust_camera(
    mut e: EventReader<WindowResized>,
    mut cam: Query<(&AspectRatio, &mut Camera)>,
    windows: Query<&Window>,
) {
    for (ratio, mut camera) in cam.iter_mut() {
        for event in e.read() {
            let window = windows.get(event.window).unwrap();
            let (window_height, window_width) = (
                window.physical_height() as f32,
                window.physical_width() as f32,
            );
            if  window_width/window_height < ratio.0 {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    window_width as u32,
                    (window_width / ratio.0) as u32,
                );
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            } else {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size = UVec2::new(
                    (window_height * ratio.0) as u32,
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

/// return an OrthographicProjection with scaling mode of given size.
/// For 2d cameras.
pub fn fixed_size_2d(width:f32,height:f32) -> OrthographicProjection{
    return OrthographicProjection{
        near:-1000.0, 
        scaling_mode: ScalingMode::Fixed { width: width, height: height },
        ..OrthographicProjection::default_3d()
    };
}

/// return an OrthographicProjection with scaling mode of given size.
/// For orthographic 3d cameras.
pub fn fixed_size_3d(width:f32,height:f32) -> OrthographicProjection{
    return OrthographicProjection{
        scaling_mode: ScalingMode::Fixed { width: width, height: height },
        ..OrthographicProjection::default_3d()
    };
}