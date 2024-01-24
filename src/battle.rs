use std::vec::Vec;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::asset_loader::BattleAsset;
use crate::characters::{CharacterName, CharacterSkills, Initiative, NoName, PortraitAtlasId};
use crate::schedule::BattleUpdateSets;
use crate::states::GameState;
use crate::utils::despawn_screen;
use crate::MainCamera;

pub struct Battle;

impl Plugin for Battle {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            .add_systems(OnEnter(GameState::Battle), battle_setup)
            .add_systems(
                OnExit(GameState::Battle),
                (set_starting_initiative, render_zones, resolve_zones),
            )
            .add_systems(OnEnter(GameState::BattleTurns), action_menu)
            .add_systems(
                Update,
                (enable_buttons,)
                    .run_if(in_state(GameState::BattleTurns))
                    .in_set(BattleUpdateSets::TurnChanges),
            )
            .add_systems(
                Update,
                (my_cursor_system, button_system)
                    .run_if(in_state(GameState::BattleTurns))
                    .in_set(BattleUpdateSets::UserInput),
            )
            .add_systems(
                Update,
                (
                    show_initiative,
                    show_button_state,
                    draw_icons_in_zone,
                    bevy::window::close_on_esc,
                )
                    .run_if(in_state(GameState::BattleTurns))
                    .in_set(BattleUpdateSets::EntityUpdates),
            )
            .add_systems(
                Update,
                (draw_icons_in_zone, bevy::window::close_on_esc)
                    .run_if(in_state(GameState::BattleEnded)),
            )
            .add_systems(
                OnExit(GameState::BattleEnded),
                despawn_screen::<OnBattleScreen>,
            );
    }
}

#[derive(Component)]
struct OnBattleScreen;

#[derive(Component)]
struct InitiativeSprite;

#[derive(Component)]
struct CurrentInitiative;

#[derive(Component)]
struct MoveButton;

#[derive(Component)]
struct ExtraMoveButton;

#[derive(Component)]
struct AttackButton;

#[derive(Component)]
struct SwitchButton;

#[derive(Component)]
struct GetUpButton;

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct ButtonPressed;

#[derive(Component)]
struct ButtonHoover;

#[derive(Component)]
struct ButtonEnabled;

#[derive(Resource, Default, Debug)]
struct MyWorldCoords(Vec2);

#[derive(Component, Debug)]
pub struct ZoneArea {
    center: Vec3,
    size: Vec2,
}

#[derive(Component, Debug)]
pub struct ZoneName {
    tag: String,
    name: String,
}

impl ZoneName {
    pub fn new(tag: &str, name: &str) -> Self {
        Self {
            tag: tag.to_string(),
            name: name.to_string(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct Zone {
    position: ZoneArea,
    name: ZoneName,
}

#[derive(Component, Debug)]
pub struct InZone {
    name: String,
    area: Option<Entity>,
}

impl InZone {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            area: None,
        }
    }
}

fn battle_setup(
    mut commands: Commands,
    battle_asset: Res<BattleAsset>,
    mut windows: Query<&mut Window>,
    mut characters: Query<(Entity, &CharacterName, &CharacterSkills, &PortraitAtlasId)>,
    mut director_characters: Query<(Entity, &NoName, &PortraitAtlasId)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("battle_setup...");
    let mut window = windows.single_mut();
    window.resolution.set(2048.0, 1024.0);

    let texture = battle_asset.maps[1].clone();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(1024., 0., 0.)),
            ..default()
        },
        OnBattleScreen,
    ));

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(1900.0, 400.0, 2.0),
                ..default()
            },
            sprite: TextureAtlasSprite::default(),
            texture_atlas: battle_asset.portrait_atlas.clone(),
            ..default()
        },
        InitiativeSprite,
        OnBattleScreen,
    ));

    add_zone(
        &mut commands,
        222.,
        142.,
        190.,
        100.,
        "cell_a_11",
        "Cell A11",
    );
    add_zone(
        &mut commands,
        360.,
        142.,
        190.,
        100.,
        "cell_a_12",
        "Cell A12",
    );
    add_zone(
        &mut commands,
        495.,
        142.,
        190.,
        100.,
        "cell_a_13",
        "Cell A13",
    );

    add_zone(
        &mut commands,
        222.,
        -142.,
        190.,
        100.,
        "cell_a_21",
        "Cell A21",
    );
    add_zone(
        &mut commands,
        360.,
        -142.,
        190.,
        100.,
        "cell_a_22",
        "Cell A22",
    );
    add_zone(
        &mut commands,
        495.,
        -142.,
        190.,
        100.,
        "cell_a_23",
        "Cell A23",
    );

    add_zone(
        &mut commands,
        330.,
        0.,
        90.,
        470.,
        "cellblock_a",
        "Cell block A",
    );
    add_zone(&mut commands, 630., 0., 140., 120., "gate_a", "Gate A");
    add_zone(&mut commands, 990., 0., 640., 580., "central", "Central");
    add_zone(
        &mut commands,
        1530.,
        0.,
        90.,
        470.,
        "access_corridor",
        "Access Corridor",
    );

    for (entity, name, skills, _portait_id) in characters.iter() {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=100);
        let tens = roll / 10u8;
        let once = roll % 10u8;
        let initiative: u8 = match skills.alertness.value {
            x if x <= roll => {
                if tens == once {
                    tens + once + 10
                } else {
                    tens + once
                }
            }
            x if x > roll => {
                if tens == once {
                    0
                } else {
                    once
                }
            }
            _ => 0,
        };
        debug!("Rolled initiative {:?} for {:?}", initiative, name);
        let character_initiative = Initiative::new(initiative);
        commands.entity(entity).insert(character_initiative);
    }
    for (entity, name, _skills, _portait_id) in characters.iter() {
        debug!("Adding player character to {:?}::{:?} map", entity, name);
        commands.entity(entity).insert(InZone::new("cell_a_13"));
    }
    let mut x_pos = 100.;
    for (entity, name, _skills, portait_id) in characters.iter() {
        debug!("Setup character portrait {:?}::{:?} map", name, portait_id);
        commands.entity(entity).insert((
            SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(x_pos, -400., 3.),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(portait_id.index),
                texture_atlas: battle_asset.portrait_atlas.clone(),
                ..default()
            },
            OnBattleScreen,
        ));
        x_pos += 100.;
    }

    for (entity, name, _portait_id) in director_characters.iter() {
        debug!("Adding director character to {:?}::{:?} map", entity, name);
        commands.entity(entity).insert(InZone::new("central"));
    }
    for (entity, _name, portait_id) in director_characters.iter() {
        debug!(
            "Adding director character portrait {:?}::{:?} map",
            entity, portait_id
        );
        commands.entity(entity).insert((
            SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(x_pos, -400., 3.),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(portait_id.index),
                texture_atlas: battle_asset.portrait_atlas.clone(),
                ..default()
            },
            OnBattleScreen,
        ));
        x_pos += 100.;
    }

    game_state.set(GameState::BattleTurns);
}

