## How to load assets

First we need to create an assets folder.
If you create it in the root of the project, bevy will automagically find it.

We will have the assets folder on the same level as the src folder:
```
    ...
    > assets
    > src
    ...
```

And then we load the asset using asset_server resource.
Pass `asset_server: Res<AssetServer>` into the system's params, and use the load method.
If you don't need to make any image adjustments, you can use the from_image method and pass in the image handle.

```
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    commands.spawn(Sprite::from_image(asset_server.load("Arex_V_char_example.png"))); 
}
```

