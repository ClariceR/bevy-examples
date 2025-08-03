use bevy::{prelude::*, window::WindowResolution};

fn main() -> AppExit {
    App::new()
        .add_plugins
        (
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window { 
                        resolution: WindowResolution::new(640., 480.).into(),
                        title: "Game Name".into(),
                        name: Some("Primary Window".to_string()),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
        ) // game window
        .insert_resource(ClearColor(Color::linear_rgb(0.5, 0., 0.2))) // backgorund colour
        .add_systems(Startup, setup) // spawn camera
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
