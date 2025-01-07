use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(20., 120.);
const PADDLE_COLOR: Color = Color::srgb(0.898, 0.784, 0.565);
const PADDLE_SPEED: f32 = 500.;
const PADDLE_X_MARGIN: f32 = 10.;
const PADDLE_Y_MARGIN: f32 = 10.;

const WALL_X: f32 = 350.;
const WALL_Y: f32 = 450.;
const WALL_THICKNESS: f32 = 10.;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
struct Collider;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let paddle_x = (WALL_X * -1.) + PADDLE_X_MARGIN;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(paddle_x, 0., 0.),
            scale: PADDLE_SIZE.extend(1.),
            ..default()
        },
        Paddle,
        Collider,
    ));
}

pub fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    let top_bound = (WALL_Y * -1.) + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_Y_MARGIN;
    let bottom_bound = WALL_Y - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_Y_MARGIN;
    paddle_transform.translation.y = new_paddle_position.clamp(top_bound, bottom_bound);
}
