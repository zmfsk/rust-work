use crate::game::{CELL_SIZE, GRID_SIZE, GameState, Stone, StoneComponent};
use crate::game_manager::check_victory;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BOARD_OFFSET: f32 = -200.0; // 棋盘向左偏移的距离

pub fn place_stone(
    mut commands: Commands,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ui_interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    let window = windows.single();

    if game_state.is_game_over {
        return;
    }

    // 检查是否有UI按钮正在被交互，如果有则不处理落子
    let ui_clicked = ui_interaction_query
        .iter()
        .any(|interaction| *interaction == Interaction::Pressed);

    if ui_clicked {
        return;
    }

    if let Some((camera, camera_transform)) = camera_query.get_single().ok() {
        if buttons.just_pressed(MouseButton::Left) {
            if let Some(cursor_position) = window.cursor_position() {
                if let Some(world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    // 考虑棋盘偏移计算行列
                    let row = ((world_position.y + (GRID_SIZE as f32 * CELL_SIZE) / 2.0)
                        / CELL_SIZE)
                        .round() as usize;
                    let col = ((world_position.x - BOARD_OFFSET
                        + (GRID_SIZE as f32 * CELL_SIZE) / 2.0)
                        / CELL_SIZE)
                        .round() as usize;

                    if row <= GRID_SIZE && col <= GRID_SIZE && game_state.board[row][col].is_none()
                    {
                        println!("row: {}, col: {}", row, col);
                        let stone_x =
                            (col as f32 - (GRID_SIZE as f32) / 2.0) * CELL_SIZE + BOARD_OFFSET;
                        let stone_y = (row as f32 - (GRID_SIZE as f32) / 2.0) * CELL_SIZE;

                        let color = match game_state.current_turn {
                            Stone::Black => Color::BLACK,
                            Stone::White => Color::WHITE,
                        };

                        commands.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shapes::Circle {
                                    radius: CELL_SIZE * 0.4,
                                    center: Vec2::ZERO,
                                }),
                                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                                    stone_x, stone_y, 2.0,
                                )),
                                ..default()
                            },
                            Fill::color(color),
                            StoneComponent,
                        ));

                        game_state.board[row][col] = Some(game_state.current_turn);

                        game_state.current_turn = match game_state.current_turn {
                            Stone::Black => Stone::White,
                            Stone::White => Stone::Black,
                        };
                    }
                }
            }
        }
    }
}
