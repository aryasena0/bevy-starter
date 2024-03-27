use bevy::prelude::*;
use bevy_starter::plug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //[user-defined-plugins]
        .add_plugins(plug::example::ExamplePlugin)
        .run();
}
