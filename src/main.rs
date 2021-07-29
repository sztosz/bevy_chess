use bevy::prelude::*;
use bevy_mod_picking::*;

mod board;
mod pieces;
mod ui;
use board::*;
use pieces::*;
use ui::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<PickingCamera>()
        .add_plugin(PickingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7., 20., 4.),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .commands()
        .spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
            ..Default::default()
        });
}
