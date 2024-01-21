use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::asset_loader::BattleAsset;
use crate::characters::{CharacterName, CharacterSkills, Initiative, PortraitAtlasId};
use crate::MainCamera;
use crate::states::GameState;
use crate::utils::despawn_screen;

pub struct Battle;


impl Plugin for Battle {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyWorldCoords>()
            .add_systems(OnEnter(GameState::Battle), battle_setup)
            .add_systems(OnExit(GameState::Battle), (set_starting_initiative, render_zones))
            .add_systems(
                Update,
                (
                    show_initiative,
                    my_cursor_system,
                    bevy::window::close_on_esc,
                ).run_if(in_state(GameState::BattleTurns)),
            )
            .add_systems(Update, (bevy::window::close_on_esc).run_if(in_state(GameState::BattleEnded)))
            .add_systems(
                OnExit(GameState::BattleEnded),
                despawn_screen::<OnBattleScreen>,
            )
        ;
    }
}

#[derive(Component)]
struct OnBattleScreen;

#[derive(Component)]
struct InitiativeSprite;

#[derive(Component)]
struct CurrentInitiative;

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
        Self { tag: tag.to_string(), name: name.to_string() }
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
}

impl InZone {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), }
    }
}

fn battle_setup(
    mut commands: Commands,
    battle_asset: Res<BattleAsset>,
    mut windows: Query<&mut Window>,
    mut characters: Query<(Entity, &CharacterName, &CharacterSkills)>,
    mut game_state: ResMut<NextState<GameState>>
) {
    info!("battle_setup...");
    let mut window = windows.single_mut();
    window.resolution.set(2048.0, 1024.0);

    let texture = battle_asset.maps[1].clone();
    commands.spawn((SpriteBundle {
        texture,
        transform: Transform::from_translation(Vec3::new(1024., 0., 0.)),
        ..default()
    }, OnBattleScreen));



    commands.spawn((SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(1900.0, 400.0, 2.0),
            ..default()
        },
        sprite: TextureAtlasSprite::default(),
        texture_atlas: battle_asset.portrait_atlas.clone(),
        ..default()
    }, InitiativeSprite, OnBattleScreen));

    add_zone(&mut commands, 222., 142., 190., 100., "cell_a_11", "Cell A11");
    add_zone(&mut commands, 360., 142., 190., 100., "cell_a_12", "Cell A12");
    add_zone(&mut commands, 495., 142., 190., 100., "cell_a_13", "Cell A13");

    add_zone(&mut commands, 222., -142., 190., 100., "cell_a_21", "Cell A21");
    add_zone(&mut commands, 360., -142., 190., 100., "cell_a_22", "Cell A22");
    add_zone(&mut commands, 495., -142., 190., 100., "cell_a_23", "Cell A23");

    add_zone(&mut commands, 330., 0., 90., 470., "cellblock_a", "Cell block A");
    add_zone(&mut commands, 630., 0., 140., 120., "gate_a", "Gate A");
    add_zone(&mut commands, 990., 0., 640., 580., "central", "Central");
    add_zone(&mut commands, 1530., 0., 90., 470., "access_corridor", "Access Corridor");

    for (entity, name, skills) in characters.iter() {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=100);
        let tens = roll / 10u8;
        let once = roll % 10u8;
        let initiative: u8 = match skills.alertness.value {
            x if x <= roll => { if tens == once { tens + once + 10 } else { tens + once } }
            x if x > roll => { if tens == once { 0 } else { once } }
            _ => 0,
        };
        debug!("Rolled initiative {:?} for {:?}", initiative, name);
        let character_initiative = Initiative::new(initiative);
        commands.entity(entity).insert(character_initiative);

        commands.entity(entity).insert(InZone::new("cell_a_13"));
    }

    game_state.set(GameState::BattleTurns);
}

fn add_zone(commands: &mut Commands, x_pos: f32, y_pos: f32, height: f32, width: f32, tag: &str, zone_name: &str) {
    commands.spawn(Zone {
        position: ZoneArea {
            center: Vec3::new(x_pos, y_pos, 1.),
            size: Vec2::new(width, height)
        },
        name: ZoneName::new(tag, zone_name)
    });
}

fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return; };
    let Ok(window) = q_window.get_single() else { return; };

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        debug!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

fn show_initiative(
    mut query: Query<&mut TextureAtlasSprite, With<InitiativeSprite>>,
    characters: Query<(&CharacterName, &PortraitAtlasId), With<CurrentInitiative>>,
) {
    let Ok(mut sprite_handle) = query.get_single_mut() else { return; };
    let Ok((current_player, current_portrait)) = characters.get_single() else { return; };
    sprite_handle.index = current_portrait.index;
}

fn render_zones(
    mut commands: Commands,
    mut zones: Query<(&ZoneArea, &ZoneName)>,
) {
    for (area, name) in zones.iter() {
        debug!("render zone: {:?} {:?}", name, area);
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.941, 0.0, 1.0, 0.5),
                custom_size: Some(area.size.clone()),
                ..default()
            },
            transform: Transform::from_translation(area.center.clone()),
            ..default()
        }, OnBattleScreen));
    }
}

fn set_starting_initiative(
    mut commands: Commands,
    mut characters: Query<(Entity, &Initiative)>,
) {
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