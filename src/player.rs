use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::Animation, WINDOW_BOTTOM_Y, WINDOW_LEFT_X};

use self::{
    direction::update_direction,
    jump::{apply_jump_sprite, fall, jump, rise},
};

mod direction;
mod jump;

const PLAYER_VELOCITY_X: f32 = 200.0;

const SPRITESHEET_COLS: usize = 7;
const SPRITESHEET_ROWS: usize = 8;
const SPRITE_TILE_WIDTH: f32 = 128.0;
const SPRITE_TILE_HEIGHT: f32 = 256.0;

const SPRITE_IDX_STAND: usize = 6;
const SPRITE_IDX_JUMP: usize = 13;
const SPRITE_IDX_WALKING: &[usize] = &[47, 40];

const SPRITE_RENDER_WIDTH: f32 = 64.0;
const SPRITE_RENDER_HEIGHT: f32 = 128.0;

const CYCLE_DELAY: Duration = Duration::from_millis(70);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement) // Moves the player based on input
            .add_system(jump) // Initiates a jump, on player input
            .add_system(rise) // Moves the player up, while jumping
            .add_system(fall) // Makes the player fall to the ground
            .add_system(apply_movement_animation) // Adds animation for horizontal movement
            .add_system(apply_idle_sprite) // Stops the animation and displays the STAND sprite
            .add_system(apply_jump_sprite) // Stops the animation and displays the jump sprite
            .add_system(update_direction) // Sets the direction the player is moving in for animations
            .add_system(update_sprite_direction); // Sets the sprite in the correct direction
    }
}

fn setup(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    // Load the spritesheet image
    let image_handle: Handle<Image> = server.load("kenney/Spritesheets/spritesheet_players.png");

    // Create a spritesheet atlas, that finds textures in a spritesheet
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );

    // Add it to the atlasses
    let atlas_handle = atlases.add(texture_atlas);

    // Spawn the player
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(SPRITE_IDX_STAND),
            texture_atlas: atlas_handle,
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                scale: Vec3::new(
                    SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
                    SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))
        .insert(KinematicCharacterController::default())
        .insert(direction::Direction::Right); // Default direction
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    // Take a mutable reference to the kinematic character controller
    let mut player = query.single_mut();

    // Create a translation vector
    let mut movement = 0.0;

    // If the player pressed the right arrow key
    if input.pressed(KeyCode::Right) {
        // Add the distance the player should move to the right to the translation
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    // If the player pressed the left arrow key
    if input.pressed(KeyCode::Left) {
        // Subtract the distance the player should move to the left, from the translation
        movement -= time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    // Set the resulting translation
    match player.translation {
        Some(translation) => player.translation = Some(Vec2::new(movement, translation.y)),
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

fn apply_movement_animation(
    mut commands: Commands,
    query: Query<(Entity, &mut KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.desired_translation.x != 0.0 && output.grounded {
        commands
            .entity(player)
            .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
    }
}

fn apply_idle_sprite(
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
    if output.desired_translation.x == 0.0 && output.grounded {
        commands.entity(player).remove::<Animation>();
        sprite.index = SPRITE_IDX_STAND;
    }
}

fn update_sprite_direction(mut query: Query<(&mut TextureAtlasSprite, &direction::Direction)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();

    match direction {
        direction::Direction::Right => sprite.flip_x = false,
        direction::Direction::Left => sprite.flip_x = true,
    }
}
