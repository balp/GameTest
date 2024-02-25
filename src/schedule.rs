use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CombatUpdateSets {
    TurnChanges,
    UserInput,
    EntityUpdates,
}

impl Plugin for CombatUpdateSets {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                CombatUpdateSets::TurnChanges,
                CombatUpdateSets::UserInput,
                CombatUpdateSets::EntityUpdates,
            )
                .chain(),
        );
    }
}
