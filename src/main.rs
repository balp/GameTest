#![allow(clippy::type_complexity)]

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_talks::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    InteractiveFiction,
    TheEnd,
    // Battle,
}

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

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Resource)]
struct SimpleTalkAsset {
    handle: Handle<RawTalk>,
}

mod splash {
    use bevy::{asset::LoadState, prelude::*};

    use super::{despawn_screen, GameState, SimpleTalkAsset};

    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Splash), load_assets)
                .add_systems(
                    Update,
                    wait_assets_loaded.run_if(in_state(GameState::Splash)),
                )
                .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
        }
    }

    const DIALOG_FILE: &str = "dialog/intro.talk.ron";

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
}

mod fiction {
    use bevy::prelude::*;
    use bevy_talks::prelude::*;

    use super::{despawn_screen, GameState, SimpleTalkAsset, TEXT_COLOR};

    pub struct InteractiveFiction;

    impl Plugin for InteractiveFiction {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::InteractiveFiction), fiction_setup)
                .add_systems(
                    Update,
                    (
                        interact,
                        update_text,
                        update_fiction_color,
                        bevy::window::close_on_esc,
                    )
                        .run_if(in_state(GameState::InteractiveFiction)),
                )
                .add_systems(
                    OnExit(GameState::InteractiveFiction),
                    despawn_screen::<OnGameScreen>,
                );
        }
    }

    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Component)]
    struct FictionText;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    fn fiction_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        raws: Res<Assets<RawTalk>>,
        simple_sp_asset: Res<SimpleTalkAsset>,
        mut init_talk_events: EventWriter<InitTalkRequest>,
    ) {
        let raw_sp = raws.get(&simple_sp_asset.handle).unwrap();
        let talk = Talk::build(&raw_sp).unwrap();
        let e = commands.spawn(TalkerBundle { talk, ..default() }).id();
        init_talk_events.send(InitTalkRequest(e));

        let icon = asset_server.load("branding/icon.png"); // TODO: Move load to splash
        commands
            .spawn((
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
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::BLACK.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Press space to advance the conversation.",
                                TextStyle {
                                    font_size: 80.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                }),
                            FictionText,
                        ));
                        parent.spawn(ImageBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                ..default()
                            },
                            image: UiImage::new(icon),
                            ..default()
                        });
                    });
            });
        commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }

    fn interact(
        input: Res<Input<KeyCode>>,
        mut next_action_events: EventWriter<NextActionRequest>,
        talks: Query<Entity, With<Talk>>,
    ) {
        if input.just_pressed(KeyCode::Space) {
            let e = talks.single();
            next_action_events.send(NextActionRequest(e));
        }
    }

    fn update_text(
        mut query: Query<&mut Text, With<FictionText>>,
        mut next_action_events: EventWriter<NextActionRequest>,
        talks: Query<Entity, With<Talk>>,
        mut game_state: ResMut<NextState<GameState>>,
        talk_comps: Query<(
            Ref<CurrentText>,
            &CurrentActors,
            &CurrentNodeKind,
            &CurrentChoices,
        )>,
    ) {
        for (tt, ca, kind, _cc) in talk_comps.iter() {
            let text_line: Option<String> = if !tt.is_changed() || tt.is_added() {
                None
            } else {
                let actors =
                    ca.0.iter()
                        .map(|a| a.name.to_owned())
                        .collect::<Vec<String>>();

                let mut speaker = "Narrator";
                if actors.len() > 0 {
                    speaker = actors[0].as_str();
                }
                debug!("kind.0: {:?} {:?}", kind.0, tt.0);
                match kind.0 {
                    TalkNodeKind::Talk => Some(format!("{}: {}", speaker, tt.0)),
                    TalkNodeKind::Join => {
                        if actors.contains(&"observer".to_string()) {
                            let e = talks.single();
                            next_action_events.send(NextActionRequest(e));
                            None
                        } else {
                            Some(format!("--- {actors:?} enters the scene."))
                        }
                    }
                    TalkNodeKind::Leave => {
                        if actors.contains(&"observer".to_string()) {
                            info!("Exit dialog and continue along.");
                            game_state.set(GameState::TheEnd);
                            None
                        } else {
                            Some(format!("--- {actors:?} exit the scene."))
                        }
                    }
                    TalkNodeKind::Choice => Some("Choice".to_string()),
                }
            };
            if let Some(ref line) = text_line {
                debug!("text_line: {:?}", text_line);
                print!("{}", line.to_string());
            }
            for mut text in &mut query {
                if let Some(ref line) = text_line {
                    debug!("update: {:?} to {}", text, line);
                    text.sections[0].value = line.to_string();
                }
            }
        }
    }

    fn update_fiction_color(
        time: Res<Time>,
        mut query: Query<&mut Text, With<FictionText>>,
    ) {
        let seconds = time.elapsed_seconds();
        for mut text in &mut query {
            text.sections[0].style.color = Color::Rgba {
                red: (1.25 * seconds).sin() / 2.0 + 0.5,
                green: (0.75 * seconds).sin() / 2.0 + 0.5,
                blue: (0.50 * seconds).sin() / 2.0 + 0.5,
                alpha: 1.0,
            };
        }
    }
}

mod the_end {
    use bevy::prelude::*;

    use crate::{despawn_screen, GameState, TEXT_COLOR};

    pub struct TheEnd;

    impl Plugin for TheEnd {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::TheEnd), end_setup)
                .add_systems(
                    Update,
                    (bevy::window::close_on_esc).run_if(in_state(GameState::TheEnd)),
                )
                .add_systems(OnExit(GameState::TheEnd), despawn_screen::<EndScreen>);
        }
    }

    #[derive(Component)]
    struct EndScreen;

    fn end_setup(mut commands: Commands) {
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
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                EndScreen,
            ))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "The End.",
                        TextStyle {
                            font_size: 80.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                );
            });
    }
}
