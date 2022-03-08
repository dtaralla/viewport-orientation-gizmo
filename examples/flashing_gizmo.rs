//! Most advanced example.
//! Demonstrates how to customize the gizmo and add individual components to the bundles composing it.

use bevy::prelude::*;
use smooth_bevy_cameras::controllers::unreal::{
    UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin,
};
use smooth_bevy_cameras::LookTransformPlugin;
use viewport_orientation_gizmo::*;

struct AlphaTimer {
    timer: Timer,
    fading_out: bool,
}

struct PulseTimer {
    timer: Timer,
    shrinking: bool,
}

#[derive(Component)]
struct Flashing;

#[derive(Component)]
struct Pulsing;

gizmo![flashing_gizmo(meshes, materials):
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 1.0,
            max_y: 0.1,
            max_z: 0.1,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("b82700").unwrap(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::identity(),
        ..default()
    } ; Flashing,
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.1,
            max_y: 1.0,
            max_z: 0.1,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("5d9900").unwrap(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    } ; Flashing ; Pulsing,
    PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.1,
            max_y: 0.1,
            max_z: 1.0,
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::hex("2e78e4").unwrap(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    } ; Flashing,
];

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AlphaTimer {
            timer: Timer::from_seconds(1.5, true),
            fading_out: true,
        })
        .insert_resource(PulseTimer {
            timer: Timer::from_seconds(1.5, true),
            shrinking: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        // Enables flashing_gizmo
        .add_plugin(ViewportOrientationGizmoPlugin::custom(PluginOptions {
            gizmo: flashing_gizmo,
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(flash_gizmo)
        .add_system(pulse_gizmo)
        .run();
}

fn flash_gizmo(
    time: Res<Time>,
    mut timer: ResMut<AlphaTimer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&mut Handle<StandardMaterial>, With<Flashing>>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        timer.fading_out = !timer.fading_out;
    }

    for handle in query.iter_mut() {
        let material = materials.get_mut(handle.id).unwrap();
        let pc_elapsed = timer.timer.elapsed_secs() / 1.5;
        material.base_color.set_a(if timer.fading_out {
            (1.0 - pc_elapsed).max(0.4)
        } else {
            pc_elapsed.max(0.4)
        });
    }
}

fn pulse_gizmo(
    time: Res<Time>,
    mut timer: ResMut<PulseTimer>,
    mut query: Query<&mut Transform, With<Pulsing>>,
) {
    // const
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        timer.shrinking = !timer.shrinking;
    }

    for mut tf in query.iter_mut() {
        let pc_elapsed = timer.timer.elapsed_secs() / 1.5;

        tf.scale.y = if timer.shrinking {
            1.4 - pc_elapsed * 0.8
        } else {
            0.6 + pc_elapsed * 0.8
        };
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // reference frame visualizer
    commands.spawn_batch(vec![
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: 0.0,
                min_y: 0.0,
                min_z: 0.0,
                max_x: 10.0,
                max_y: 0.05,
                max_z: 0.05,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("b82700").unwrap(),
                unlit: true,
                ..default()
            }),
            transform: Transform::identity(),
            ..default()
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: 0.0,
                min_y: 0.0,
                min_z: 0.0,
                max_x: 0.05,
                max_y: 10.0,
                max_z: 0.05,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("5d9900").unwrap(),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: 0.0,
                min_y: 0.0,
                min_z: 0.0,
                max_x: 0.05,
                max_y: 0.05,
                max_z: 10.0,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("2e78e4").unwrap(),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ]);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // flying camera
    commands
        .spawn_bundle(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            PerspectiveCameraBundle::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
        ))
        // This is what enabled rotation tracking on that camera
        .insert(TrackedRotator);
}
