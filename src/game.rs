use bevy::prelude::*;

pub const GRID_SIZE: usize = 15; // 棋盘大小
pub const CELL_SIZE: f32 = 40.0; // 每个单元格的大小

#[derive(Resource)]
pub struct GameState {
    pub board: [[Option<Stone>; GRID_SIZE + 1]; GRID_SIZE + 1], // 棋盘
    pub current_turn: Stone,                                    // 当前轮到谁下
    pub is_game_over: bool,                                     // 游戏是否结束
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[None; GRID_SIZE + 1]; GRID_SIZE + 1],
            current_turn: Stone::Black,
            is_game_over: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Black,
    White,
}
