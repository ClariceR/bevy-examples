use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins) // To see how to customise the window, please refer to howto.md file
        .insert_resource(ClearColor(Color::linear_rgb(0.5, 0., 0.2))) // backgorund colour
        .add_systems(Startup, setup) // spawn camera
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}