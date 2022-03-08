//! [![crates.io](https://img.shields.io/crates/v/viewport_orientation_gizmo)](https://crates.io/crates/viewport_orientation_gizmo)
//! [![docs.rs](https://docs.rs/viewport_orientation_gizmo/badge.svg)](https://docs.rs/viewport_orientation_gizmo)
//!
//! A way to display the world's reference frame orientation at all times, to help when authoring in
//! Bevy viewports.
//! Can also be used to display the orientation of any other entity featuring a `Transform`.
//!
//! **PRs welcomed!**
//!
//! # Usage
//!
//! Add the plugin, then attach a [TrackedRotator] component to your camera (or any other entity
//! with a `Transform`, if you want to track the rotation of something else).
//!
//! ```rust
//! use bevy::prelude::*;
//! use viewport_orientation_gizmo::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Enables the system that draws the gizmo
//!         .add_plugin(ViewportOrientationGizmoPlugin::new())
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands
//!         .spawn_bundle(PerspectiveCameraBundle {
//!             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
//!                 .looking_at(Vec3::default(), Vec3::Y),
//!             ..default()
//!         })
//!         // Enables tracking of this camera
//!         .insert(TrackedRotator);
//! }
//! ```
//!
//! # Plugin options
//!
//! By default, the plugin will create a 64x64 area in the bottom left corner of the canvas where
//! the gizmo will be drawn.
//! When adding the plugin, you can set a few options to change this behavior using
//! [ViewportOrientationGizmoPlugin::custom] instead of [ViewportOrientationGizmoPlugin::new].
//! The function [ViewportOrientationGizmoPlugin::new_sized] can also be used if you are only
//! interested in changing the default size but keep it to the bottom left corner.
//!
//! ```rust
//! use bevy::prelude::*;
//! use viewport_orientation_gizmo::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Make gizmo a 120x120 area in the somewhat top-right of the screen.
//!         .add_plugin(ViewportOrientationGizmoPlugin::custom(PluginOptions {
//!             size: 120,
//!             location: CanvasLocation::Custom(Rect {
//!                 top: Val::Percent(5.0),
//!                 right: Val::Px(10.0),
//!                 ..default()
//!             }),
//!             // Use the default gizmo mesh
//!             ..default()
//!         }))
//!         .add_startup_system(setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands
//!         .spawn_bundle(PerspectiveCameraBundle {
//!             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
//!                 .looking_at(Vec3::default(), Vec3::Y),
//!             ..default()
//!         })
//!         // Enables tracking of this camera
//!         .insert(TrackedRotator);
//! }
//! ```
//!
//! ## Custom gizmos
//!
//! This crate supports customized gizmos through the use of the [gizmo] macro and setting the
//! [PluginOptions::gizmo] option.
//! See `examples/custom_gizmo.rs` and `examples/flashing_gizmo.rs`.
//!
//! # Known bugs & limitations
//!
//! - This crash might occur for some gizmo sizes:
//! ```thread 'main' panicked at 'assertion failed: self.axis_slices.x * self.axis_slices.y * self.axis_slices.z <= 4096'```
//! In that case, try to use the default or find another size where it doesn't.
//!
//! I've submitted a [bug report](https://github.com/bevyengine/bevy/issues/4127).

mod first_pass;
mod gizmo;
mod plugin;

pub use gizmo::*;
pub use plugin::*;
