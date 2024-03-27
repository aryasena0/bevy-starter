//! ## Example Plugin
//!
//! use bevy::prelude::*;
//!
//! pub struct ExamplePlugin;
//!
//! impl Plugin for ExamplePlugin {
//!    fn build(&self, app: &mut App) {
//!       app.add_systems(Update, spam_hello_sampai_nangis);
//!    }
//! }
//!
//! fn spam_hello_sampai_nangis() {
//!   println!("Hello world!")
//! }
//!
//! This is the example plugin that will be loaded by the main.rs
//!
//! Note that we separate the plugin into different files for better organization.
//! This plugin will print "Hello world!" to the console every frame and the
//! System functions are local scoped for better organization.
use bevy::prelude::*;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spam_hello_sampai_nangis);
    }
}

fn spam_hello_sampai_nangis() {
    println!("Hello world!")
}
