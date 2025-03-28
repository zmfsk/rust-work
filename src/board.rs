use crate::game::{CELL_SIZE, GRID_SIZE};
use bevy::prelude::*;

pub fn setup_board(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()); // 确保摄像机存在

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
            transform: Transform::from_xyz(offset, 0.0, 0.0),
            ..default()
        });

        // 水平线
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: grid_color,
                custom_size: Some(Vec2::new(GRID_SIZE as f32 * CELL_SIZE, 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, offset, 0.0),
            ..default()
        });
    }
}
