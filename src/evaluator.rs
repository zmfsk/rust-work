use crate::game::{GRID_SIZE, GameState, Stone};

/// 棋盘评估器
pub struct BoardEvaluator;

impl BoardEvaluator {
    /// 评估一个位置的得分
    pub fn evaluate_move(game_state: &GameState, row: usize, col: usize, stone: Stone) -> i32 {
        if row > GRID_SIZE || col > GRID_SIZE || game_state.board[row][col].is_some() {
            return 0; // 无效位置
        }

        // 模拟在该位置落子
        let mut simulated_state = game_state.clone();
        simulated_state.board[row][col] = Some(stone);

        // 计算该位置的得分
        Self::evaluate_position(&simulated_state, row, col, stone)
    }

    /// 评估棋盘上某个位置的得分
    fn evaluate_position(game_state: &GameState, row: usize, col: usize, stone: Stone) -> i32 {
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)]; // 水平、垂直、左斜、右斜
        let mut total_score = 0;

        // 评估自己的棋型得分
        for (delta_row, delta_col) in directions.iter() {
            total_score += Self::evaluate_direction(
                &game_state.board,
                row,
                col,
                *delta_row,
                *delta_col,
                stone,
            );
        }

        // 评估对对方棋型的影响（防守得分）
        let opponent_stone = match stone {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            _ => return total_score, // 如果是空，直接返回
        };

        // 创建一个临时状态，模拟对手在此位置落子
        let mut opponent_state = GameState {
            board: [[None; GRID_SIZE + 1]; GRID_SIZE + 1],
            current_turn: opponent_stone,
            is_game_over: false,
            winner: None
        };

        // 复制当前棋盘状态
        for r in 0..=GRID_SIZE {
            for c in 0..=GRID_SIZE {
                opponent_state.board[r][c] = game_state.board[r][c];
            }
        }

        // 假设对手在此位置落子
        opponent_state.board[row][col] = Some(opponent_stone);

        // 计算对手在此位置的得分
        let mut opponent_score = 0;
        for (delta_row, delta_col) in directions.iter() {
            opponent_score += Self::evaluate_direction(
                &opponent_state.board,
                row,
                col,
                *delta_row,
                *delta_col,
                opponent_stone,
            );
        }

        
        
            total_score += opponent_score ;
       

        total_score
    }

    /// 评估某个方向的得分
    fn evaluate_direction(
        board: &[[Option<Stone>; GRID_SIZE + 1]; GRID_SIZE + 1],
        row: usize,
        col: usize,
        delta_row: isize,
        delta_col: isize,
        stone: Stone,
    ) -> i32 {
        // 计算连续的棋子数量和两端的开放情况
        let mut count = 1; // 当前位置已有一个棋子
        let mut left_open = false;
        let mut right_open = false;

        // 向左检查
        let mut left_count = 0;
        for i in 1..5 {
            let new_row = row as isize - i * delta_row;
            let new_col = col as isize - i * delta_col;

            if new_row < 0 || new_row > GRID_SIZE as isize || new_col < 0 || new_col > GRID_SIZE as isize {
                break;
            }

            if board[new_row as usize][new_col as usize] == Some(stone) {
                left_count += 1;
                count += 1;
            } else if board[new_row as usize][new_col as usize].is_none() {
                left_open = true;
                break;
            } else {
                break;
            }
        }

        // 向右检查
        let mut right_count = 0;
        for i in 1..5 {
            let new_row = row as isize + i * delta_row;
            let new_col = col as isize + i * delta_col;

            if new_row < 0 || new_row > GRID_SIZE as isize || new_col < 0 || new_col > GRID_SIZE as isize {
                break;
            }

            if board[new_row as usize][new_col as usize] == Some(stone) {
                right_count += 1;
                count += 1;
            } else if board[new_row as usize][new_col as usize].is_none() {
                right_open = true;
                break;
            } else {
                break;
            }
        }

        // 根据棋型返回得分
        match count {
            5 => 100000, // 五连珠，胜利
            4 => {
                if left_open && right_open {
                    10000 // 活四
                } else if left_open || right_open {
                    1000 // 冲四
                } else {
                    0
                }
            },
            3 => {
                if left_open && right_open {
                    1500 // 活三
                } else if left_open || right_open {
                    100 // 眠三
                } else {
                    0
                }
            },
            2 => {
                if left_open && right_open {
                    100 // 活二
                } else if left_open || right_open {
                    10 // 眠二
                } else {
                    0
                }
            },
            _ => 0,
        }
    }

    /// 找到棋盘上最佳落子位置和得分
    pub fn find_best_move(game_state: &GameState, stone: Stone) -> Option<((usize, usize), i32)> {
        let mut best_move = None;
        let mut best_score = -1;

        // 遍历所有可能的落子位置
        for row in 0..=GRID_SIZE {
            for col in 0..=GRID_SIZE {
                if game_state.board[row][col].is_none() {
                    let score = Self::evaluate_move(game_state, row, col, stone);
                    if score > best_score {
                        best_score = score;
                        best_move = Some(((row, col), score));
                    }
                }
            }
        }

        best_move
    }
}