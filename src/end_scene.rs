use bevy::prelude::*;

use crate::states::GameState;
use crate::utils::despawn_screen;
use crate::TEXT_COLOR;

pub struct TheEnd;

impl Plugin for TheEnd {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TheEnd), end_setup)
            .add_systems(
                Update,
                (bevy::window::close_on_esc).run_if(in_state(GameState::TheEnd)),
            )
            .add_systems(OnExit(GameState::TheEnd), despawn_screen::<EndScreen>);
    }
}

#[derive(Component)]
struct EndScreen;

fn end_setup(mut commands: Commands) {
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
                background_color: Color::BLACK.into(),
                ..default()
            },
            EndScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "The End.",
                    TextStyle {
                        font_size: 80.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
}
