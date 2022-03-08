use bevy::prelude::*;
use smooth_bevy_cameras::controllers::unreal::{
    UnrealCameraBundle, UnrealCameraController, UnrealCameraPlugin,
};
use smooth_bevy_cameras::LookTransformPlugin;
use viewport_orientation_gizmo::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        // Enables the system that draws the gizmo
        .add_plugin(ViewportOrientationGizmoPlugin::new())
        .add_startup_system(setup)
        .run();
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
