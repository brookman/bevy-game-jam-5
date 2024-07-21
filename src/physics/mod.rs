use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use blenvy::GltfBlueprintsSet;

use physics_replace_proxies::{AutoAABBCollider, Collider, physics_replace_proxies};

use crate::physics::controls::{pause_physics, resume_physics, toggle_physics_debug};
use crate::state::Game;

mod physics_replace_proxies;

mod utils;

mod controls;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
        ))
        .register_type::<AutoAABBCollider>()
        .register_type::<Collider>()
        .add_systems(
            Update,
            physics_replace_proxies.after(GltfBlueprintsSet::AfterSpawn),
        )
        .add_systems(Update, toggle_physics_debug)
        .add_systems(OnEnter(Game::InGame), resume_physics)
        .add_systems(OnExit(Game::InGame), pause_physics);
    }
}
