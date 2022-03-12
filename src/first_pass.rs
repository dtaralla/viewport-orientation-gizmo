/// Heavily inspired from the [render_to_texture][1] example
/// [1]: https://github.com/bevyengine/bevy/blob/main/examples/3d/render_to_texture.rs
use crate::FirstPassCamera;
use bevy::{
    core_pipeline::{draw_3d_graph, node, AlphaMask3d, Opaque3d, Transparent3d},
    prelude::*,
    render::{
        camera::{ActiveCamera, CameraTypePlugin},
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, SlotValue},
        render_phase::RenderPhase,
        renderer::RenderContext,
        RenderApp, RenderStage,
    },
};

/// The name of the final node of the first pass.
pub const FIRST_PASS_DRIVER: &str = "viewport_ref_frame__first_pass_driver";

/// Initializes rendering graph to account for first pass
pub fn init_app_rendering(app: &mut App) {
    app.add_plugin(CameraTypePlugin::<FirstPassCamera>::default());

    let render_app = app.sub_app_mut(RenderApp);
    let driver = FirstPassCameraDriver::new(&mut render_app.world);
    // This will add 3D render phases for the new camera.
    render_app.add_system_to_stage(RenderStage::Extract, extract_first_pass_camera_phases);

    let mut graph = render_app.world.resource_mut::<RenderGraph>();

    // Add a node for the first pass.
    graph.add_node(FIRST_PASS_DRIVER, driver);

    // The first pass's dependencies include those of the main pass.
    graph
        .add_node_edge(node::MAIN_PASS_DEPENDENCIES, FIRST_PASS_DRIVER)
        .unwrap();

    // Insert the first pass node: CLEAR_PASS_DRIVER -> FIRST_PASS_DRIVER -> MAIN_PASS_DRIVER
    graph
        .add_node_edge(node::CLEAR_PASS_DRIVER, FIRST_PASS_DRIVER)
        .unwrap();
    graph
        .add_node_edge(FIRST_PASS_DRIVER, node::MAIN_PASS_DRIVER)
        .unwrap();
}

/// System that adds 3D render phases for FIRST_PASS_CAMERA.
fn extract_first_pass_camera_phases(
    mut commands: Commands,
    active: Res<ActiveCamera<FirstPassCamera>>,
) {
    if let Some(entity) = active.get() {
        commands.get_or_spawn(entity).insert_bundle((
            RenderPhase::<Opaque3d>::default(),
            RenderPhase::<AlphaMask3d>::default(),
            RenderPhase::<Transparent3d>::default(),
        ));
    }
}

/// A node for the first pass camera that runs draw_3d_graph with this camera.
struct FirstPassCameraDriver {
    query: QueryState<Entity, With<FirstPassCamera>>,
}

impl FirstPassCameraDriver {
    pub fn new(render_world: &mut World) -> Self {
        Self {
            query: QueryState::new(render_world),
        }
    }
}

impl Node for FirstPassCameraDriver {
    fn update(&mut self, world: &mut World) {
        self.query.update_archetypes(world);
    }

    fn run(
        &self,
        graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        for camera in self.query.iter_manual(world) {
            graph.run_sub_graph(draw_3d_graph::NAME, vec![SlotValue::Entity(camera)])?;
        }
        Ok(())
    }
}
