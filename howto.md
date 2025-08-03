## How to spawn a sprite

Similarly to how we spawn the camera, we can spawn a sprite:
`commands.spawn(Sprite::default());`

If we use the default option, the sprite will be a white pixel in the center of the screen:

```
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins) // game window
        .add_systems(Startup, setup) // spawn camera and sprite
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn(Sprite::default());
}
```

`bevy run` if using bevy cli OR `cargo run` to see the result.

In order to make the sprite "bigger" we can change its size by accessing the custom_size property:

```
...
commands.spawn(Sprite {
        custom_size: Some(Vec2::new(100., 100.)),
        ..default()
    });
...
```

Rust-wise, there is a lot to unpack here, so please refer to the notes files for details.




