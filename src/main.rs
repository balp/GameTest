#![allow(clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::prelude::*;

mod asset_loader;
mod battle;
mod characters;
mod end_scene;
mod schedule;
mod states;
mod utils;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,game_test=debug".into(),
                level: bevy::log::Level::DEBUG,
                ..default()
            }),
        ))
        .init_state::<states::GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            asset_loader::AssetLoader,
            end_scene::TheEnd,
            battle::Battle,
        ))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(1024.0, 0.0, 0.0),
            ..default()
        },
        MainCamera,
    ));
}
