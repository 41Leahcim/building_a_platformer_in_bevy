use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const COLOR_BACKGROUND: Color = Color::rgb(0.29, 0.31, 0.41);
const COLOR_PLATFORM: Color = Color::rgb(0.13, 0.13, 0.23);
const COLOR_PLAYER: Color = Color::rgb(0.6, 0.55, 0.6);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Platformer".to_owned(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_startup_system(setup)
        .run();
}

fn draw_platform(commands: &mut Commands, position: Vec3, scale: Vec3) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: COLOR_PLATFORM,
            ..Default::default()
        },
        transform: Transform {
            translation: position,
            scale,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Hello, World!");

    // Spawn 3 rectangles
    draw_platform(
        &mut commands,
        Vec3::new(-100.0, WINDOW_BOTTOM_Y + (200.0 / 2.0), 0.0),
        Vec3::new(75.0, 200.0, 1.0),
    );

    draw_platform(
        &mut commands,
        Vec3::new(100.0, WINDOW_BOTTOM_Y + (350.0 / 2.0), 0.0),
        Vec3::new(50.0, 350.0, 1.0),
    );

    draw_platform(
        &mut commands,
        Vec3::new(350.0, WINDOW_BOTTOM_Y + (250.0 / 2.0), 0.0),
        Vec3::new(150.0, 250.0, 1.0),
    );

    /* Other examples of spawning an object
    commands.spawn(Transform::from_xyz(0.0, 0.0, 0.0));
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Sprite {
            color: Color::LIME_GREEN,
            ..Default::default()
        },
    )); */

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(COLOR_PLAYER)),
        transform: Transform {
            translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 30.0, 0.0),
            scale: Vec3::new(30.0, 30.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
}
