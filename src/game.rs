use bevy::prelude::*;

pub const GRID_SIZE: usize = 15; // 棋盘大小
pub const CELL_SIZE: f32 = 40.0; // 每个单元格的大小

#[derive(Resource, Clone)]
pub struct GameState {
    pub board: [[Option<Stone>; GRID_SIZE + 1]; GRID_SIZE + 1], // 棋盘
    pub current_turn: Stone,                                    // 当前轮到谁下
    pub is_game_over: bool,                                     // 游戏是否结束
    pub winner: Option<Stone>,                                  // 胜利者
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: [[None; GRID_SIZE + 1]; GRID_SIZE + 1],
            current_turn: Stone::Black,
            is_game_over: false,
            winner: None,
        }
    }

    pub fn reset(&mut self) {
        self.board = [[None; GRID_SIZE + 1]; GRID_SIZE + 1];
        self.current_turn = Stone::Black;
        self.is_game_over = false;
    }

    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        let mut valid_moves = Vec::new();
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if self.board[row][col].is_none() {
                    valid_moves.push((row, col));
                }
            }
        }
        valid_moves
    }

    pub fn make_move_simulated(&self, row: usize, col: usize, stone: Stone) -> Option<GameState> {
        if row >= GRID_SIZE || col >= GRID_SIZE || self.board[row][col].is_some() {
            return None; // Invalid move
        }
        let mut next_state = self.clone();
        next_state.board[row][col] = Some(stone);
        next_state.current_turn = match stone {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        };
        Some(next_state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn opponent(&self) -> Stone {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

#[derive(Component)]
pub struct StoneComponent;
