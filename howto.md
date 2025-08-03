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