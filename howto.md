## How to create the game window:

The window is added when DefaultPlugins is added:
`.add_plugins(DefaultPlugins)`

```
fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}
```
This plugin group will add [all the default plugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) for a Bevy application.

The Plugin that defines an interface for windowing support in Bevy is the [WindowPlugin](https://docs.rs/bevy/latest/bevy/prelude/struct.WindowPlugin.html)

---
## How to spawn the camera:

`commands.spawn(Camera2d::default());`

You can add it in a system, like a setup system for example:

```
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
```

In order to run the system, add it to the main function:
```
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}
```

The [Startup](https://docs.rs/bevy/latest/bevy/app/struct.Startup.html) schedule runs once when the app starts.

So, it will spawn a 2d camera once at the start.

---
## How to change the color of the background*:

`.insert_resource(ClearColor(Color::linear_rgb(0.5, 0., 0.2)))`

Struct [ClearColor](https://docs.rs/bevy/latest/bevy/prelude/struct.ClearColor.html) is a [resource](https://docs.rs/bevy/latest/bevy/prelude/trait.Resource.html) that stores the color that is used to clear the screen between frames.

*The color appears as the “background” color for simple apps, when there are portions of the screen with nothing rendered.*

```
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::linear_rgb(0.5, 0., 0.2)))
        .add_systems(Startup, setup)
        .run()
}
```

There are many ways to define the [Color](https://docs.rs/bevy/latest/bevy/prelude/enum.Color.html). The following example shows the use of the color white from the bevy color css palette option:

`bevy::color::palettes::css::WHITE`

```
fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::from(WHITE)))
        .add_systems(Startup, setup)
        .run()
}
```