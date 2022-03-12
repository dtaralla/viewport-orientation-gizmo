//! Feature allowing to receive clicks made on the meshes representing the gizmo axis.

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ElementState;
use bevy::prelude::*;
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, RayCastMesh, RayCastMethod, RayCastSource, RaycastSystem,
};

use crate::{FirstPassCamera, GizmoUi};

pub(crate) struct ClickReactionPlugin;

impl Plugin for ClickReactionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DefaultRaycastingPlugin::<GizmoRaycastSet>::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                update_raycast_with_cursor.before(RaycastSystem::BuildRays),
            )
            // Make sure this startup runs last, so that the FirstPassCamera is already spawned
            .add_startup_system_to_stage(StartupStage::PostStartup, setup)
            .add_system(listen_for_clicks)
            .add_event::<ClickEvent>();
    }
}

pub struct GizmoRaycastSet;

pub type RaycastableGizmo = RayCastMesh<GizmoRaycastSet>;

fn setup(mut commands: Commands, query: Query<&Children, With<FirstPassCamera>>) {
    if let Ok(e) = query.get_single() {
        let e = e.get(0).unwrap();
        commands
            .entity(*e)
            .insert(RayCastSource::<GizmoRaycastSet>::default());
    }
}

/// Represents a gizmo axis.
#[derive(Clone, Copy, Debug)]
pub enum GizmoAxis {
    X,
    Y,
    Z,
}

impl From<GizmoAxis> for Vec3 {
    fn from(axis: GizmoAxis) -> Self {
        match axis {
            GizmoAxis::X => Vec3::X,
            GizmoAxis::Y => Vec3::Y,
            GizmoAxis::Z => Vec3::Z,
        }
    }
}

/// Attach this component to the meshes you want to be clickable.
#[derive(Component)]
pub struct GizmoClickable {
    /// The axis this mesh represents.
    pub axis: GizmoAxis,
}

/// Event sent when one of the [GizmoClickable] is left-clicked. Contains the [GizmoAxis] that was
/// clicked.
pub struct ClickEvent(pub GizmoAxis);

fn listen_for_clicks(
    mut events: EventWriter<ClickEvent>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    tracked_entities: Query<(Entity, &GizmoClickable), With<RayCastMesh<GizmoRaycastSet>>>,
    raycast_src: Query<&RayCastSource<GizmoRaycastSet>>,
) {
    // Did we receive a left mouse click this frame?
    let mut left_click = false;
    for e in mouse_button_events.iter() {
        if e.button == MouseButton::Left && e.state == ElementState::Released {
            left_click = true;
            break;
        }
    }
    if !left_click {
        return;
    }

    // Did we click on one of the gizmo parts? If yes, derive which axis it represents
    let mut axis = None;
    for s in raycast_src.iter() {
        if let Some(ls) = s.intersect_list() {
            for intersect in ls {
                if let Ok(clickable) = tracked_entities.get(intersect.0) {
                    axis = Some(clickable.1.axis);
                    break;
                }
            }
        }
    }

    if axis.is_none() {
        //
        return;
    }

    events.send(ClickEvent(axis.unwrap()))
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<GizmoRaycastSet>>,
    gizmo_ui: Query<(&Interaction, &GlobalTransform, &Node), With<GizmoUi>>,
) {
    // Grab the most recent cursor event if it exists:
    let mut cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    let (gizmo_interaction, global_transform, node) = gizmo_ui.single();

    // Not over the gizmo UI, don't even update raycast source
    if gizmo_interaction == &Interaction::None {
        return;
    };

    // Get back cursor_position in the size x size rendered texture coordinate system
    {
        let position = global_transform.translation;
        let ui_position = position.truncate();
        let extents = node.size / 2.0;
        let min = ui_position - extents;
        cursor_position -= min;
    }

    for mut pick_source in &mut query.iter_mut() {
        pick_source.cast_method = RayCastMethod::Screenspace(cursor_position);
    }
}
