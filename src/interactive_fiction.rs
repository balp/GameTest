use bevy::prelude::*;
use bevy_talks::prelude::*;

use crate::asset_loader::SimpleTalkAsset;
use crate::states::GameState;
use crate::TEXT_COLOR;
use crate::utils::despawn_screen;

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
                    update_speaker_logo,
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

#[derive(Component)]
struct SpeakerLogo;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn fiction_setup(
    mut commands: Commands,
    raws: Res<Assets<RawTalk>>,
    simple_sp_asset: Res<SimpleTalkAsset>,
    mut init_talk_events: EventWriter<InitTalkRequest>,
) {
    let raw_sp = raws.get(&simple_sp_asset.intro_dialog).unwrap();
    let talk = Talk::build(raw_sp).unwrap();
    let e = commands.spawn(TalkerBundle { talk, ..default() }).id();
    init_talk_events.send(InitTalkRequest(e));

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
                    parent.spawn((
                        AtlasImageBundle {
                            style: Style {
                                width: Val::Px(256.0),
                                height: Val::Px(256.0),
                                ..default()
                            },
                            texture_atlas: simple_sp_asset.portrait_atlas.clone(),
                            texture_atlas_image: UiTextureAtlasImage::default(),
                            ..default()
                        },
                        SpeakerLogo));
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
            if !actors.is_empty() {
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
            print!("{}", line);
        }
        for mut text in &mut query {
            if let Some(ref line) = text_line {
                debug!("update: {:?} to {}", text, line);
                text.sections[0].value = line.to_string();
            }
        }
    }
}

fn update_speaker_logo(
    mut atlas_images: Query<&mut UiTextureAtlasImage>,
    talk_comps: Query<(
        Ref<CurrentText>,
        &CurrentActors,
        &CurrentNodeKind,
        &CurrentChoices,
    )>,
) {
    for (tt, ca, _kind, _cc) in talk_comps.iter() {
        if !tt.is_changed() || tt.is_added() {
            continue;
        }
        let actors =
            ca.0.iter()
                .map(|a| a.name.to_owned())
                .collect::<Vec<String>>();

        let mut speaker = "Narrator";
        if !actors.is_empty() {
            speaker = actors[0].as_str();
        }
        for mut atlas_image in &mut atlas_images {
            debug!("Update atlas image for {:?}", atlas_image);
            match speaker {
                "Narrator" => { atlas_image.index = 0; }
                "Ferris" => { atlas_image.index = 1; }
                "Bevy" => { atlas_image.index = 2; }
                &_ => {}
            }
        }
    }


}
fn update_fiction_color(time: Res<Time>, mut query: Query<&mut Text, With<FictionText>>) {
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

