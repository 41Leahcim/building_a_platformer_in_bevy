use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use platform::PlatformsPlugin;
use player::PlayerPlugin;

mod animation;
mod platform;
mod player;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

const FLOOR_THICKNESS: f32 = 10.0;

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
        .add_plugin(PlatformsPlugin) // Self-made plugin
        .add_plugin(PlayerPlugin)
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

fn setup(mut commands: Commands) {
    // Draw the floor
    draw_floor(&mut commands);

    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
}
