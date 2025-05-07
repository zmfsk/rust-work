mod agent;
mod board;
mod game;
mod game_manager;
mod input;
mod ui;

use agent::SmartAgent;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::{ResetButton, SwitchButton, SwitchButtonText, setup_board};
use game::{CELL_SIZE, GRID_SIZE, GameState, Stone, StoneComponent};
use game_manager::check_victory;
use input::place_stone;
use ui::{
    AppState, CloseButton, PlayAgainButton, StartButton, UsageButton, UsageWindow,
    VictoryCloseButton, VictoryWindow, cleanup_main_menu, handle_close_button,
    handle_play_again_button, handle_start_button, handle_usage_button,
    handle_victory_close_button, setup_main_menu, show_victory_window,
}; // 导入UI组件和系统

const BOARD_OFFSET: f32 = -200.0;
const AI_DIFFICULTY: u32 = 4; // 最高支持=4但有明显卡顿

// 修改导入部分
// 在 main 函数中添加系统
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.8, 0.6)))
        .insert_resource(GameState::new())
        .insert_resource(SmartAgent::new(Stone::White, AI_DIFFICULTY)) // 默认AI使用白子
        .add_state::<AppState>() // 添加应用状态
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "五子棋".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        // 主菜单系统
        .add_systems(Startup, setup_main_menu) // 让UI模块管理相机
        .add_systems(
            Update,
            handle_start_button.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            handle_usage_button.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            handle_close_button.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        // 游戏系统
        .add_systems(OnEnter(AppState::InGame), setup_board)
        .add_systems(
            Update,
            handle_buttons
                .before(place_stone)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, place_stone.run_if(in_state(AppState::InGame)))
        .add_systems(
            Update,
            check_victory_system
                .after(place_stone)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            ai_move
                .after(check_victory_system)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            update_switch_button_text
                .after(check_victory_system)
                .run_if(in_state(AppState::InGame)),
        )
        // 游戏系统部分添加胜利窗口相关系统
        .add_systems(
            Update,
            show_victory_window
                .after(check_victory_system)
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            handle_victory_close_button.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            handle_play_again_button.run_if(in_state(AppState::InGame)),
        )
        .run();
}

/// 系统：检查胜负
fn check_victory_system(mut game_state: ResMut<GameState>) {
    // Only check if game is not already over
    if game_state.is_game_over {
        return;
    }

    if let Some(winner) = check_victory(&game_state) {
        println!("Game Over! Winner: {:?}", winner);
        game_state.is_game_over = true;
        game_state.winner = Some(winner); // Store the winner
    } else {
        // Check for draw (board full, no winner)
        if game_state.get_valid_moves().is_empty() {
            println!("Game Over! It's a Draw!");
            game_state.is_game_over = true;
            game_state.winner = None; // Explicitly mark no winner for draw
        }
    }
}

/// 系统：AI落子
fn ai_move(mut commands: Commands, mut game_state: ResMut<GameState>, ai: Res<SmartAgent>) {
    if game_state.is_game_over {
        return;
    }

    // sleep(Duration::from_secs(1));

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
    ai: Res<SmartAgent>,
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
    mut ai: ResMut<SmartAgent>, // Now mutable to change settings
    camera_query: Query<(&Camera, &GlobalTransform)>,
    reset_button_query: Query<(&ResetButton, &GlobalTransform)>,
    switch_button_query: Query<(&SwitchButton, &GlobalTransform)>,
    stone_query: Query<Entity, With<StoneComponent>>,
    // Example: Add UI elements later to change ai.set_depth(new_depth)
) {
    let window = windows.single();

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some((camera, camera_transform)) = camera_query.get_single().ok() {
                if let Some(world_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    // Check reset button click
                    for (_, transform) in reset_button_query.iter() {
                        let button_pos = transform.translation();
                        // Adjust button size check if needed
                        let button_rect = Rect::new(
                            button_pos.x - 75.0, // half width
                            button_pos.y - 30.0, // half height
                            button_pos.x + 75.0,
                            button_pos.y + 30.0,
                        );

                        if button_rect.contains(world_position) {
                            // Reset game state
                            game_state.reset();
                            // Clear all stones
                            for entity in stone_query.iter() {
                                commands.entity(entity).despawn_recursive(); // Use despawn_recursive
                            }
                            println!("Game Reset!"); // Feedback
                            return; // Processed button click
                        }
                    }

                    // Check switch button click
                    for (_, transform) in switch_button_query.iter() {
                        let button_pos = transform.translation();
                        let button_rect = Rect::new(
                            button_pos.x - 75.0,
                            button_pos.y - 30.0,
                            button_pos.x + 75.0,
                            button_pos.y + 30.0,
                        );

                        if button_rect.contains(world_position) {
                            // Switch AI's stone color
                            let current_stone = ai.get_stone();
                            ai.set_stone(current_stone.opponent()); // Use opponent() helper

                            // Reset game state
                            game_state.reset();

                            // Clear all stones
                            for entity in stone_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                            println!("AI switched to {:?}", ai.get_stone()); // Feedback
                            return; // Processed button click
                        }
                    }
                }
            }
        }
    }
}
