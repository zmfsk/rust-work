use crate::game::{CELL_SIZE, GRID_SIZE, PlayerScore};
use bevy::prelude::*;

pub fn setup_board(mut commands: Commands, player_score: Res<PlayerScore>) {
    commands.spawn(Camera2dBundle::default()); // 确保摄像机存在

    // 将棋盘向左移动
    let board_offset = -200.0; // 向左偏移200像素
    let grid_color = Color::rgb(0.3, 0.3, 0.3);
    let half_grid_size = (GRID_SIZE as f32) / 2.0;

    for i in 0..=GRID_SIZE {
        let offset = (i as f32 - half_grid_size) * CELL_SIZE;

        // 垂直线
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(2.0, GRID_SIZE as f32 * CELL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(offset + board_offset, 0.0, 0.0),
            ..default()
        });

        // 水平线
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(GRID_SIZE as f32 * CELL_SIZE, 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(board_offset, offset, 0.0),
            ..default()
        });
    }

    // 计算按钮位置
    let button_x = board_offset + (GRID_SIZE as f32 * CELL_SIZE) / 2.0 + 200.0;
    let score_y = 350.0; // 得分显示位置
    let reset_button_y = 250.0;
    let switch_button_y = 150.0;
    
    // 添加玩家得分显示
    commands.spawn((Text2dBundle {
        text: Text::from_section(
            format!("Player Rating: {}", player_score.current_rating),
            TextStyle {
                font_size: 24.0,
                color: Color::rgb(0.2, 0.2, 0.2),
                ..default()
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform::from_xyz(button_x, score_y, 2.0),
        ..default()
    }, ScoreText));


    // 添加重置按钮
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(200.0, 60.0)),
                ..default()
            },
            transform: Transform::from_xyz(button_x, reset_button_y, 1.0),
            ..default()
        },
        ResetButton,
    ));

    // 添加重置按钮文字
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Reset Game",
            TextStyle {
                font_size: 24.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform::from_xyz(button_x, reset_button_y, 2.0),
        ..default()
    });

    // 添加切换按钮
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(200.0, 60.0)),
                ..default()
            },
            transform: Transform::from_xyz(button_x, switch_button_y, 1.0),
            ..default()
        },
        SwitchButton,
    ));

    // 添加切换按钮文字
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "AI: White\nClick to Switch",
                TextStyle {
                    font_size: 24.0,
                    color: Color::BLACK,
                    ..default()
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform::from_xyz(button_x, switch_button_y, 2.0),
            ..default()
        },
        SwitchButtonText,
    ));
}

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct SwitchButton;

#[derive(Component)]
pub struct SwitchButtonText;

#[derive(Component)]
pub struct ScoreText;
