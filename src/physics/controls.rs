use bevy::{
    ecs::system::Res,
    input::{keyboard::KeyCode, ButtonInput},
    log::info,
    prelude::ResMut,
};
use bevy_rapier3d::{prelude::RapierConfiguration, render::DebugRenderContext};

pub fn pause_physics(mut physics_config: ResMut<RapierConfiguration>) {
    info!("pausing physics");
    physics_config.physics_pipeline_active = false;
}

pub fn resume_physics(mut physics_config: ResMut<RapierConfiguration>) {
    info!("unpausing physics");
    physics_config.physics_pipeline_active = true;
}

#[allow(clippy::needless_pass_by_value)]
pub fn toggle_physics_debug(
    mut debug_config: ResMut<DebugRenderContext>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    if keycode.just_pressed(KeyCode::F1) {
        debug_config.enabled = !debug_config.enabled;
    }
}
