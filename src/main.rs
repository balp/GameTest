#![allow(clippy::type_complexity)]

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    InteractiveFiction,
    // Battle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::SplashPlugin, fiction::InteractiveFiction))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

mod splash {
    use bevy::prelude::*;
    use super::{despawn_screen, GameState};

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(GameState::Splash), splash_setup)
                .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
                .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
        }
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::InteractiveFiction);
        }
    }
}

mod fiction {
    use bevy::prelude::*;
    use super::{despawn_screen, GameState, TEXT_COLOR};

    pub struct InteractiveFiction;

    impl Plugin for InteractiveFiction {
        fn build(&self, app: &mut App) {
            app
                .add_systems(OnEnter(GameState::InteractiveFiction), fiction_setup)
                .add_systems(Update, update_fiction.run_if(in_state(GameState::InteractiveFiction)))
                .add_systems(OnExit(GameState::InteractiveFiction), despawn_screen::<OnGameScreen>);
        }
    }

    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn fiction_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("branding/icon.png");
        commands.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        )).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "This will be a story shorty...",
                        TextStyle {
                            font_size: 80.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }),
                );
                parent.spawn(
                    ImageBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            ..default()
                        },
                        image: UiImage::new(icon),
                        ..default()
                    }
                );
            });
        });
        commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }


    fn update_fiction(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timer: ResMut<GameTimer>,
    ) {}
}