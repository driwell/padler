use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(20., 120.);
const PADDLE_COLOR: Color = Color::srgb(0.898, 0.784, 0.565);
const PADDLE_SPEED: f32 = 500.;
const PADDLE_X_MARGIN: f32 = 10.;

const WALL_X: f32 = 350.;
const WALL_Y: f32 = 450.;
const WALL_THICKNESS: f32 = 10.;
const LEFT_WALL_COLOR: Color = Color::srgb(1., 0., 0.);
const RIGHT_WALL_COLOR: Color = Color::srgb(0., 1., 0.);
const BOTTOM_WALL_COLOR: Color = Color::srgb(0., 0., 1.);
const TOP_WALL_COLOR: Color = Color::srgb(1., 0.5, 0.);

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0., 0., 1.);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(-WALL_X, 0.),
            WallLocation::Right => Vec2::new(WALL_X, 0.),
            WallLocation::Bottom => Vec2::new(0., -WALL_Y),
            WallLocation::Top => Vec2::new(0., WALL_Y),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = WALL_Y - -WALL_Y;
        let arena_width = WALL_X - -WALL_X;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation, color: Color) -> WallBundle {
        WallBundle {
            sprite: Sprite::from_color(color, Vec2::ONE),
            transform: Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
            collider: Collider,
        }
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let paddle_x = -WALL_X + PADDLE_X_MARGIN;

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

    commands.spawn(WallBundle::new(WallLocation::Left, LEFT_WALL_COLOR));
    commands.spawn(WallBundle::new(WallLocation::Right, RIGHT_WALL_COLOR));
    commands.spawn(WallBundle::new(WallLocation::Bottom, BOTTOM_WALL_COLOR));
    commands.spawn(WallBundle::new(WallLocation::Top, TOP_WALL_COLOR));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_STARTING_POSITION)
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
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

    let bottom_bound = -WALL_Y + WALL_THICKNESS / 2. + PADDLE_SIZE.y / 2.;
    let top_bound = WALL_Y - WALL_THICKNESS / 2. - PADDLE_SIZE.y / 2.;
    paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
