//! Simple example.
//! Demonstrates how to enable a clickable gizmo and attach it to a camera, rotating the camera
//! through the shortest path to align its Z with the clicked axis. Use LCtrl when clicking to
//! align the view in the opposite direction.

use bevy::prelude::*;
use bevy_easings::EasingState::Play;
use bevy_easings::*;
use std::f32::consts::PI;
use std::time::Duration;

use viewport_orientation_gizmo::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        // Enables the system that draws the gizmo
        .add_plugin(ViewportOrientationGizmoPlugin::custom(PluginOptions {
            gizmo: GIZMO,
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(react_to_clicks)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sphere along X-
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::MAROON.into()),
        transform: Transform::from_xyz(-5.0, 0.0, 0.0),
        ..default()
    });
    // Sphere along X+
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(5.0, 0.0, 0.0),
        ..default()
    });

    // Sphere along -Y
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::DARK_GREEN.into()),
        transform: Transform::from_xyz(0.0, -5.0, 0.0),
        ..default()
    });

    // Sphere along +Y
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });

    // Sphere along -Z
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::MIDNIGHT_BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.0, -5.0),
        ..default()
    });

    // Sphere along +Z
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        material: materials.add(Color::NAVY.into()),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands
        .spawn_bundle(TransformBundle::from(
            Transform::identity().looking_at(Vec3::new(0.0, 0.0, -1.0), Vec3::Y),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
                    .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                ..default()
            });
        })
        // This is what enabled rotation tracking on that camera
        .insert(TrackedRotator);
}

fn react_to_clicks(
    mut commands: Commands,
    mut events: EventReader<ClickEvent>,
    keyboard: Res<Input<KeyCode>>,
    rotator: Query<(Entity, &mut Transform), With<TrackedRotator>>,
    animations: Query<&mut EasingComponent<Transform>, With<TrackedRotator>>,
) {
    const ANIM_DURATION: Duration = Duration::from_millis(1000);

    let dir = if keyboard.pressed(KeyCode::LControl) {
        1.
    } else {
        -1.
    };

    let (entity, rotator) = rotator.single();
    for event in events.iter() {
        let axis = event.0;
        if let Ok(current_anim) = animations.get(entity) {
            if current_anim.state == Play {
                // Wait for animations to finish before allowing another rotation
                return;
            } else {
                commands
                    .entity(entity)
                    .remove::<EasingComponent<Transform>>();
            }
        }

        // Align camera Z with selected direction
        let rot_axis = dir * Vec3::from(axis).cross(rotator.local_z());
        commands.entity(entity).insert(
            rotator.ease_to(
                Transform::identity()
                    .with_rotation(Quat::from_axis_angle(rot_axis, PI / 2.) * rotator.rotation),
                EaseFunction::ExponentialOut,
                EasingType::Once {
                    duration: ANIM_DURATION,
                },
            ),
        );
    }
}
