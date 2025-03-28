use crate::game::{CELL_SIZE, GRID_SIZE, GameState, Stone, StoneComponent};
use crate::game_manager::check_victory;
use crate::board::ResetButton;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const BOARD_OFFSET: f32 = -200.0; // 棋盘向左偏移的距离

pub fn place_stone(
    mut commands: Commands,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    reset_button_query: Query<(&ResetButton, &GlobalTransform)>,
    stone_query: Query<Entity, With<StoneComponent>>,
) {
    let window = windows.single();

    // 检查是否点击了重置按钮
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some((camera, camera_transform)) = camera_query.get_single().ok() {
                if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    for (_, transform) in reset_button_query.iter() {
                        let button_pos = transform.translation();
                        let button_rect = Rect::new(
                            button_pos.x - 50.0,
                            button_pos.y - 20.0,
                            button_pos.x + 50.0,
                            button_pos.y + 20.0,
                        );
                        
                        if button_rect.contains(world_position) {
                            // 重置游戏状态
                            game_state.reset();
                            // 清除所有棋子
                            for entity in stone_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                            return;
                        }
                    }
                }
            }
        }
    }

    if game_state.is_game_over {
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
                    let col = ((world_position.x - BOARD_OFFSET + (GRID_SIZE as f32 * CELL_SIZE) / 2.0)
                        / CELL_SIZE)
                        .round() as usize;

                    if row <= GRID_SIZE && col <= GRID_SIZE && game_state.board[row][col].is_none()
                    {
                        println!("row: {}, col: {}", row, col);
                        let stone_x = (col as f32 - (GRID_SIZE as f32) / 2.0) * CELL_SIZE + BOARD_OFFSET;
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

                        // 检查胜负
                        if let Some(winner) = check_victory(&game_state) {
                            println!("游戏结束！获胜者是: {:?}", winner);
                        }

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