fn enable_buttons(
    mut commands: Commands,
    move_button: Query<Entity, With<MoveButton>>,
    extra_move_button: Query<Entity, With<ExtraMoveButton>>,
    attack_button: Query<Entity, With<AttackButton>>,
    switch_button: Query<Entity, With<SwitchButton>>,
    get_up_button: Query<Entity, With<GetUpButton>>,
    end_turn_button: Query<Entity, With<EndTurnButton>>,
) {
    let Ok(move_button_entity) = move_button.get_single() else {
        return;
    };
    let Ok(extra_move_button_entity) = extra_move_button.get_single() else {
        return;
    };
    let Ok(attack_button_entity) = attack_button.get_single() else {
        return;
    };
    let Ok(switch_button_entity) = switch_button.get_single() else {
        return;
    };
    let Ok(get_up_button_entity) = get_up_button.get_single() else {
        return;
    };
    let Ok(end_turn_button_entity) = end_turn_button.get_single() else {
        return;
    };
    commands.entity(move_button_entity).insert(ButtonEnabled);
    commands
        .entity(extra_move_button_entity)
        .remove::<ButtonEnabled>();
    commands
        .entity(attack_button_entity)
        .remove::<ButtonEnabled>();
    commands
        .entity(switch_button_entity)
        .remove::<ButtonEnabled>();
    commands
        .entity(get_up_button_entity)
        .remove::<ButtonEnabled>();
    commands
        .entity(end_turn_button_entity)
        .insert(ButtonEnabled);
}

const DISABLED_BUTTON: Color = Color::rgb(0.05, 0.05, 0.05);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut commands: Commands,
    mut pressed_query: Query<(Entity), (With<ButtonEnabled>, With<ButtonPressed>)>,
    mut interaction_query: Query<
        (Entity, &Interaction),
        (
            Changed<Interaction>,
            (With<ButtonEnabled>, Without<ButtonPressed>),
        ),
    >,
) {
    for (entity, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if pressed_query.is_empty() {
                    commands.entity(entity).insert(ButtonPressed);
                    commands.entity(entity).remove::<ButtonHoover>();
                }
            }
            Interaction::Hovered => {
                if !(pressed_query.contains(entity)) {
                    commands.entity(entity).insert(ButtonHoover);
                }
            }
            Interaction::None => {
                commands.entity(entity).remove::<ButtonHoover>();
            }
        }
    }
}

fn show_button_state(
    mut pressed_buttons: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (
            With<Button>,
            With<ButtonEnabled>,
            With<ButtonPressed>,
            Without<ButtonHoover>,
        ),
    >,
    mut hoover_buttons: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (
            With<Button>,
            With<ButtonEnabled>,
            Without<ButtonPressed>,
            With<ButtonHoover>,
        ),
    >,
    mut enabled_buttons: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (
            With<Button>,
            With<ButtonEnabled>,
            Without<ButtonPressed>,
            Without<ButtonHoover>,
        ),
    >,
    mut disabled_buttons: Query<
        (&mut BackgroundColor, &mut BorderColor),
        (With<Button>, Without<ButtonEnabled>),
    >,
) {
    for (mut color, mut border_color) in pressed_buttons.iter_mut() {
        *color = PRESSED_BUTTON.into();
        border_color.0 = Color::RED;
    }
    for (mut color, mut border_color) in hoover_buttons.iter_mut() {
        *color = HOVERED_BUTTON.into();
        border_color.0 = Color::WHITE;
    }
    for (mut color, mut border_color) in enabled_buttons.iter_mut() {
        *color = NORMAL_BUTTON.into();
        border_color.0 = Color::BLACK;
    }
    for (mut color, mut border_color) in disabled_buttons.iter_mut() {
        *color = DISABLED_BUTTON.into();
        border_color.0 = Color::BLUE;
    }
}

