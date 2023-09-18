use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::Animation;

use super::SPRITE_IDX_JUMP;

const FALL_DIVIDER: f32 = 1.25;
const MAX_JUMP_HEIGHT: f32 = 230.0;
const PLAYER_VELOCITY_Y: f32 = 400.0;

#[derive(Debug, Component)]
pub struct Jump(pub f32);

pub fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (With<KinematicCharacterController>, Without<Jump>),
    >,
) {
    // Only continue if the player doesn't isn't jumping
    if query.is_empty() {
        return;
    }

    // Take a shared reference to the player
    let (player, output) = query.single();

    // If the player is pressing the up key and is grounded
    if input.pressed(KeyCode::Up) && output.grounded {
        // Make the player jump
        commands.entity(player).insert(Jump(0.0));
    }
}

pub fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump)>,
) {
    // Only continue, if the player is jumping
    if query.is_empty() {
        return;
    }

    // Take a mutable reference to the entity, character controller, and jump component
    let (entity, mut player, mut jump) = query.single_mut();

    // Calculate the jump movement in the time since the last frame
    let mut movement = time.delta().as_secs_f32() * PLAYER_VELOCITY_Y;

    // If the player reached the maximum jump height
    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        // Finish the jump
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    // Add the movement value to the current jump height
    jump.0 += movement;

    // Set the resulting translation
    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
    // Only continue, if the player isn't jumping
    if query.is_empty() {
        return;
    }

    // Take a mutable reference to the player
    let mut player = query.single_mut();

    // Move the player down slower than it rises
    let movement = time.delta().as_secs_f32() * -(PLAYER_VELOCITY_Y / FALL_DIVIDER);

    // Set the resulting translation
    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

pub fn apply_jump_sprite(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlasSprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if !output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_JUMP;
    }
}
