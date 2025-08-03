## Some?

Sometimes it's desirable to catch the failure of some parts of a program instead of calling panic!; this can be accomplished using the [Option](https://doc.rust-lang.org/rust-by-example/std/option.html) enum.

The Option\<T> enum has two variants:

- None, to indicate failure or lack of value, and
- Some(value), a tuple struct that wraps a value with type T.

The [custom_size](https://docs.rs/bevy/latest/bevy/prelude/struct.Sprite.html#structfield.custom_size) property is of type Option\<Vec2>

The [Vec2](https://docs.rs/bevy/latest/bevy/prelude/struct.Vec2.html) is a struct that holds an x and y value, representing a 2-dimensional vector.

`custom_size: Some(Vec2::new(100., 100.))` reads like: *Set the sprite's x value to 100.0 and the y vlaue to 100.0*.

---
## ..default()?

We need to implement all the Sprite's traits so we can use ..default() at the end to take care of all the other traits we don't care about.


