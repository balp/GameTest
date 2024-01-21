use bevy::prelude::*;

#[derive(Component, Debug)]
pub(crate) struct PortraitAtlasId {
    pub(crate) index: usize,
}

impl Default for PortraitAtlasId {
    fn default() -> Self {
        Self { index: 0 }
    }
}

#[derive(Component, Debug)]
pub(crate) struct CharacterName {
    pub(crate) slug: String,
    pub(crate) alias: String,
    pub(crate) first: String,
    pub(crate) last: String,
}

impl CharacterName {
    pub(crate) fn new(slug: &str, alias: &str, first: &str, last: &str) -> CharacterName {
        CharacterName {
            slug: slug.to_string(),
            alias: alias.to_string(),
            first: first.to_string(),
            last: last.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Skill {
    pub(crate) value: u8,
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
pub(crate) struct CharacterSkills {
    pub(crate) agility: Skill,
    pub(crate) alertness: Skill,
    pub(crate) sneak: Skill,
}

impl CharacterSkills {
    pub fn new(agility: u8, alertness: u8, sneak: u8) -> Self {
        Self { agility: Skill::new(agility), alertness: Skill::new(alertness), sneak: Skill::new(sneak) }
    }
}

#[derive(Component, Debug)]
pub struct Vitality {
    pub value: u8,
}

impl Vitality {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
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
    pub portrait: PortraitAtlasId,
    pub skills: CharacterSkills,
    pub vitality: Vitality,
}

#[derive(Bundle, Debug)]
pub(crate) struct SceneActor {
    pub(crate) name: CharacterName,
    pub(crate) portrait: PortraitAtlasId,
}

#[derive(Component, Debug)]
pub(crate) struct NoName {
    pub(crate) slug: String,
    pub(crate) alias: String,
    pub(crate) generic: String,
}

impl NoName {
    pub fn new(slug: &str, alias: &str, generic: &str) -> Self {
        Self {
            slug: slug.to_string(),
            alias: alias.to_string(),
            generic: generic.to_string(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct DirectorCharacter {
    pub name: NoName,
    pub portrait: PortraitAtlasId,
    pub initiative: Initiative,
    pub vitality: Vitality,
}
