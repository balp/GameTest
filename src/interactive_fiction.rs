use bevy::prelude::*;
use bevy_talks::prelude::*;
use rand::Rng;
use crate::asset_loader::DialogTalkAsset;
use crate::characters::{CharacterName, PortraitAtlasId, CharacterSkills};
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
                    update_actors_in_scene,
                    // update_fiction_color,
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

#[derive(Component)]
struct InScene;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn fiction_setup(
    mut commands: Commands,
    talks: Res<Assets<TalkData>>,
    simple_sp_asset: Res<DialogTalkAsset>,
) {
    let intro_dialog = talks.get(&simple_sp_asset.intro_dialog).unwrap();
    let talk_builder = TalkBuilder::default().fill_with_talk_data(intro_dialog);
    let mut talk_commands = commands.talks();
    talk_commands.spawn_talk(talk_builder, ());

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
                                font: simple_sp_asset.fiction_font.clone(),
                                font_size: 40.0,
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
                        SpeakerLogo,
                    ));
                });
        });
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn interact(
    input: Res<Input<KeyCode>>,
    mut next_action_events: EventWriter<NextActionRequest>,
    mut choose_action_events: EventWriter<ChooseActionRequest>,
    talks: Query<(Entity, &Talk)>,
    actors: Query<(Entity, &CharacterName, &CharacterSkills), With<InScene>>,
) {
    if let Ok((talk_ent, talk)) = talks.get_single() {
        let selection_keys = vec![KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4,
                                  KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9,
                                  KeyCode::Key0];
        if talk.current_kind == NodeKind::Choice {
            // debug!("select:: {:?}", talk.current_choices);
            let no_choices = talk.current_choices.len();
            for n in 0..no_choices {
                if input.just_pressed(selection_keys[n]) {
                    debug!("just pressed choice {n} -> {:?}", &talk.current_choices[n]);
                    if let Some(check) = &talk.current_choices[n].check {
                        for (entity, character, skills) in actors.iter() {
                            debug!("check({:?}, {:?}, {:?})", check, character, skills);
                            let mut rng = rand::thread_rng();
                            let result = match check.as_str() {
                                "agility" => { rng.gen_range(1..=100) < skills.agility.value }
                                "sneak" => { rng.gen_range(1..=100) < skills.sneak.value }
                                &_ => {false}
                            };
                            if result {
                                debug!("Skill check passed, go next");
                            } else {
                                debug!("Skill check failed!, option failed");
                            }
                        }
                    }
                    continue;
                }
            }
            if input.just_pressed(KeyCode::Key1) {
                choose_action_events.send(ChooseActionRequest::new(
                    talk_ent,
                    talk.current_choices[0].next,
                ));
            } else if input.just_pressed(KeyCode::Key2) {
                choose_action_events.send(ChooseActionRequest::new(
                    talk_ent,
                    talk.current_choices[1].next,
                ));
            };
        } else if input.just_pressed(KeyCode::Space) {
            next_action_events.send(NextActionRequest(talk_ent));
        }
    } else {}
}

fn update_text(
    mut query: Query<&mut Text, With<FictionText>>,
    mut next_action_events: EventWriter<NextActionRequest>,
    talks: Query<Entity, With<Talk>>,
    mut game_state: ResMut<NextState<GameState>>,
    talk_comps: Query<Ref<Talk>>,
    characters: Query<(&CharacterName)>,
) {
    for talk in &talk_comps {
        if !talk.is_changed() || talk.is_added() {
            continue;
        }
        debug!("Talk: {:?}", talk);
        let text_line: Option<String> = {
            let actors = &talk.current_actors;
            let current_actor = if talk.current_actors.is_empty() {
                "narrator"
            } else {
                talk.current_actors[0].as_str()
            };

            let mut speaker = "";
            for name in characters.iter() {
                if name.slug == current_actor {
                    speaker = name.alias.as_str();
                }
            }

            match talk.current_kind {
                NodeKind::Start => None,
                NodeKind::Talk => Some(format!("{}:\n{}", speaker, talk.current_text)),
                NodeKind::Join => {
                    if actors.contains(&"observer".to_string()) {
                        let e = talks.single();
                        next_action_events.send(NextActionRequest(e));
                        None
                    } else {
                        Some(format!("--- {actors:?} enters the scene."))
                    }
                }
                NodeKind::Leave => {
                    if actors.contains(&"observer".to_string()) {
                        info!("Exit dialog and continue along.");
                        game_state.set(GameState::TheEnd);
                        None
                    } else {
                        Some(format!("--- {actors:?} exit the scene."))
                    }
                }
                NodeKind::Choice => {
                    let mut prompt = "Choice:".to_owned();
                    for (i, choice) in talk.current_choices.iter().enumerate() {
                        prompt.push_str("\n");
                        prompt.push_str(&(i + 1).to_string());
                        prompt.push_str(": ");
                        prompt.push_str(&choice.text);
                        println!("{}: {}", i + 1, choice.text);
                    }
                    Some(prompt)
                }
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

fn update_actors_in_scene (
    mut commands: Commands,
    characters: Query<(Entity, &CharacterName)>,
    talk_comps: Query<Ref<Talk>>,
) {
    for talk in &talk_comps {
        if !talk.is_changed() || talk.is_added() {
            continue;
        }
        match talk.current_kind {
            NodeKind::Start => {}
            NodeKind::Talk => {}
            NodeKind::Choice => {}
            NodeKind::Join => {
                for (entity, character) in characters.iter() {
                    debug!("Join node: {:?} {:?}", entity, character);
                    if talk.current_actors.contains(&character.slug) {
                        debug!("Adding: {:?}", character);
                        commands.entity(entity).insert(InScene);
                    }
                }

            }
            NodeKind::Leave => {
                for (entity, character) in characters.iter() {
                    debug!("Leave node: {:?} {:?}", entity, character);
                    if talk.current_actors.contains(&character.slug) {
                        debug!("Removing: {:?}", character);
                        commands.entity(entity).remove::<InScene>();
                    }
                }
            }
        }
    }
}

fn update_speaker_logo(
    mut atlas_images: Query<&mut UiTextureAtlasImage>,
    talk_comps: Query<Ref<Talk>>,
    characters: Query<(&CharacterName, &PortraitAtlasId)>,
) {
    for talk in &talk_comps {
        if !talk.is_changed() || talk.is_added() {
            continue;
        }

        let current_actor = if talk.current_kind == NodeKind::Join
            || talk.current_kind == NodeKind::Start
            || talk.current_kind == NodeKind::Leave {
            "empty"
        } else if talk.current_actors.is_empty() {
            "narrator"
        } else {
            talk.current_actors[0].as_str()
        };
        let mut index: usize = 0; // Hack as stuff happens to be in folder for now
        for (name, portrait) in characters.iter() {
            if name.slug == current_actor {
                index = portrait.index;
                continue;
            }
        }
        for mut atlas_image in &mut atlas_images {
            debug!("Update atlas image for {:?}, {:?}", atlas_image, index);
            atlas_image.index = index;
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
