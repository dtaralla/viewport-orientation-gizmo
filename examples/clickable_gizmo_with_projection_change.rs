//! Advanced example.
//! Demonstrates how to enable a clickable gizmo and attach it to a camera, rotating the camera
//! through the shortest path to align its Z with the clicked axis. Use LCtrl when clicking to
//! align the view in the opposite direction.
//! Click the middle cube to switch from orthographic to perspective projection.

use bevy::prelude::*;
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
            gizmo: my_gizmo,
            size: 128,
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
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0))
                        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                    ..default()
                })
                .insert(MainCamera);
        })
        // This is what enabled rotation tracking on that camera
        .insert(TrackedRotator);
}

#[derive(Component)]
struct MainCamera;

fn react_to_clicks(
    mut commands: Commands,
    mut events: EventReader<ClickEvent>,
    keyboard: Res<Input<KeyCode>>,
    rotator: Query<(Entity, &mut Transform), With<TrackedRotator>>,
    animations: Query<&mut EasingComponent<Transform>, With<TrackedRotator>>,
    camera: Query<
        (
            Entity,
            &Transform,
            Option<&PerspectiveProjection>,
            Option<&OrthographicProjection>,
        ),
        (With<MainCamera>, Without<TrackedRotator>),
    >,
) {
    const ANIM_DURATION: Duration = Duration::from_millis(1000);

    let dir = if keyboard.pressed(KeyCode::LControl) {
        1.
    } else {
        -1.
    };

    let (entity, rotator) = rotator.single();
    for event in events.iter() {
        if let Some(axis) = event.0 {
            if let Ok(current_anim) = animations.get(entity) {
                if current_anim.state == EasingState::Play {
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
        } else if let (entity, tfm, Some(_), None) = camera.single() {
            commands
                .entity(entity)
                .remove::<PerspectiveProjection>()
                .insert(OrthographicProjection {
                    scale: tfm.translation.z / 1000.,
                    ..default()
                });
        } else if let (entity, _, None, Some(_)) = camera.single() {
            commands
                .entity(entity)
                .remove::<OrthographicProjection>()
                .insert(PerspectiveProjection::default());
        }
    }
}

gizmo![my_gizmo(meshes, materials):
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 1.0,
            max_y: 0.15,
            max_z: 0.15,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("b82700").unwrap(),
            unlit: true,
            ..default()
        }),
        transform: Transform::identity(),
        ..default()
    } ; RaycastableGizmo::default() ; GizmoClickableAxis::X,
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.15,
            max_y: 1.0,
            max_z: 0.15,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("5d9900").unwrap(),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    } ; RaycastableGizmo::default() ; GizmoClickableAxis::Y,
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.15,
            max_y: 0.15,
            max_z: 1.0,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("2e78e4").unwrap(),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    } ; RaycastableGizmo::default() ; GizmoClickableAxis::Z,
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: -0.15,
            min_y: -0.15,
            min_z: -0.15,
            max_x: 0.35,
            max_y: 0.35,
            max_z: 0.35,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            unlit: true,
            ..default()
        }),
        transform: Transform::default(),
        ..default()
    } ; RaycastableGizmo::default(),
];
