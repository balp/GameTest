use bevy::prelude::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    AssetsLoading,
    AssetsSetup,
    AssetsFailed,
    InteractiveFiction,
    Battle,
    BattleTurns,
    BattleEnded,
    TheEnd,
}
