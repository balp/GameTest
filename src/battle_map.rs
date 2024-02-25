use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ZoneMove {
    pub check: Option<String>,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapPosition {
    x_pos: f32,
    y_pos: f32,
    height: f32,
    width: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapZone {
    pub position: MapPosition,
    pub name: String,
    pub tag: String,
    pub adjacent: Vec<ZoneMove>,
}

#[derive(Asset, TypePath, Debug, Deserialize, Serialize)]
pub struct CombatMap {
    pub bitmap: String,
    pub zones: Vec<MapZone>,
}

#[derive(Default)]
pub struct CombatMapAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CombatMapAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
}

impl AssetLoader for CombatMapAssetLoader {
    type Asset = CombatMap;
    type Settings = ();
    type Error = CombatMapAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<CombatMap>(&bytes)?;
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["custom"]
    }
}
