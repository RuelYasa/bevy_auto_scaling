use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        event::EventReader,
        resource::Resource,
        schedule::{IntoScheduleConfigs, common_conditions::resource_exists},
        system::{Query, Res, ResMut},
    },
    math::UVec2,
    render::camera::{Camera, OrthographicProjection, Projection, RenderTarget, ScalingMode},
    ui::UiScale,
    window::{PrimaryWindow, Window, WindowRef, WindowResized},
};

/// Fixed aspect ratio of the camera.
/// ratio=width/height
#[derive(Component)]
pub struct AspectRatio(pub f32);

/// Logical size of window, for UI use.
/// Typically matches the sizes of cameras.
/// Insert it to enable scaling on UI. Will take over UiScale resource.
#[derive(Resource)]
pub struct ScalingUI {
    pub width: f32,
    pub height: f32,
}

/// The plugin of the plugin.
/// Scale the view of cameras with AspectRatio component to fit the window.
pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Update, adjust_camera);
        app.add_systems(Update, adjust_ui.run_if(resource_exists::<ScalingUI>));
    }
}

fn adjust_camera(
    mut e: EventReader<WindowResized>,
    mut cam: Query<(&AspectRatio, &mut Camera)>,
    windows: Query<&Window>,
    primary: Query<&PrimaryWindow>,
) {
    for event in e.read() {
        for (ratio, mut camera) in cam.iter_mut() {
            let RenderTarget::Window(rref) = camera.target else {
                continue;
            };
            if let WindowRef::Primary = rref {
                if !primary.contains(event.window) {
                    continue;
                }
            } else if let WindowRef::Entity(e) = rref {
                if e != event.window {
                    continue;
                }
            }
            let window = windows.get(event.window).unwrap();
            let (window_height, window_width) = (
                window.physical_height() as f32,
                window.physical_width() as f32,
            );
            if window_width / window_height < ratio.0 {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size =
                    UVec2::new(window_width as u32, (window_width / ratio.0) as u32);
                viewport.physical_position = UVec2::new(
                    (window_width / 2.0) as u32 - viewport.physical_size.x / 2,
                    (window_height / 2.0) as u32 - viewport.physical_size.y / 2,
                );
            } else {
                let viewport = camera.viewport.get_or_insert_default();
                viewport.physical_size =
                    UVec2::new((window_height * ratio.0) as u32, window_height as u32);
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
pub fn fixed_size_2d(width: f32, height: f32) -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::Fixed { width, height },
        ..OrthographicProjection::default_2d()
    })
}

/// return an OrthographicProjection with scaling mode of given size.
/// For orthographic 3d cameras.
pub fn fixed_size_3d(width: f32, height: f32) -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::Fixed { width, height },
        ..OrthographicProjection::default_3d()
    })
}

fn adjust_ui(
    mut e: EventReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
    logic_size: Res<ScalingUI>,
    windows: Query<&Window>,
) {
    let ratio = logic_size.width / logic_size.height;
    for event in e.read() {
        let window = windows.get(event.window).unwrap();
        let (window_height, window_width) = (
            window.physical_height() as f32,
            window.physical_width() as f32,
        );
        if window_height == 0.0 && window_width == 0.0 {
            continue;
        }
        if window_width / window_height < ratio {
            ui_scale.0 = window_width / logic_size.width;
        } else {
            ui_scale.0 = window_height / logic_size.height;
        }
    }
}
