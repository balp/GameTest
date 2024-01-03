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

#[derive(Bundle, Debug)]
pub(crate) struct PlayerCharacter {
    pub(crate) name: CharacterName,
    pub(crate) portrait: PortraitAtlasId,
}

#[derive(Bundle, Debug)]
pub(crate) struct SceneActor {
    pub(crate) name: CharacterName,
    pub(crate) portrait: PortraitAtlasId,
}
