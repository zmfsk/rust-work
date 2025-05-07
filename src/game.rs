use bevy::prelude::*;

pub const GRID_SIZE: usize = 14; // 棋盘大小
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

    /// 在棋盘上应用一个落子 (in-place 修改)。
    /// 如果位置越界或已被占据，返回 Err。
    pub fn apply_move(&mut self, r: usize, c: usize, stone: Stone) -> Result<(), &'static str> {
        if r > GRID_SIZE || c > GRID_SIZE {
            return Err("落子位置越界");
        }
        if self.board[r][c].is_some() {
            return Err("位置已被占据");
        }
        self.board[r][c] = Some(stone);
        Ok(())
    }

    /// 撤销在 (r, c) 位置的落子。期望该位置有棋子。
    pub fn undo_move(&mut self, r: usize, c: usize) {
        // 在实际使用中，您可能需要更严格的检查或 panic。
        // 在 Minimax 悔棋的场景，我们期望该位置是我们刚刚放下的棋子。
        if r > GRID_SIZE || c > GRID_SIZE {
            eprintln!("尝试悔棋的位置越界： ({}, {})", r, c);
            return; // 或者 panic!
        }
        if self.board[r][c].is_none() {
            eprintln!("尝试悔棋的位置是空的： ({}, {})", r, c);
            // 这可能意味着 apply_move 失败了，或者 undo_move 被错误调用
            return; // 或者 panic!
        }
        self.board[r][c] = None; // 将位置设为空
    }

    // make_move_simulated 仍然可以在 make_move 的顶层使用，或者也可以用 apply/undo 替换
    // 如果保留 make_move_simulated，GameState 需要实现 Clone
    pub fn make_move_simulated(&self, r: usize, c: usize, stone: Stone) -> Option<GameState> {
        if r > GRID_SIZE || c > GRID_SIZE || self.board[r][c].is_some() {
            return None; // 无效移动
        }
        let mut next_state = self.clone(); // 克隆当前状态
        next_state.board[r][c] = Some(stone);
        // 如果需要在模拟状态中体现回合切换，可以在这里加上
        // next_state.current_turn = stone.opponent();
        Some(next_state)
    }

    // 获取所有有效移动 (棋盘上的所有空位)
    pub fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let board_size = GRID_SIZE + 1;
        for r in 0..board_size {
            for c in 0..board_size {
                if self.board[r][c].is_none() {
                    moves.push((r, c));
                }
            }
        }
        moves
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
