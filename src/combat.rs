use std::vec::Vec;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::asset_loader::CombatAsset;
use crate::characters::{
    CharacterName, CharacterSkills, CharacterType, DirectorCharacter, IconName, Initiative, NoName,
    PortraitAtlasId, SaveCharacters, Vitality,
};
use crate::combat_map::CombatMap;
use crate::schedule::CombatUpdateSets;
use crate::states::GameState;
use crate::utils::despawn_screen;
use crate::MainCamera;

pub struct Combat;

impl Plugin for Combat {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
            .add_systems(OnEnter(GameState::Combat), combat_setup)
            .add_systems(
                OnExit(GameState::Combat),
                (set_starting_initiative, setup_zone_sprites, resolve_zones),
            )
            .add_systems(OnEnter(GameState::CombatTurns), action_menu)
            .add_systems(
                Update,
                (enable_buttons,)
                    .run_if(in_state(GameState::CombatTurns))
                    .in_set(CombatUpdateSets::TurnChanges),
            )
            .add_systems(
                Update,
                (my_cursor_system, button_interaction_system)
                    .run_if(in_state(GameState::CombatTurns))
                    .in_set(CombatUpdateSets::UserInput),
            )
            .add_systems(
                Update,
                (
                    show_initiative,
                    show_button_state,
                    draw_icons_in_zone,
                    render_zones,
                    bevy::window::close_on_esc,
                )
                    .run_if(in_state(GameState::CombatTurns))
                    .in_set(CombatUpdateSets::EntityUpdates),
            )
            .add_systems(
                Update,
                (draw_icons_in_zone, bevy::window::close_on_esc)
                    .run_if(in_state(GameState::CombatEnded)),
            )
            .add_systems(
                OnExit(GameState::CombatEnded),
                despawn_screen::<OnCombatScreen>,
            );
    }
}

#[derive(Component)]
struct OnCombatScreen;

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

#[derive(Component)]
struct CharacterZone;

#[derive(Component)]
struct AdjacentZone;

#[derive(Component)]
struct HooverZone;

#[derive(Resource, Default, Debug)]
struct MyWorldCoords(Vec2);

fn combat_setup(
    mut commands: Commands,
    combat_asset: Res<CombatAsset>,
    mut windows: Query<&mut Window>,
    characters: Query<(Entity, &CharacterName, &CharacterSkills, &PortraitAtlasId)>,
    director_characters: Query<(Entity, &NoName, &PortraitAtlasId)>,
    mut game_state: ResMut<NextState<GameState>>,
    saved_characters: Res<Assets<SaveCharacters>>,
    combat_maps: Res<Assets<CombatMap>>,
) {
    info!("combat_setup...");
    let mut window = windows.single_mut();
    window.resolution.set(2048.0, 1024.0);

    // Draw map
    if let Some(combat_map) = combat_maps.get(combat_asset.combat_map.clone()) {
        debug!("combat_map: {:?}", combat_map);
        if let Some(saved_chars) = saved_characters.get(combat_asset.characters.clone()) {
            setup_combat_map(&mut commands, combat_map, &combat_asset);

            for in_scene in combat_map.start_positions.iter() {
                if let Some(char_type) = saved_chars.get_char_for_tag(in_scene.entity_tag.clone()) {
                    match char_type {
                        CharacterType::PlayerCharacter { char } => {
                            if let Some((entity, portait_id)) =
                                character_entity_and_portrait_for_tag(
                                    &characters,
                                    &in_scene.entity_tag,
                                )
                            {
                                let initiative = char.initiative();
                                let character_initiative = Initiative::new(initiative);
                                commands.entity(entity).insert(character_initiative);
                                commands
                                    .entity(entity)
                                    .insert(InZone::new(in_scene.zone_tag.as_str()));
                                add_combat_token(
                                    &mut commands,
                                    &combat_asset,
                                    100.,
                                    entity,
                                    &portait_id,
                                );
                            }
                        }
                        CharacterType::DirectorCharacter { char } => {
                            if let Some(portait_id) = director_portrait_for_tag(
                                &director_characters,
                                &in_scene.entity_tag,
                            ) {
                                commands.spawn((
                                    NoName {
                                        slug: in_scene.entity_tag.clone(),
                                        alias: in_scene.entity_tag.clone(),
                                        generic: in_scene.entity_tag.clone(),
                                    },
                                    Initiative {
                                        value: char.initiative,
                                    },
                                    Vitality {
                                        value: char.vitality,
                                    },
                                    SpriteSheetBundle {
                                        transform: Transform {
                                            translation: Vec3::new(100., -400., 3.),
                                            ..default()
                                        },
                                        atlas: TextureAtlas {
                                            layout: combat_asset.portrait_atlas.clone(),
                                            index: portait_id.index,
                                        },
                                        texture: combat_asset.portrait_image.clone(),
                                        ..default()
                                    },
                                    InZone::new(in_scene.zone_tag.as_str()),
                                    OnCombatScreen,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(1900.0, 400.0, 2.0),
                ..default()
            },
            atlas: TextureAtlas {
                layout: combat_asset.portrait_atlas.clone(),
                index: 0,
            },
            texture: combat_asset.portrait_image.clone(),
            ..default()
        },
        InitiativeSprite,
        OnCombatScreen,
    ));

    game_state.set(GameState::CombatTurns);
}

fn character_entity_and_portrait_for_tag(
    characters: &Query<(Entity, &CharacterName, &CharacterSkills, &PortraitAtlasId)>,
    tag: &String,
) -> Option<(Entity, PortraitAtlasId)> {
    for (entity, name, _skills, portait_id) in characters.iter() {
        if &(name.slug) == tag {
            return Some((entity.clone(), portait_id.clone()));
        }
    }
    None
}

fn director_portrait_for_tag(
    dcs: &Query<(Entity, &NoName, &PortraitAtlasId)>,
    tag: &String,
) -> Option<PortraitAtlasId> {
    for (_entity, name, portait_id) in dcs.iter() {
        if &(name.slug) == tag {
            return Some(portait_id.clone());
        }
    }
    None
}

fn setup_combat_map(
    commands: &mut Commands,
    combat_map: &CombatMap,
    combat_asset: &Res<CombatAsset>,
) {
    let texture = combat_asset.maps[&combat_map.bitmap.clone()].clone();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(1024., 0., 0.)),
            ..default()
        },
        OnCombatScreen,
    ));

    for zone in combat_map.zones.iter() {
        add_zone(
            commands,
            zone.position.x_pos,
            zone.position.y_pos,
            zone.position.height,
            zone.position.width,
            zone.tag.as_str(),
            zone.name.as_str(),
        );
    }

    for start_pos in combat_map.start_positions.iter() {
        debug!("adding: {:?}", start_pos);
    }
}

fn add_combat_token(
    commands: &mut Commands,
    combat_asset: &Res<CombatAsset>,
    x_pos: f32,
    entity: Entity,
    portait_id: &PortraitAtlasId,
) {
    commands.entity(entity).insert((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(x_pos, -400., 3.),
                ..default()
            },
            atlas: TextureAtlas {
                layout: combat_asset.portrait_atlas.clone(),
                index: portait_id.index,
            },
            texture: combat_asset.portrait_image.clone(),
            ..default()
        },
        OnCombatScreen,
    ));
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

