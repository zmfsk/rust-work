use crate::game::{GRID_SIZE, GameState, Stone};

/// 检查是否有玩家获胜
pub fn check_victory(game_state: &GameState) -> Option<Stone> {
    for row in 0..GRID_SIZE + 1 {
        for col in 0..GRID_SIZE + 1 {
            if let Some(stone) = game_state.board[row][col] {
                // 检查四个方向：水平、垂直、左斜、右斜
                if check_direction(&game_state.board, row, col, 1, 0, stone) // 水平
                    || check_direction(&game_state.board, row, col, 0, 1, stone) // 垂直
                    || check_direction(&game_state.board, row, col, 1, 1, stone) // 左斜
                    || check_direction(&game_state.board, row, col, 1, -1, stone)
                {
                    // 右斜
                    return Some(stone);
                }
            }
        }
    }
    None
}

/// 检查某个方向是否有连续 5 个相同的棋子
fn check_direction(
    board: &[[Option<Stone>; GRID_SIZE + 1]; GRID_SIZE + 1],
    row: usize,
    col: usize,
    delta_row: isize,
    delta_col: isize,
    stone: Stone,
) -> bool {
    let mut count = 0;

    for i in 0..5 {
        let new_row = row as isize + i * delta_row;
        let new_col = col as isize + i * delta_col;

        if new_row < 0
            || new_row > GRID_SIZE as isize
            || new_col < 0
            || new_col > GRID_SIZE as isize
        {
            return false;
        }

        if board[new_row as usize][new_col as usize] == Some(stone) {
            count += 1;
        } else {
            break;
        }
    }

    count == 5
}
