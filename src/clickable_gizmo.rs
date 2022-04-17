use bevy::prelude::*;

use crate::click_reaction::*;
use crate::gizmo;

gizmo![the_gizmo(meshes, materials):
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
    } ; RaycastableGizmo::default() ; GizmoClickableAxis::Z
];

/// Default clickable gizmo
pub static GIZMO: crate::plugin::Gizmo = the_gizmo;