fn button_interaction_system(
    mut commands: Commands,
    pressed_query: Query<Entity, (With<ButtonEnabled>, With<ButtonPressed>)>,
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
                if !pressed_query.contains(entity) {
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
    mut commands: Commands,
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    buttons: Res<ButtonInput<MouseButton>>,
    zones: Query<(Entity, &ZoneArea)>,
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
        if buttons.just_pressed(MouseButton::Left) {
            debug!("Button just pressed@ {:?}", mycoords);
        }
        for (entity, zone) in zones.iter() {
            if zone.in_bounds(mycoords.0) {
                commands.entity(entity).insert(HooverZone);
            } else {
                commands.entity(entity).remove::<HooverZone>();
            }
        }
    }
}

fn show_initiative(
    mut query: Query<&mut TextureAtlas, With<InitiativeSprite>>,
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

fn setup_zone_sprites(mut commands: Commands, zones: Query<(Entity, &ZoneArea, &ZoneName)>) {
    for (entity, area, name) in zones.iter() {
        debug!("setup_zone_sprites: {:?} {:?}", name, area);
        commands.entity(entity).insert((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.941, 0.0, 1.0, 0.5),
                    custom_size: Some(area.size),
                    ..default()
                },
                transform: Transform::from_translation(area.center),
                ..default()
            },
            OnCombatScreen,
        ));
    }
}

fn render_zones(
    mut hoover_zones: Query<&mut Sprite, (With<ZoneArea>, With<HooverZone>)>,
    mut other_zones: Query<&mut Sprite, (With<ZoneArea>, Without<HooverZone>)>,
) {
    for mut sprite in hoover_zones.iter_mut() {
        sprite.color = Color::rgba(0.941, 0., 1., 0.5);
    }
    for mut sprite in other_zones.iter_mut() {
        sprite.color = Color::rgba(0., 0., 1., 0.);
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

fn set_starting_initiative(mut commands: Commands, characters: Query<(Entity, &Initiative)>) {
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
    zones: Query<&ZoneArea>,
) {
    let mut char_in_zone: HashMap<u32, Vec<Entity>> = HashMap::new();
    for (entity, mut transform, in_zone) in characters.iter_mut() {
        if let Some(in_area) = in_zone.area {
            if let Ok(area) = zones.get(in_area) {
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

#[derive(Component, Debug)]
pub struct ZoneArea {
    pub center: Vec3,
    pub size: Vec2,
}

impl ZoneArea {
    pub fn in_bounds(&self, pos: Vec2) -> bool {
        let x_min = self.center.x - self.size.x / 2.;
        let x_max = self.center.x + self.size.x / 2.;
        let y_min = self.center.y - self.size.y / 2.;
        let y_max = self.center.y + self.size.y / 2.;

        pos.x >= x_min && pos.y >= y_min && pos.x <= x_max && pos.y <= y_max
    }
}

#[derive(Component, Debug)]
pub struct ZoneName {
    pub tag: String,
    pub name: String,
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
    pub position: ZoneArea,
    pub name: ZoneName,
}

#[derive(Component, Debug)]
pub struct InZone {
    pub name: String,
    pub area: Option<Entity>,
}

impl InZone {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            area: None,
        }
    }
}
