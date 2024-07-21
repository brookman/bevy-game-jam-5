use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use blenvy::{BlenvyPlugin, BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

mod camera;
mod light;
mod physics;
mod state;
mod ui;

static APP_NAME: &str = "Bevy Jam #5";

// #[derive(Component, Reflect, Default, Debug)]
// #[reflect(Component)]
// pub struct MyHealth {
//     hp: f32,
//     extra: f32,
// }

fn main() {
    let mut app = App::new();

    // Default plugins -------------------------------------

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let log_plugin = bevy::log::LogPlugin {
        level: bevy::log::Level::INFO,
        ..default()
    };
    let asset_plugin = AssetPlugin {
        watch_for_changes_override: Some(true),
        meta_check: AssetMetaCheck::Never,
        ..Default::default()
    };

    let default_plugins = DefaultPlugins
        .set(window_plugin)
        .set(log_plugin)
        .set(asset_plugin);

    // -----------------------------------------------------

    let blenvy_plugin = BlenvyPlugin {
        aabbs: true, //  automatically calculate aabb for the scene/blueprint
        ..Default::default()
    };
    app.add_plugins((
        default_plugins,
        blenvy_plugin,
        WorldInspectorPlugin::new(),
        state::Plugin,
        ui::Plugin,
        camera::Plugin,
        physics::Plugin,
        // light::Plugin,
    ));
    // app.register_type::<MyHealth>();
    app.add_systems(Startup, setup_game);
    app.insert_resource(Msaa::Off); // Disable MSAA because it's incompatible with SSAO
                                    // app.insert_resource(DefaultOpaqueRendererMethod::deferred());
    app.run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/Scene.glb"), // all we need is a Blueprint info...
        SpawnBlueprint, // and spawnblueprint to tell blenvy to spawn the blueprint now
        HideUntilReady, // only reveal the  level once it is ready
        GameWorldTag,
    ));
}
