use bevy::prelude::*;

mod systems;
mod components;

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // So the With type allows us to say “I want entities that have a snake head, 
    // but I don’t care about the snake head component itself, just give me the transform
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn main() {
    App::new()
    .add_startup_system(systems::camera::setup_camera)
    .add_system(snake_movement)
    .add_startup_system(spawn_snake)
    // adds basic systems like
    // create a window, one to do a render loop, 
    // one to handle input, one to handle sprites
    .add_plugins(DefaultPlugins)
    .run();
}
