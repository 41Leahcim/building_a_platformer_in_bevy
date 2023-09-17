use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use jump::Jump;
use platform::PlatformBundle;

mod jump;
mod platform;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);
const COLOR_PLAYER: Color = Color::rgb(0.6, 0.55, 0.6);
const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

const FLOOR_THICKNESS: f32 = 10.0;

const PLAYER_VELOCITY_X: f32 = 200.0;
const PLAYER_VELOCITY_Y: f32 = 400.0;
const FALL_DIVIDER: f32 = 1.25;
const MAX_JUMP_HEIGHT: f32 = 230.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND)) // Sets the background color
        // Set the window properties
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_owned(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Call setup at startup
        .add_startup_system(setup)
        // Use Rapier physics with 200 pixels per meter
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        // Display debug information like the size and center of physics objects
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(movement) // Moves the player based on input
        .add_system(jump) // Initiates a jump, on player input
        .add_system(rise) // Moves the player up, while jumping
        .add_system(fall) // Makes the player move to the ground
        .run();
}

fn draw_floor(commands: &mut Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: COLOR_FLOOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, WINDOW_BOTTOM_Y + FLOOR_THICKNESS / 2.0, 0.0),
                scale: Vec3::new(WINDOW_WIDTH, FLOOR_THICKNESS, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}

fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
            transform: Transform {
                translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Draw 3 platforms
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));

    // Draw the floor
    draw_floor(&mut commands);

    // Spawn the player
    spawn_player(&mut commands, &mut meshes, &mut materials);

    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
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

fn jump(
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

fn rise(
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

fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, Without<Jump>>) {
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
