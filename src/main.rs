use bevy::prelude::*;
use padler::{apply_velocity, move_paddle, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (apply_velocity, move_paddle))
        .run();
}