fn action_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            border_color: BorderColor::from(Color::rgba(0.0, 0.0, 1.0, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(20.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::FlexStart,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: BorderColor::from(Color::rgba(0.5, 0.0, 0.0, 0.5)),
                    ..default()
                })
                .with_children(|parent| {
                    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

                    add_button(parent, MoveButton, "Move", font_handle.clone());
                    add_button(parent, ExtraMoveButton, "Extra Move", font_handle.clone());
                    add_button(parent, AttackButton, "Attack", font_handle.clone());
                    add_button(parent, SwitchButton, "Switch Weapon", font_handle.clone());
                    add_button(parent, GetUpButton, "Get Up", font_handle.clone());
                    add_button(parent, EndTurnButton, "End Turn", font_handle.clone());
                });
        });
}

fn add_button(
    parent: &mut ChildBuilder,
    button_tag: impl Component,
    button_text: &str,
    font_handle: Handle<Font>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                border_color: BorderColor::from(Color::BLACK),
                background_color: BackgroundColor::from(NORMAL_BUTTON),
                ..default()
            },
            button_tag,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                button_text,
                TextStyle {
                    font: font_handle,
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn add_zone(
    commands: &mut Commands,
    x_pos: f32,
    y_pos: f32,
    height: f32,
    width: f32,
    tag: &str,
    zone_name: &str,
) {
    commands.spawn(Zone {
        position: ZoneArea {
            center: Vec3::new(x_pos, y_pos, 1.),
            size: Vec2::new(width, height),
        },
        name: ZoneName::new(tag, zone_name),
    });
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };
    let Ok(window) = q_window.get_single() else {
        return;
    };

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        // debug!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

fn show_initiative(
    mut query: Query<&mut TextureAtlasSprite, With<InitiativeSprite>>,
    characters: Query<&PortraitAtlasId, With<CurrentInitiative>>,
) {
    let Ok(mut sprite_handle) = query.get_single_mut() else {
        return;
    };
    let Ok(current_portrait) = characters.get_single() else {
        return;
    };
    sprite_handle.index = current_portrait.index;
}

fn render_zones(mut commands: Commands, mut zones: Query<(&ZoneArea, &ZoneName)>) {
    for (area, name) in zones.iter() {
        debug!("render zone: {:?} {:?}", name, area);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.941, 0.0, 1.0, 0.5),
                    custom_size: Some(area.size.clone()),
                    ..default()
                },
                transform: Transform::from_translation(area.center.clone()),
                ..default()
            },
            OnBattleScreen,
        ));
    }
}

fn resolve_zones(
    mut located_objects: Query<(Entity, &mut InZone)>,
    zones: Query<(Entity, &ZoneArea, &ZoneName)>,
) {
    for (entity, mut zone) in located_objects.iter_mut() {
        for (zone_entity, area, name) in zones.iter() {
            if zone.name == name.tag {
                debug!(
                    "placing {:?}::{:?} in {:?}::{:?}::{:?}",
                    entity, zone, zone_entity, area, name
                );
                zone.area = Some(zone_entity);
            }
        }
    }
}

fn set_starting_initiative(mut commands: Commands, mut characters: Query<(Entity, &Initiative)>) {
    let mut start_player: Option<Entity> = None;
    let mut highest_initiative = 0;
    for (entity, initiative) in characters.iter() {
        debug!("{:?}::{:?} > {:?}", entity, initiative, highest_initiative);
        if initiative.value > highest_initiative {
            start_player = Some(entity);
            highest_initiative = initiative.value;
        }
    }
    if let Some(entity) = start_player {
        commands.entity(entity).insert(CurrentInitiative);
    }
}

fn draw_icons_in_zone(
    mut characters: Query<(Entity, &mut Transform, &InZone)>,
    zones: Query<(&ZoneArea)>,
) {
    let mut char_in_zone: HashMap<u32, Vec<Entity>> = HashMap::new();
    for (entity, mut transform, in_zone) in characters.iter_mut() {
        if let Some(in_area) = in_zone.area {
            if let Ok(area) = zones.get_component::<ZoneArea>(in_area) {
                let hash_key = in_area.index();
                char_in_zone
                    .entry(hash_key)
                    .or_insert_with(Vec::new)
                    .push(entity);
                let people_in_zone = char_in_zone[&hash_key].len() as f32;
                let off_set = -60. + people_in_zone * 20.;
                transform.translation = Vec3::new(
                    area.center.x + off_set,
                    area.center.y,
                    area.center.z + people_in_zone,
                );
            }
        }
    }
}