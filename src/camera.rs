use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::experimental::taa::{TemporalAntiAliasBundle, TemporalAntiAliasPlugin};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceReflectionsBundle};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PanOrbitCameraPlugin, TemporalAntiAliasPlugin));
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    let translation = Vec3::new(2.0, 0.5, 7.0);
    let focus = Vec3::new(0.0, 0.0, 0.0);
    let radius = Some((translation - focus).length());

    let mut command = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(focus, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.001,
                far: 5.0,
                ..default()
            }),
            tonemapping: Tonemapping::BlenderFilmic,
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        BloomSettings::NATURAL,
        PanOrbitCamera {
            focus,
            radius,
            button_pan: MouseButton::Right,
            button_orbit: MouseButton::Middle,
            orbit_smoothness: 0.0,
            pan_smoothness: 0.0,
            zoom_smoothness: 0.0,
            ..default()
        },
    ));
    command.insert(ScreenSpaceAmbientOcclusionBundle::default());
    command.insert(ScreenSpaceReflectionsBundle::default());
    #[cfg(not(target_arch = "wasm32"))]
    command.insert(TemporalAntiAliasBundle::default());
}
