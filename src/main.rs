use bevy::prelude::*;
use padler::{apply_velocity, check_for_collisions, move_paddle, setup, CollisionEvent};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_velocity, move_paddle, check_for_collisions),
        )
        .run();
}
