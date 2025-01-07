use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(20., 120.);
const PADDLE_COLOR: Color = Color::srgb(0.898, 0.784, 0.565);
const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 10.;

const WALL_X: f32 = 350.;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let paddle_x = (WALL_X * -1.) + GAP_BETWEEN_PADDLE_AND_WALL;

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
