use bevy::prelude::*;
use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Component, Debug, Default)]
pub struct PortraitAtlasId {
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct IconName {
    pub slug: String,
}

#[derive(Component, Debug)]
pub struct CharacterName {
    pub slug: String,
    pub alias: String,
    pub first: String,
    pub last: String,
}

#[derive(Debug)]
pub struct Skill {
    pub value: u8,
}

impl Default for Skill {
    fn default() -> Self {
        Self { value: 15 }
    }
}

impl Skill {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug)]
pub struct CharacterSkills {
    pub agility: Skill,
    pub alertness: Skill,
    pub sneak: Skill,
}

impl CharacterSkills {
    pub fn new(agility: u8, alertness: u8, sneak: u8) -> Self {
        Self {
            agility: Skill::new(agility),
            alertness: Skill::new(alertness),
            sneak: Skill::new(sneak),
        }
    }
}

#[derive(Component, Debug)]
pub struct Vitality {
    pub value: u8,
}

#[derive(Component, Debug)]
pub struct Initiative {
    pub value: u8,
}

impl Initiative {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Bundle, Debug)]
pub struct PlayerCharacter {
    pub name: CharacterName,
    pub icon: IconName,
    pub portrait: PortraitAtlasId,
    pub skills: CharacterSkills,
    pub vitality: Vitality,
}

#[derive(Bundle, Debug)]
pub struct SceneActor {
    pub name: CharacterName,
    pub portrait: PortraitAtlasId,
}

#[derive(Component, Debug)]
pub struct NoName {
    pub slug: String,
    pub alias: String,
    pub generic: String,
}

#[derive(Bundle, Debug)]
pub struct DirectorCharacter {
    pub name: NoName,
    pub icon: IconName,
    pub portrait: PortraitAtlasId,
    pub initiative: Initiative,
    pub vitality: Vitality,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerName {
    pub first: String,
    pub last: String,
    pub alias: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SkillType {
    Agility { value: u8 },
    Alertness { value: u8 },
    Charm { value: u8 },
    Contacts { value: u8 },
    Credit { value: u8 },
    Electronics { value: u8 },
    Endurance { value: u8 },
    Engineering { value: u8 },
    Entertainment { value: u8 },
    Humanities { value: u8 },
    Investigation { value: u8 },
    Languages { value: u8 },
    Machinery { value: u8 },
    Medicine { value: u8 },
    Melee { value: u8 },
    Prestidigitation { value: u8 },
    RangedCombat { value: u8 },
    RedTape { value: u8 },
    Science { value: u8 },
    Search { value: u8 },
    Security { value: u8 },
    Sneak { value: u8 },
    Status { value: u8 },
    Strength { value: u8 },
    Subterfuge { value: u8 },
    Survival { value: u8 },
    Vehicles { value: u8 },
    Willpower { value: u8 },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AbilityType {
    BornBehindTheWheel,
    BeenEverywhere,
    PressCredentials,
    JudoBlackBelt,
    LockPicker,
    SixthSence,
    Peerage,
    Bushman,
    Pilot,
    MilitaryRank,
    FighterPilot,
    TechWiz,
    Young,
    MadInventor,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ComplicationType {
    Overconfident,
    CodeOfHonour,
    BadReputation,
    Honest,
    Drunkard,
    Patriot,
    Underage,
    Sleepy,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LanguageLevel {
    Native,
    Fluent,
    Learning4,
    Learning3,
    Learning2,
    Learning1,
    Learning0,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageType {
    name: String,
    level: LanguageLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PlotHook {
    MediaDarling,
    LookingForACase,
    LookingForAdventure,
    LookingForThePast,
    SecretService,
    ArchEnemy,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GearKitType {
    BeachWear,
    Bicycle,
    Binoculars,
    Camera,
    CampingGear,
    ChemistryLabSet,
    CompactCar,
    DisguiseKit,
    ElectronicsToolbox,
    FilmCamera,
    FlashLight,
    Furisode,
    Handgun,
    HikingGear,
    HuntingRifle,
    LockPicks,
    MechanicsToolbox,
    OffRoadVecicle,
    ParadeUniform,
    PocketHandgun,
    RacingCar,
    RadioSet,
    Scooter,
    SkiGear,
    SportsCar,
    SurvivalGear,
    TapeRecorder,
    WadOfCash,
    WalkieTalkie,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GearKit {
    kit_type: GearKitType,
    signature: bool,
    scene: bool,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct SavePlayerCharacter {
    pub tag: String,
    pub name: PlayerName,
    pub profession: String,
    pub skills: Vec<SkillType>,
    pub vitality: u8,
    pub abilities: Vec<AbilityType>,
    pub complications: Vec<ComplicationType>,
    pub languages: Vec<LanguageType>,
    pub plot_hooks: Vec<PlotHook>,
    pub gear_kits: Vec<GearKit>,
}

impl SavePlayerCharacter {
    pub fn get_agility(&self) -> u8 {
        15
    }
    pub fn get_alertness(&self) -> u8 {
        15
    }
    pub fn get_sneak(&self) -> u8 {
        15
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DCTag {
    Mook,
    Lieutenant,
    Flips { value: u8 },
    MultipleAttacks { value: u8 },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AttackTag {
    ShortRange,
    Reload { value: u8 },
    Paralytic,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attack {
    name: String,
    skill: u8,
    damage: u8,
    tags: Vec<AttackTag>,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct SaveDirectorCharacter {
    pub tag: String,
    pub tags: Vec<DCTag>,
    pub initiative: u8,
    pub vitality: u8,
    pub attacks: Vec<Attack>,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct SaveCharacters {
    pub player_characters: Vec<SavePlayerCharacter>,
    pub director_characters: Vec<SaveDirectorCharacter>,
}

#[derive(Default)]
pub struct CharactersAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CharactersAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CharactersAssetLoader {
    type Asset = SaveCharacters;
    type Settings = ();
    type Error = CharactersAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<SaveCharacters>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["characters"]
    }
}
