use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins) // game window
        .insert_resource(ClearColor(Color::linear_rgb(0.2, 0.1, 0.9))) // backgorund colour
        .add_systems(Startup, setup) // spawn camera
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let image_handle = asset_server.load("Arex_V_char_example.png");

    //commands.spawn(Sprite::from_image(asset_server.load("Arex_V_char_example.png"))); // if you want to load the image without any adjustments
    commands.spawn(Sprite {
        image: image_handle,
        custom_size: Some(Vec2::new((19. * 4.), (29. * 4.))), //if you want to scale it, get the size of the sprite image and multiply it by a f32 to get desired size
        ..default()
    });
}