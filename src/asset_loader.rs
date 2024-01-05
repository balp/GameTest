use bevy::{asset::Handle, asset::LoadedFolder, prelude::*};
use bevy_talks::prelude::*;

use crate::characters::{CharacterName, PlayerCharacter, PortraitAtlasId, SceneActor};
use crate::states::GameState;

#[derive(Resource)]
pub struct PreloadAssets {
    pub(crate) intro_dialog: Handle<TalkData>,
    pub(crate) fiction_font: Handle<Font>,
}

#[derive(Resource)]
pub struct SimpleTalkAsset {
    pub(crate) intro_dialog: Handle<TalkData>,
    pub(crate) portrait_atlas: Handle<TextureAtlas>,
    pub(crate) fiction_font: Handle<Font>,
}

const DIALOG_FILE: &str = "dialog/the_cell.talk.ron";

pub struct AssetLoader;

impl Plugin for AssetLoader {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), show_splash_screen)
            .add_systems(
                OnEnter(GameState::AssetsLoading),
                (add_characters, load_assets),
            )
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

fn add_characters(mut commands: Commands) {
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("elektra", "Electra", "Elektra", "Ambrosia"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("yurika", "Yurika", "Yurika", "Mishida"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("paul", "Paul", "Paul", "Marchand"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("harry", "Harry", "Harold", "Fitzroy"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("frida", "Frida", "Anni-Frid", "Bäckströn"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(PlayerCharacter {
        name: CharacterName::new("eloise", "Éloïse", "Éloïse", "Giraud"),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(SceneActor {
        name: CharacterName::new("narrator", "Narrator", "", ""),
        portrait: PortraitAtlasId::default(),
    });
    commands.spawn(SceneActor {
        name: CharacterName::new("empty", "", "", ""),
        portrait: PortraitAtlasId::default(),
    });
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PortraitIconsFolder(
        asset_server.load_folder("portraits/dialog"),
    ));
    let intro_talk = asset_server.load(DIALOG_FILE);
    commands.insert_resource(PreloadAssets {
        intro_dialog: intro_talk,
        fiction_font: asset_server.load("fonts/gnuolane-free.rg-regular.otf"),
    });
}

fn check_assets_loaded(
    server: Res<AssetServer>,
    preloaded_assets: Res<PreloadAssets>,
    portrait_icons_folder: Res<PortraitIconsFolder>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if server.is_loaded_with_dependencies(preloaded_assets.intro_dialog.clone())
        && server.is_loaded_with_dependencies(preloaded_assets.fiction_font.clone())
        && server.is_loaded_with_dependencies(&portrait_icons_folder.0)
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
    preloaded_assets: Res<PreloadAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    mut characters: Query<(&CharacterName, &mut PortraitAtlasId)>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let loaded_folder = loaded_folders.get(&portrait_icons_folder.0).unwrap();
    for (index, handle) in loaded_folder.handles.iter().enumerate() {
        let id = handle.id().typed_unchecked::<Image>();
        let path = handle.path().clone();
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
                for (name, mut atlasid) in characters.iter_mut() {
                    debug!("{:?} == {:?}: {:?}", stem, name, atlasid);
                    if stem == name.slug.as_str() {
                        debug!("set atlasid index to {index}");
                        atlasid.index = index;
                    }
                }
            }
        }
        texture_atlas_builder.add_texture(id, texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let portrait_atlas = texture_atlases.add(texture_atlas);
    let asset = SimpleTalkAsset {
        intro_dialog: preloaded_assets.intro_dialog.clone(),
        portrait_atlas,
        fiction_font: preloaded_assets.fiction_font.clone(),
    };
    commands.insert_resource(asset);
}

fn to_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InteractiveFiction);
}
