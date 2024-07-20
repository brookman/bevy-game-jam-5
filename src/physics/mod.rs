pub(crate) mod physics_replace_proxies;

use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
pub(crate) use physics_replace_proxies::*;

pub(crate) mod utils;

pub(crate) mod controls;

pub(crate) use controls::*;

use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use blenvy::GltfBlueprintsSet;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .register_type::<AutoAABBCollider>()
        .register_type::<Collider>()
        .add_systems(
            Update,
            physics_replace_proxies.after(GltfBlueprintsSet::AfterSpawn),
        )
        .add_systems(Update, toggle_physics_debug);
        // .add_systems(OnEnter(GameState::InGame), resume_physics)
        // .add_systems(OnExit(GameState::InGame), pause_physics);
    }
}
