use bevy::prelude::{Commands, Component, Entity, Query, With};
use bevy::hierarchy::DespawnRecursiveExt;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
