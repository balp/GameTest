#![allow(clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_talks::prelude::*;

mod asset_loader;
mod end_scene;
mod interactive_fiction;
mod states;
mod utils;


const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,game_test=debug".into(),
                level: bevy::log::Level::DEBUG,
            }),
            TalksPlugin,
        ))
        .add_state::<states::GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            asset_loader::AssetLoader,
            interactive_fiction::InteractiveFiction,
            end_scene::TheEnd,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
