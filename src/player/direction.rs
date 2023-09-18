use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;

#[derive(Debug, Component)]
pub enum Direction {
    Left,
    Right,
}

pub fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();

    if output.desired_translation.x > 0.0 {
        commands.entity(player).insert(Direction::Right);
    } else if output.desired_translation.x < 0.0 {
        commands.entity(player).insert(Direction::Left);
    }
}
