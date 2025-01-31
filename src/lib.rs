use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_event::<CollisionEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                move_computer_paddle,
                check_for_collisions,
            ),
        );
        app.add_systems(Update, update_scoreboard);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct Score(i32);

#[derive(Component)]
struct ScoreboardUi;

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

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

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
struct PlayerGoal;

#[derive(Component)]
struct ComputerGoal;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(
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
        Player,
    ));

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(-paddle_x, 0., 0.),
            scale: PADDLE_SIZE.extend(1.),
            ..default()
        },
        Paddle,
        Collider,
        Computer,
    ));

    commands.spawn((
        PlayerGoal,
        WallBundle::new(WallLocation::Left, LEFT_WALL_COLOR),
    ));
    commands.spawn((
        ComputerGoal,
        WallBundle::new(WallLocation::Right, RIGHT_WALL_COLOR),
    ));
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

    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, (With<Paddle>, With<Player>)>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction -= 0.5;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction += 0.5;
    }

    reposition_paddle(&mut paddle_transform, time, direction);
}

fn move_computer_paddle(
    ball_query: Single<&Transform, (With<Ball>, Without<Paddle>)>,
    mut paddle_transform: Single<&mut Transform, (With<Paddle>, With<Computer>)>,
    time: Res<Time>,
) {
    let ball_transform = ball_query.into_inner();

    let mut direction = 0.0;

    if ball_transform.translation.y < paddle_transform.translation.y {
        direction -= 0.5;
    } else {
        direction += 0.5;
    }

    reposition_paddle(&mut paddle_transform, time, direction);
}

fn reposition_paddle<T: bevy::prelude::Component>(
    paddle_transform: &mut Single<&mut Transform, (With<Paddle>, With<T>)>,
    time: Res<Time>,
    direction: f32,
) {
    let new_paddle_position =
        paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

    let bottom_bound = -WALL_Y + WALL_THICKNESS / 2. + PADDLE_SIZE.y / 2.;
    let top_bound = WALL_Y - WALL_THICKNESS / 2. - PADDLE_SIZE.y / 2.;
    paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

type Goal<'a> = (Option<&'a PlayerGoal>, Option<&'a ComputerGoal>);

fn check_for_collisions(
    mut score: ResMut<Score>,
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Goal), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    for (_, collider_transform, goal) in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            if goal.0.is_some() {
                **score -= 1;
            }

            if goal.1.is_some() {
                **score += 1;
            }

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn update_scoreboard(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
