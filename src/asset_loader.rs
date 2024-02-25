use crate::combat_map::{CombatMap, CombatMapAssetLoader};

use bevy::utils::HashMap;
use bevy::{asset::Handle, asset::LoadedFolder, prelude::*};

use crate::characters::{
    CharacterName, CharacterSkills, CharactersAssetLoader, DirectorCharacter, IconName, Initiative,
    NoName, PlayerCharacter, PortraitAtlasId, SaveCharacters, Vitality,
};
use crate::states::GameState;

#[derive(Resource)]
pub struct PreloadAssets {
    pub(crate) fiction_font: Handle<Font>,
    pub combat_map: Handle<CombatMap>,
    pub characters: Handle<SaveCharacters>,
}

#[derive(Resource)]
pub struct CombatAsset {
    pub portrait_atlas: Handle<TextureAtlasLayout>,
    pub portrait_image: Handle<Image>,
    pub maps: HashMap<String, Handle<Image>>,
    pub combat_map: Handle<CombatMap>,
    pub characters: Handle<SaveCharacters>,
}

const DIALOG_FILE: &str = "dialog/the_cell.talk.ron";

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.init_asset::<CombatMap>()
            .init_asset::<SaveCharacters>()
            .init_asset_loader::<CombatMapAssetLoader>()
            .init_asset_loader::<CharactersAssetLoader>()
            .add_systems(OnEnter(GameState::Splash), show_splash_screen)
            .add_systems(OnEnter(GameState::AssetsLoading), load_assets)
            .add_systems(
                Update,
                check_assets_loaded.run_if(in_state(GameState::AssetsLoading)),
            )
            .add_systems(OnExit(GameState::AssetsLoading), setup_assets)
            .add_systems(OnEnter(GameState::AssetsSetup), to_game)
            .add_systems(
                OnExit(GameState::AssetsSetup),
                crate::utils::despawn_screen::<OnSplashScreen>,
            );
    }
}

#[derive(Resource, Default)]
struct PortraitIconsFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
struct MapsFolder(Handle<LoadedFolder>);

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn show_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let logo = asset_server.load("branding/icon.png");
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
                image: UiImage::new(logo),
                ..default()
            });
        });
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
    game_state.set(GameState::AssetsLoading);
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PortraitIconsFolder(asset_server.load_folder("portraits")));
    commands.insert_resource(MapsFolder(asset_server.load_folder("maps/bitmaps")));
    commands.insert_resource(PreloadAssets {
        fiction_font: asset_server.load("fonts/gnuolane-free.rg-regular.otf"),
        combat_map: asset_server.load("maps/cell_blocks.map"),
        characters: asset_server.load("characters.characters"),
    });
}

fn check_assets_loaded(
    server: Res<AssetServer>,
    preloaded_assets: Res<PreloadAssets>,
    portrait_icons_folder: Res<PortraitIconsFolder>,
    maps_folder: Res<MapsFolder>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if server.is_loaded_with_dependencies(preloaded_assets.fiction_font.clone())
        && server.is_loaded_with_dependencies(preloaded_assets.combat_map.clone())
        && server.is_loaded_with_dependencies(preloaded_assets.characters.clone())
        && server.is_loaded_with_dependencies(&portrait_icons_folder.0)
        && server.is_loaded_with_dependencies(&maps_folder.0)
    {
        game_state.set(GameState::AssetsSetup);
    } else if timer.tick(time.delta()).finished() {
        game_state.set(GameState::AssetsFailed);
    }
}

