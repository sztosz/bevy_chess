use crate::pieces::*;
use bevy::{app::AppExit, prelude::*};
use bevy_mod_picking::*;

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct PlayerTurn {
    pub color: PieceColor,
}
impl Default for PlayerTurn {
    fn default() -> Self {
        Self {
            color: PieceColor::White,
        }
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    // let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    // let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());
    for x in 0..8 {
        for y in 0..8 {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (x + y + 1) % 2 == 0 {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, 0., y as f32)),
                    ..Default::default()
                })
                .insert(PickableMesh::default())
                .insert(Square { x, y });
        }
    }
}

fn color_squares(
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
    picking_camera_query: Query<&PickingCamera>,
) {
    let top_entity = match picking_camera_query.iter().last() {
        Some(picking_camera) => picking_camera
            .intersect_top()
            .map(|(entity, _intersection)| entity),
        None => None,
    };

    for (entity, square, material_handle) in query.iter() {
        let material = materials.get_mut(material_handle).unwrap();

        material.base_color = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn select_square(
    mut commands: Commands,
    picking_camera_query: Query<&PickingCamera>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    mut app_exit_events: EventWriter<AppExit>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece, &Children)>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(picking_camera) = picking_camera_query.iter().last() {
        if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(square) = squares_query.get(square_entity) {
                selected_square.entity = Some(square_entity);

                if let Some(selected_piece_entity) = selected_piece.entity {
                    let pieces_entity_vec: Vec<(Entity, Piece, Vec<Entity>)> = pieces_query
                        .iter_mut()
                        .map(|(entity, piece, children)| {
                            (entity, *piece, children.iter().copied().collect())
                        })
                        .collect();

                    let pieces_vec = pieces_query
                        .iter_mut()
                        .map(|(_, piece, _)| *piece)
                        .collect();

                    if let Ok((_piece_entity, mut piece, _)) =
                        pieces_query.get_mut(selected_piece_entity)
                    {
                        if piece.is_move_valid((square.x, square.y), pieces_vec) {
                            for (other_entity, other_piece, other_children) in
                                pieces_entity_vec.iter()
                            {
                                if other_piece.x == square.x
                                    && other_piece.y == square.y
                                    && other_piece.color != piece.color
                                {
                                    if other_piece.piece_type == PieceType::King {
                                        println!(
                                            "{} won.",
                                            match turn.color {
                                                PieceColor::White => "Black",
                                                PieceColor::Black => "White",
                                            }
                                        );
                                        app_exit_events.send(AppExit);
                                    }
                                    commands.entity(*other_entity).despawn();
                                    for child in other_children.iter() {
                                        commands.entity(*child).despawn();
                                    }
                                }
                            }
                            piece.x = square.x;
                            piece.y = square.y;

                            turn.color = match turn.color {
                                PieceColor::White => PieceColor::Black,
                                PieceColor::Black => PieceColor::White,
                            }
                        }
                    }
                    selected_piece.entity = None;
                } else {
                    for (piece_entity, piece, _) in pieces_query.iter_mut() {
                        if piece.x == square.x && piece.y == square.y && piece.color == turn.color {
                            selected_piece.entity = Some(piece_entity);
                            break;
                        }
                    }
                }
            }
        } else {
            selected_square.entity = None;
            selected_piece.entity = None;
        }
    }

    selected_square.entity = match picking_camera_query.iter().last() {
        Some(picking_camera) => picking_camera
            .intersect_top()
            .map(|(entity, _intersection)| entity),
        None => None,
    };
}
