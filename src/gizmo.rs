#[macro_export]
macro_rules! gizmo {
    ($gizmo_name:ident($meshes:ident, $materials:ident):$($bundle:expr$(; $cpt:expr)*),+$(,)?) => {
        #[allow(unused_variables, unused_mut, unused_parens)]
        fn $gizmo_name(
            layers: bevy::render::view::RenderLayers,
            commands: &mut bevy::prelude::Commands,
            mut $meshes: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::Mesh>>,
            mut $materials: bevy::prelude::ResMut<bevy::prelude::Assets<bevy::prelude::StandardMaterial>>,
        ) {$(
            commands
                .spawn_bundle($bundle)
              $(.insert($cpt))*
                .insert(layers);
        )*}
    };
}
