use bevy::prelude::*;
use padler::Game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Game)
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}
