use bevy::asset::Handle;
use bevy::prelude::Resource;
use bevy::{asset::LoadState, prelude::*};
use bevy_talks::prelude::RawTalk;

use crate::states::GameState;

#[derive(Resource)]
pub struct SimpleTalkAsset {
    pub(crate) handle: Handle<RawTalk>,
}

const DIALOG_FILE: &str = "dialog/intro.talk.ron";

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), load_assets)
            .add_systems(
                Update,
                wait_assets_loaded.run_if(in_state(GameState::Splash)),
            )
            .add_systems(
                OnExit(GameState::Splash),
                crate::utils::despawn_screen::<OnSplashScreen>,
            );
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let intro_talk = asset_server.load(DIALOG_FILE);
    commands.insert_resource(SimpleTalkAsset { handle: intro_talk });

    let icon = asset_server.load("branding/icon.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn wait_assets_loaded(
    server: Res<AssetServer>,
    simple_sp_asset: Res<SimpleTalkAsset>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if let Some(load_state) = server.get_load_state(&simple_sp_asset.handle) {
        if load_state == LoadState::Loaded {
            info!("Loaded dialog asset: {}", DIALOG_FILE);
            game_state.set(GameState::InteractiveFiction);
        } else if load_state == LoadState::Failed {
            error!("Unable to load dialog: {}", DIALOG_FILE);
            game_state.set(GameState::InteractiveFiction);
        } else if load_state == LoadState::NotLoaded {
            warn!("Not loaded state for {}", DIALOG_FILE);
        }
    }
    if timer.tick(time.delta()).finished() {
        error!("Time out loading: {}", DIALOG_FILE);
        game_state.set(GameState::InteractiveFiction);
    }
}
