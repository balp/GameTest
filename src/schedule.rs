use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum BattleUpdateSets {
    TurnChanges,
    UserInput,
    EntityUpdates,
}

impl Plugin for BattleUpdateSets {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                BattleUpdateSets::TurnChanges,
                BattleUpdateSets::UserInput,
                BattleUpdateSets::EntityUpdates,
            )
                .chain(),
        );
    }
}
