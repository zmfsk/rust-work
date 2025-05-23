use crate::agent::SmartAgent;
use crate::game::{CELL_SIZE, GRID_SIZE, GameState, PlayerScore, Stone, StoneComponent};
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
    mut player_score: ResMut<PlayerScore>,
    ai: ResMut<SmartAgent>,
) {
    let window = windows.single();

    if game_state.is_game_over {
        return;
    }

    if game_state.current_turn == ai.get_stone() {
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
                        let player_stone = game_state.current_turn;

                        // --- 玩家评分计算 (在应用落子之前) ---
                        let game_state_before = game_state.clone();
                        // 为玩家创建一个临时 AI 代理以进行评估
                        let player_agent = SmartAgent::new(player_stone, ai.get_difficulty());

                        if let Some((best_move, best_score)) =
                            player_agent.find_best_move_and_score(&game_state_before)
                        {
                            if let Some(player_move_score) =
                                player_agent.get_score_for_move(&game_state_before, (row, col))
                            {
                                let loss = (best_score - player_move_score).max(0); // 损失值不能为负
                                player_score.add_move(loss);
                            } else {
                                println!("警告: 无法评估玩家落子分数。");
                                player_score.add_move(10000); // 如果评估失败，是否给予惩罚？
                            }
                        } else {
                            println!("警告: 找不到最佳落子/分数。也许没有可落子的地方了？");
                            // 如果没有可行的移动，则不增加损失
                        }

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
                        game_state.board[row][col] = Some(player_stone); // 使用 player_stone
                        game_state.current_turn = player_stone.opponent(); // 切换回合
                    }
                }
            }
        }
    }
}
