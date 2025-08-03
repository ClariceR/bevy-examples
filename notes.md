# First steps

In the main.rs file, import [bevy prelude](https://docs.rs/bevy/latest/bevy/prelude/index.html): `use bevy::prelude::*;`
This will import common components, bundles, and plugins.

---
### Bevy App

The Struct [App](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html) is the primary API for writing user applications. It automates the setup of a standard lifecycle and provides interface glue for plugins.

### `.run()`

[Runs](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html#method.run) the App by calling its runner.
This will (re)build the App first. 

---
### Resources

The [Resource](https://docs.rs/bevy/latest/bevy/prelude/trait.Resource.html) type is a type that can be inserted into a World as a singleton.

You can access resource data in systems using the Res and ResMut system parameters

Only one resource of each type can be stored in a World at any given time.

---
## Some()?

Sometimes it's desirable to catch the failure of some parts of a program instead of calling panic!; this can be accomplished using the Option enum.

The Option\<T> enum has two variants:

- None, to indicate failure or lack of value, and
- Some(value), a tuple struct that wraps a value with type T.

The [primary_window](https://docs.rs/bevy/latest/bevy/window/struct.PrimaryWindow.html) property is of type Option\<Window>.

The WindowPlugin will spawn a Window entity with this component if primary_window is Some.

---
All links:

[Bevy Prelude Doc](https://docs.rs/bevy/latest/bevy/prelude/index.html)

[Full list of the default plugins](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html)

[WindowPlugin](https://docs.rs/bevy/latest/bevy/prelude/struct.WindowPlugin.html)

[App](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html)

[.run()](https://docs.rs/bevy/latest/bevy/prelude/struct.App.html#method.run)