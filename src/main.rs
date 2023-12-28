#![allow(clippy::type_complexity)]

mod end_scene;
mod interactive_fiction;
mod asset_loader;
mod states;
mod utils;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_talks::prelude::*;
use end_scene::the_end;
use interactive_fiction::fiction;
use states::GameState;
use crate::asset_loader::splash;


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
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((
            splash::SplashPlugin,
            fiction::InteractiveFiction,
            the_end::TheEnd,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
