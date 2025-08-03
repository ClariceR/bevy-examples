# First steps

In the main.rs file, import [bevy prelude](https://docs.rs/bevy/latest/bevy/prelude/index.html): `use bevy::prelude::*;`
This will import common components, bundles, and plugins.


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
### Bevy App

The Struct [App](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html) is the primary API for writing user applications. It automates the setup of a standard lifecycle and provides interface glue for plugins.

### `.run()`

[Runs](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html#method.run) the App by calling its runner.
This will (re)build the App first. 

---
All links:

[Bevy Prelude Doc](https://docs.rs/bevy/latest/bevy/prelude/index.html)

[Full list of the default plugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html)

[WindowPlugin](https://docs.rs/bevy/latest/bevy/prelude/struct.WindowPlugin.html)

[App](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html)

[.run()](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html#method.run)