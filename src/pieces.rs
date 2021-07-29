use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]

pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}
impl Piece {
    pub fn is_move_valid(&self, new_position: (u8, u8), pieces: Vec<Piece>) -> bool {
        if color_of_square(new_position, &pieces) == Some(self.color) {
            return false;
        }

        match self.piece_type {
            PieceType::King => {
                (self.x as i8 - new_position.0 as i8).abs() == 1 && (self.y == new_position.1)
                    || (self.y as i8 - new_position.1 as i8).abs() == 1
                        && (self.x == new_position.0)
                    || (self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
            }

            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
                        || ((self.x == new_position.0 && self.y != new_position.1)
                            || (self.y == new_position.1 && self.x != new_position.0)))
            }
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && (self.x as i8 - new_position.0 as i8).abs()
                        == (self.y as i8 - new_position.1 as i8).abs()
            }
            PieceType::Knight => {
                ((self.x as i8 - new_position.0 as i8).abs() == 2
                    && (self.y as i8 - new_position.1 as i8).abs() == 1)
                    || ((self.x as i8 - new_position.0 as i8).abs() == 1
                        && (self.y as i8 - new_position.1 as i8).abs() == 2)
            }
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_position, &pieces)
                    && ((self.x == new_position.0 && self.y != new_position.1)
                        || (self.y == new_position.1 && self.x != new_position.0))
            }
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    if (new_position.0 as i8 - self.x as i8 == 1
                        && (self.y == new_position.1)
                        && color_of_square(new_position, &pieces).is_none())
                        || (self.x == 1
                            && new_position.0 as i8 - self.x as i8 == 2
                            && (self.y == new_position.1)
                            && is_path_empty((self.x, self.y), new_position, &pieces)
                            && color_of_square(new_position, &pieces).is_none())
                        || (new_position.0 as i8 - self.x as i8 == 1
                            && (self.y as i8 - new_position.1 as i8).abs() == 1
                            && color_of_square(new_position, &pieces) == Some(PieceColor::Black))
                    {
                        return true;
                    }
                } else if (new_position.0 as i8 - self.x as i8 == -1
                    && (self.y == new_position.1)
                    && color_of_square(new_position, &pieces).is_none())
                    || (self.x == 6
                        && new_position.0 as i8 - self.x as i8 == -2
                        && (self.y == new_position.1)
                        && is_path_empty((self.x, self.y), new_position, &pieces)
                        && color_of_square(new_position, &pieces).is_none())
                    || (new_position.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_position.1 as i8).abs() == 1
                        && color_of_square(new_position, &pieces) == Some(PieceColor::White))
                {
                    return true;
                }

                false
            }
        }
    }
}
pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_pieces.system())
            .add_system(move_pieces.system());
    }
}

pub fn spawn_king(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::King,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });

            parent.spawn_bundle(PbrBundle {
                mesh: mesh_cross,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}

pub fn spawn_knight(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    bottom_mesh: Handle<Mesh>,
    top_mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Knight,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: bottom_mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });

            parent.spawn_bundle(PbrBundle {
                mesh: top_mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}

pub fn spawn_queen(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Queen,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}

pub fn spawn_bishop(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Bishop,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}

pub fn spawn_rook(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Rook,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}

pub fn spawn_pawn(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) -> Commands {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Pawn,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
    commands
}
fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * (time.delta_seconds() * 3.);
        }
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh2/Primitive0");
    let knight_bottom_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh3/Primitive0");
    let knight_top_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess_kit/pieces.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    commands = spawn_rook(
        commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 0),
    );

    commands = spawn_knight(
        commands,
        white_material.clone(),
        PieceColor::White,
        knight_bottom_handle.clone(),
        knight_top_handle.clone(),
        (0, 1),
    );

    commands = spawn_bishop(
        commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 2),
    );

    commands = spawn_queen(
        commands,
        white_material.clone(),
        PieceColor::White,
        queen_handle.clone(),
        (0, 3),
    );

    commands = spawn_king(
        commands,
        white_material.clone(),
        PieceColor::White,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0, 4),
    );

    commands = spawn_bishop(
        commands,
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 5),
    );

    commands = spawn_knight(
        commands,
        white_material.clone(),
        PieceColor::White,
        knight_bottom_handle.clone(),
        knight_top_handle.clone(),
        (0, 6),
    );

    commands = spawn_rook(
        commands,
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 7),
    );

    for i in 0..8 {
        commands = spawn_pawn(
            commands,
            white_material.clone(),
            PieceColor::White,
            pawn_handle.clone(),
            (1, i),
        );
    }
    commands = spawn_rook(
        commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 0),
    );

    commands = spawn_knight(
        commands,
        black_material.clone(),
        PieceColor::Black,
        knight_bottom_handle.clone(),
        knight_top_handle.clone(),
        (7, 1),
    );

    commands = spawn_bishop(
        commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 2),
    );

    commands = spawn_queen(
        commands,
        black_material.clone(),
        PieceColor::Black,
        queen_handle,
        (7, 3),
    );

    commands = spawn_king(
        commands,
        black_material.clone(),
        PieceColor::Black,
        king_handle,
        king_cross_handle,
        (7, 4),
    );

    commands = spawn_bishop(
        commands,
        black_material.clone(),
        PieceColor::Black,
        bishop_handle,
        (7, 5),
    );

    commands = spawn_knight(
        commands,
        black_material.clone(),
        PieceColor::Black,
        knight_bottom_handle,
        knight_top_handle,
        (7, 6),
    );

    commands = spawn_rook(
        commands,
        black_material.clone(),
        PieceColor::Black,
        rook_handle,
        (7, 7),
    );

    for i in 0..8 {
        commands = spawn_pawn(
            commands,
            black_material.clone(),
            PieceColor::Black,
            pawn_handle.clone(),
            (6, i),
        );
    }
}

fn color_of_square(pos: (u8, u8), pieces: &[Piece]) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &[Piece]) -> bool {
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }
    true
}
