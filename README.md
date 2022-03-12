[![crates.io](https://img.shields.io/crates/v/viewport_orientation_gizmo)](https://crates.io/crates/viewport_orientation_gizmo)
[![docs.rs](https://docs.rs/viewport_orientation_gizmo/badge.svg)](https://docs.rs/viewport_orientation_gizmo)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

A way to display the world's reference frame orientation at all times, to help when authoring in Bevy viewports. Can
also be used to display the orientation of any other entity featuring a `Transform`.

**PRs welcomed!**

# Prerequisites

This uses the
experimental *[Render to texture](https://github.com/bevyengine/bevy/blob/main/examples/3d/render_to_texture.rs)*
feature of Bevy.

I'm trying to follow Bevy main whenever there are breaking changes. See table below for which version needs what minimum
Bevy version/Bevy main commit.

| x         | Supported features | Bevy min. version | Bevy main                                                                                     |
|-----------|--------------------|-------------------|-----------------------------------------------------------------------------------------------|
| **0.3.0** | all                | N/A[^1]           | [bf6de89](https://github.com/bevyengine/bevy/commit/bf6de8962287050369cd98605490bdd7770c87b4) |
| **0.2.0** | all                | N/A[^1]           | [81d57e1](https://github.com/bevyengine/bevy/commit/81d57e129b507047ab165b1cee1975cd54ba100f) |

[^1]: There is currently no official release of Bevy including the required commit.

# Usage

Add the plugin, then attach a `TrackedRotator` component to your camera (or any other entity with a `Transform`, if you
want to track the rotation of something else).

```rust
use bevy::prelude::*;
use viewport_orientation_gizmo::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Enables the system that draws the gizmo
        .add_plugin(ViewportOrientationGizmoPlugin::new())
        .add_startup_system(setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        // Enables tracking of this camera
        .insert(TrackedRotator);
}
```

# Plugin options
By default, the plugin will create a 64x64 area in the bottom left corner of the canvas where
the gizmo will be drawn.
When adding the plugin, you can set a few options to change this behavior using the initializer
instead of `ViewportOrientationGizmoPlugin::new()`.
The function `ViewportOrientationGizmoPlugin::new_sized(...)` can also be used if you are only
interested in changing the default size but keep it to the bottom left corner.
```rust
use bevy::prelude::*;
use viewport_orientation_gizmo::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Make gizmo a 120x120 area in the somewhat top-right of the screen.
        .add_plugin(ViewportOrientationGizmoPlugin::custom(PluginOptions {
            size: 120,
            location: CanvasLocation::Custom(Rect {
                top: Val::Percent(5.0),
                right: Val::Px(10.0),
                ..default()
            }),
            // Use the default gizmo mesh
            ..default()
        }))
        .add_startup_system(setup)
        .run();
}
fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        // Enables tracking of this camera
        .insert(TrackedRotator);
}
```

## Custom gizmos
This crate supports customized gizmos through the use of the `gizmo!` macro and setting the
`PluginOptions.gizmo` option.

See `examples/custom_gizmo.rs` and `examples/flashing_gizmo.rs`.

## Clickable gizmos
This plugin can emit click events when the meshes constituting your gizmo are clicked.

This feature is optional; you will need to include `click-reaction`.

See `examples/clickable_gizmo.rs`.
