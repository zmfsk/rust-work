mod board;
mod game;
mod game_manager;
mod input;
mod agent;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::{setup_board, ResetButton, SwitchButton, SwitchButtonText};
use game::{GameState, Stone, CELL_SIZE, GRID_SIZE, StoneComponent};
use game_manager::check_victory;
use input::place_stone;
use agent::RandomAgent;

const BOARD_OFFSET: f32 = -200.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.8, 0.6)))
        .insert_resource(GameState::new())
        .insert_resource(RandomAgent::new(Stone::White)) // 默认AI使用白子
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "五子棋".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_board)
        .add_systems(Update, place_stone.after(check_victory_system))
        .add_systems(Update, check_victory_system)
        .add_systems(Update, ai_move.after(place_stone))
        .add_systems(Update, handle_buttons)
        .add_systems(Update, update_switch_button_text)
        .run();
}

/// 系统：检查胜负
fn check_victory_system(mut game_state: ResMut<GameState>) {
    if let Some(winner) = check_victory(&game_state) {
        println!("游戏结束！获胜者是: {:?}", winner);
        game_state.is_game_over = true;
    }
}

/// 系统：AI落子
fn ai_move(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    ai: Res<RandomAgent>,
) {
    if game_state.is_game_over {
        return;
    }

    // 只在AI回合且游戏未结束时执行
    if game_state.current_turn == ai.get_stone() {
        if let Some((row, col)) = ai.make_move(&game_state) {
            let stone_x = (col as f32 - (GRID_SIZE as f32) / 2.0) * CELL_SIZE + BOARD_OFFSET;
            let stone_y = (row as f32 - (GRID_SIZE as f32) / 2.0) * CELL_SIZE;

            let color = match ai.get_stone() {
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

            game_state.board[row][col] = Some(ai.get_stone());
            game_state.current_turn = match ai.get_stone() {
                Stone::Black => Stone::White,
                Stone::White => Stone::Black,
            };
        }
    }
}

/// 系统：更新切换按钮文字
fn update_switch_button_text(
    mut text_query: Query<&mut Text, With<SwitchButtonText>>,
    ai: Res<RandomAgent>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!(
            "AI: {}\nClick to Switch",
            match ai.get_stone() {
                Stone::Black => "Black",
                Stone::White => "White",
            }
        );
    }
}

/// 系统：处理按钮点击
fn handle_buttons(
    mut commands: Commands,
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut game_state: ResMut<GameState>,
    mut ai: ResMut<RandomAgent>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    reset_button_query: Query<(&ResetButton, &GlobalTransform)>,
    switch_button_query: Query<(&SwitchButton, &GlobalTransform)>,
    stone_query: Query<Entity, With<StoneComponent>>,
) {
    let window = windows.single();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some((camera, camera_transform)) = camera_query.get_single().ok() {
                if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    // 检查重置按钮点击
                    for (_, transform) in reset_button_query.iter() {
                        let button_pos = transform.translation();
                        let button_rect = Rect::new(
                            button_pos.x - 75.0,
                            button_pos.y - 30.0,
                            button_pos.x + 75.0,
                            button_pos.y + 30.0,
                        );
                        
                        if button_rect.contains(world_position) {
                            // 重置游戏状态
                            game_state.reset();
                            // 清除所有棋子
                            for entity in stone_query.iter() {
                                commands.entity(entity).despawn();
                            }
                            return;
                        }
                    }

                    // 检查切换按钮点击
                    for (_, transform) in switch_button_query.iter() {
                        let button_pos = transform.translation();
                        let button_rect = Rect::new(
                            button_pos.x - 75.0,
                            button_pos.y - 30.0,
                            button_pos.x + 75.0,
                            button_pos.y + 30.0,
                        );
                        
                        if button_rect.contains(world_position) {
                            // 切换AI的棋子颜色
                            let current_stone = ai.get_stone();
                            let new_stone = match current_stone {
                                Stone::Black => Stone::White,
                                Stone::White => Stone::Black,
                            };
                            ai.set_stone(new_stone);
                            
                            // 重置游戏状态
                            game_state.reset();
                            
                            // 清除所有棋子
                            for entity in stone_query.iter() {
                                commands.entity(entity).despawn();
                            }
                            
                            return;
                        }
                    }
                }
            }
        }
    }
}
