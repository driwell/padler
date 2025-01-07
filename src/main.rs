use bevy::prelude::*;
use padler::{
    apply_velocity, check_for_collisions, move_computer_paddle, move_paddle, setup, CollisionEvent,
    Score,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(Score(0))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                move_computer_paddle,
                check_for_collisions,
            ),
        )
        .run();
}