fn setup_assets(
    mut commands: Commands,
    loaded_folders: Res<Assets<LoadedFolder>>,
    portrait_icons_folder: Res<PortraitIconsFolder>,
    maps_folder: Res<MapsFolder>,
    preloaded_assets: Res<PreloadAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<Assets<Image>>,
    save_chars: Res<Assets<SaveCharacters>>,
    mut characters: Query<(&IconName, &mut PortraitAtlasId)>,
) {
    let mut portrait_indexes = HashMap::new();

    let mut portrait_texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_portrait_folder = loaded_folders.get(&portrait_icons_folder.0).unwrap();
    for (index, handle) in loaded_portrait_folder.handles.iter().enumerate() {
        let id = handle.id().typed_unchecked::<Image>();
        let path = handle.path();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };
        debug!("Loaded texture: {index:?} - {path:?} - {id:?}");
        if let Some(asset_path) = path {
            if let Some(stem) = asset_path.path().file_stem() {
                if let Some(base_name) = stem.to_str() {
                    portrait_indexes.insert(base_name, index);
                }

                debug!("save: {:?} == {:?}", stem, index);
                for (name, mut atlasid) in characters.iter_mut() {
                    debug!("{:?} == {:?}: {:?}", stem, name, atlasid);
                    if stem == name.slug.as_str() {
                        debug!("set atlasid index to {index}");
                        atlasid.index = index;
                    }
                }
            }
        }
        portrait_texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (portrait_texture_atlas, portrait_raw_image) =
        portrait_texture_atlas_builder.finish().unwrap();
    let portrait_atlas = texture_atlases.add(portrait_texture_atlas);
    let portrait_image = textures.add(portrait_raw_image);

    let mut maps = HashMap::new();
    let loaded_map_folder = loaded_folders.get(&maps_folder.0).unwrap();
    for handle in loaded_map_folder.handles.iter() {
        if let Some(path) = handle.path() {
            if let Some(path_stem) = path.path().file_stem() {
                if let Some(stem) = path_stem.to_str() {
                    let texture_handle = handle.clone();
                    let typed_handle = texture_handle.typed_unchecked::<Image>();
                    debug!("adding maps:: {:?}->{:?}", stem.to_string(), typed_handle);
                    maps.insert(stem.to_string(), typed_handle);
                }
            }
        }
    }

    if let Some(e) = save_chars.get(preloaded_assets.characters.id()) {
        debug!("Setup player characters");
        for (i, player_char) in e.player_characters.iter().enumerate() {
            debug!("pc got: {:?} -> {:?}", i, player_char);
            if let Some(index) = portrait_indexes.get(player_char.tag.as_str()) {
                debug!("index: {:?} -> {:?}", i, index);
                commands.spawn(PlayerCharacter {
                    name: CharacterName {
                        slug: player_char.tag.clone(),
                        alias: player_char.name.alias.clone(),
                        first: player_char.name.first.clone(),
                        last: player_char.name.last.clone(),
                    },
                    icon: IconName {
                        slug: player_char.tag.clone(),
                    },
                    portrait: PortraitAtlasId {
                        index: *index,
                    },
                    skills: CharacterSkills::new(
                        player_char.get_agility(),
                        player_char.get_alertness(),
                        player_char.get_sneak(),
                    ),
                    vitality: Vitality {
                        value: player_char.vitality,
                    },
                });
            }
        }
        debug!("Setup director characters");
        for (i, directory_char) in e.director_characters.iter().enumerate() {
            debug!("dc got: {:?} -> {:?}", i, directory_char);
            if let Some(index) = portrait_indexes.get(directory_char.tag.as_str()) {
                debug!("index: {:?} -> {:?}", i, index);
                commands.spawn(DirectorCharacter {
                    name: NoName {
                        slug: directory_char.tag.clone(),
                        alias: directory_char.tag.clone(),
                        generic: directory_char.tag.clone(),
                    },
                    icon: IconName {
                        slug: directory_char.tag.clone(),
                    },
                    portrait: PortraitAtlasId {
                        index: *index,
                    },
                    initiative: Initiative {
                        value: directory_char.initiative,
                    },
                    vitality: Vitality {
                        value: directory_char.vitality,
                    },
                });
            }
        }
    }

    let combat_asset = CombatAsset {
        portrait_atlas,
        portrait_image,
        maps,
        combat_map: preloaded_assets.combat_map.clone(),
        characters: preloaded_assets.characters.clone(),
    };
    commands.insert_resource(combat_asset);
}

fn to_game(mut game_state: ResMut<NextState<GameState>>) {
    info!("to_game()");
    game_state.set(GameState::Combat);
}
