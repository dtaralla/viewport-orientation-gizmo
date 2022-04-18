#![doc = include_str!("../README.MD")]

#[cfg(feature = "click-reaction")]
pub use click_reaction::*;
#[cfg(feature = "click-reaction")]
pub use clickable_gizmo::*;
pub use gizmo::*;
pub use plugin::*;

mod default_gizmo;
mod first_pass;
mod gizmo;
mod plugin;

#[cfg(feature = "click-reaction")]
mod click_reaction;
#[cfg(feature = "click-reaction")]
mod clickable_gizmo;
