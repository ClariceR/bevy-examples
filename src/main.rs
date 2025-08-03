use bevy::prelude::*;
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins) // game window
        .insert_resource(ClearColor(Color::linear_rgb(0.5, 0., 0.2))) // backgorund colour
        .add_systems(Startup, setup) // spawn camera
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn(Sprite {
        custom_size: Some(Vec2::new(100., 100.)),
        ..default()
    });
}

