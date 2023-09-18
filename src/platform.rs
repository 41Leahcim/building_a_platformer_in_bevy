use crate::{FLOOR_THICKNESS, WINDOW_BOTTOM_Y};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR_PLATFORM,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        x,
                        WINDOW_BOTTOM_Y + (scale.y / 2.0) + FLOOR_THICKNESS,
                        0.0,
                    ),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            },
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands) {
    println!("Hello from the platforms plugin!");

    // Draw 3 platforms
    commands.spawn(PlatformBundle::new(-100.0, Vec3::new(75.0, 200.0, 1.0)));
    commands.spawn(PlatformBundle::new(100.0, Vec3::new(50.0, 350.0, 1.0)));
    commands.spawn(PlatformBundle::new(350.0, Vec3::new(150.0, 250.0, 1.0)));
}
