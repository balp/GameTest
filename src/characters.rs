use bevy::prelude::*;
use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Component, Debug, Default, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerName {
    pub first: String,
    pub last: String,
    pub alias: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SkillType {
    Agility(u8),
    Alertness(u8),
    Charm(u8),
    Contacts(u8),
    Credit(u8),
    Electronics(u8),
    Endurance(u8),
    Engineering(u8),
    Entertainment(u8),
    Humanities(u8),
    Investigation(u8),
    Languages(u8),
    Machinery(u8),
    Medicine(u8),
    Melee(u8),
    Prestidigitation(u8),
    RangedCombat(u8),
    RedTape(u8),
    Science(u8),
    Search(u8),
    Security(u8),
    Sneak(u8),
    Status(u8),
    Strength(u8),
    Subterfuge(u8),
    Survival(u8),
    Vehicles(u8),
    Willpower(u8),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LanguageLevel {
    Native,
    Fluent,
    Learning4,
    Learning3,
    Learning2,
    Learning1,
    Learning0,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LanguageType {
    name: String,
    level: LanguageLevel,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PlotHook {
    MediaDarling,
    LookingForACase,
    LookingForAdventure,
    LookingForThePast,
    SecretService,
    ArchEnemy,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GearKit {
    kit_type: GearKitType,
    signature: bool,
    scene: bool,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize, Clone)]
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
        for skill in self.skills.iter() {
            match skill {
                SkillType::Agility(value) => {
                    return value.clone();
                }
                _ => {}
            }
        }
        15
    }
    pub fn get_alertness(&self) -> u8 {
        for skill in self.skills.iter() {
            match skill {
                SkillType::Alertness(value) => {
                    return value.clone();
                }
                _ => {}
            }
        }
        15
    }
    pub fn get_sneak(&self) -> u8 {
        for skill in self.skills.iter() {
            match skill {
                SkillType::Sneak(value) => {
                    return value.clone();
                }
                _ => {}
            }
        }
        15
    }

    pub fn initiative(&self) -> u8 {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=100);
        let tens = roll / 10u8;
        let once = roll % 10u8;
        match self.get_alertness() {
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
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DCTag {
    Mook,
    Lieutenant,
    Flips(u8),
    MultipleAttacks(u8),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AttackTag {
    ShortRange,
    Reload(u8),
    Paralytic,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attack {
    name: String,
    skill: u8,
    damage: u8,
    tags: Vec<AttackTag>,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Clone)]
pub enum CharacterType {
    PlayerCharacter { char: SavePlayerCharacter },
    DirectorCharacter { char: SaveDirectorCharacter },
}

impl SaveCharacters {
    pub fn get_char_for_tag(&self, tag: String) -> Option<CharacterType> {
        for pc in self.player_characters.iter() {
            if pc.tag == tag {
                return Some(CharacterType::PlayerCharacter { char: pc.clone() });
            }
        }
        for dc in self.director_characters.iter() {
            if dc.tag == tag {
                return Some(CharacterType::DirectorCharacter { char: dc.clone() });
            }
        }
        None
    }
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
