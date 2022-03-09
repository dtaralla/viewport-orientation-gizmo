/// Heavily inspired from the [render_to_texture][1] example
/// [1]: https://github.com/bevyengine/bevy/blob/main/examples/3d/render_to_texture.rs
use super::first_pass::*;
use crate::default_gizmo;
use bevy::{
    core_pipeline::RenderTargetClearColors,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera, RenderTarget},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};

/// Attach this component to an entity featuring a [Transform] which you want to track the rotation.
#[derive(Component)]
pub struct TrackedRotator;

/// Represents a position on the canvas. You can use one of the four presets, or specify your custom
/// [Rect] (relative to the screen).
#[derive(Clone)]
pub enum CanvasLocation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    Custom(Rect<Val>),
}

pub(crate) type Gizmo =
    fn(RenderLayers, &mut Commands, ResMut<Assets<Mesh>>, ResMut<Assets<StandardMaterial>>);

/// Options that enable plugin behavior customization.
/// Defaults to 64x64, [BottomLeft].
#[derive(Clone)]
pub struct PluginOptions {
    pub size: u32,
    pub location: CanvasLocation,
    pub gizmo: Gizmo,
}

impl Default for PluginOptions {
    fn default() -> Self {
        Self {
            size: 64,
            location: CanvasLocation::BottomLeft,
            gizmo: default_gizmo::GIZMO,
        }
    }
}

/// A plugin for displaying the reference frame orientation in the bottom left corner of the screen
/// that updates when the TrackedRotator rotates.
pub struct ViewportOrientationGizmoPlugin {
    options: PluginOptions,
}

impl ViewportOrientationGizmoPlugin {
    /// Displays a 64x64 gizmo in the bottom left corner of the screen.
    pub fn new() -> Self {
        Self::new_sized(64)
    }

    /// Displays a gizmo with canvas size `size` in the bottom left corner of the screen.
    pub fn new_sized(size: u32) -> Self {
        Self::custom(PluginOptions { size, ..default() })
    }

    /// Displays a customized gizmo.
    pub fn custom(options: PluginOptions) -> Self {
        Self { options }
    }
}

impl Default for ViewportOrientationGizmoPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ViewportOrientationGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.options.clone())
            .add_startup_system(setup)
            .add_system(update_1st_pass_camera_transform);
        init_app_rendering(app);
    }
}

/// Helper to quickly identify the first pass camera spawned in setup(...)
#[derive(Component)]
struct FirstPassCamera;

/// This handle will point at the texture to which we will render in the first pass.
const RENDER_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 13378939762009864042);

/// Update the virtual camera transform
fn update_1st_pass_camera_transform(
    tracked_rotator: Query<&Transform, (With<TrackedRotator>, Without<FirstPassCamera>)>,
    mut first_pass_cam: Query<&mut Transform, With<FirstPassCamera>>,
) {
    if let Some(tracked_transform) = tracked_rotator.iter().next() {
        let mut cam_transform = first_pass_cam.single_mut();
        cam_transform.rotation = tracked_transform.rotation;
    }
}

/// Setup virtual camera, gizmo mesh and plugin canvas
fn setup(
    plugin_options: Res<PluginOptions>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut active_cameras: ResMut<ActiveCameras>,
    mut images: ResMut<Assets<Image>>,
    mut clear_colors: ResMut<RenderTargetClearColors>,
) {
    let size = Extent3d {
        width: plugin_options.size,
        height: plugin_options.size,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.set(RENDER_IMAGE_HANDLE, image);

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and coordinate system.
    let first_pass_layer = RenderLayers::layer(1);

    // What will be rendered to the texture
    (plugin_options.gizmo)(first_pass_layer, &mut commands, meshes, materials);

    // First pass camera capturing what will be rendered to the texture
    let render_target = RenderTarget::Image(image_handle);
    clear_colors.insert(render_target.clone(), Color::rgba(0.0, 0.0, 0.0, 0.0));
    active_cameras.add(FIRST_PASS_CAMERA);
    commands
        .spawn()
        .insert(Transform::identity().looking_at(Vec3::new(0.0, 0.0, -3.0), Vec3::Y))
        .insert(GlobalTransform::identity())
        .insert(FirstPassCamera)
        .with_children(|parent| {
            parent
                .spawn_bundle(PerspectiveCameraBundle {
                    camera: Camera {
                        name: Some(FIRST_PASS_CAMERA.to_string()),
                        target: render_target,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
                    ..default()
                })
                .insert(first_pass_layer);
        });

    // Create UI Camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Display RenderTexture in bottom left corner of UI canvas
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(
                Val::Px(plugin_options.size as f32),
                Val::Px(plugin_options.size as f32),
            ),
            position_type: PositionType::Absolute,
            position: match plugin_options.location {
                CanvasLocation::TopLeft => Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                CanvasLocation::TopRight => Rect {
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                CanvasLocation::BottomLeft => Rect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                CanvasLocation::BottomRight => Rect {
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..default()
                },
                CanvasLocation::Custom(r) => r,
            },
            ..default()
        },
        image: UiImage::from(RENDER_IMAGE_HANDLE.typed()),
        ..default()
    });
}
