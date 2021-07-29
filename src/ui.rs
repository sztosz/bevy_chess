use crate::{board::*, pieces::*};
use bevy::prelude::*;

struct NextMoveText;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_next_move_text.system())
            .add_system(next_move_text_update.system());
    }
}

fn init_next_move_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: color_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Next move: White".to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(NextMoveText);
        });
}

fn next_move_text_update(turn: Res<PlayerTurn>, mut query: Query<(&mut Text, &NextMoveText)>) {
    for (mut text, _tag) in query.iter_mut() {
        text.sections[0].value = format!(
            "Next move: {}",
            match turn.color {
                PieceColor::White => "White",
                PieceColor::Black => "Black",
            }
        );
    }
}
