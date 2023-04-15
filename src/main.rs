use std::time::Duration;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::component;
use bevy::prelude::*;

#[derive(Component)]
struct Animation {
    frames: Vec<Handle<Image>>,
    frame: usize,
    frame_timer: Timer,
}

impl Animation {
    pub fn new(frames: Vec<Handle<Image>>, frame: usize, repeating_duration: Duration) -> Self {
        Self {
            frames,
            frame,
            frame_timer: Timer::new(repeating_duration, TimerMode::Repeating),
        }
    }
}

fn animation(
    mut animation: Query<&mut Animation>,
    mut handle_image: Query<&mut Handle<Image>, With<Animation>>,
    time: Res<Time>,
) {
    for mut animated in animation.iter_mut() {
        for mut hi in handle_image.iter_mut() {
            animated.frame_timer.tick(time.delta());
            if animated.frame_timer.finished() {
                let hi = hi.as_mut();
                *hi = animated.frames.get(animated.frame).unwrap().clone();
                animated.frame = (animated.frame + 1) % animated.frames.len();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(animation)
        .run();
}

fn load_frames(file: &str, n: u8, asset_server: Res<AssetServer>) -> Vec<Handle<Image>> {
    let mut frames = Vec::new();
    for n in 0..n {
        frames.push(asset_server.load(::std::path::Path::new(&format!("{}{}.png", file, n))));
    }
    frames
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let frames = load_frames("knight_f_idle_anim_f", 4, asset_server);

    commands.spawn((
        SpriteBundle {
            texture: frames.get(0).unwrap().clone(),
            transform: Transform::default().with_scale(Vec3::splat(10.)),

            ..default()
        },
        Animation::new(frames, 0, Duration::from_millis(140)),
    ));

    commands.spawn(Camera2dBundle::default());
    // commands.spawn();
}
