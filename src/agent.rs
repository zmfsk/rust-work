use crate::game::{GRID_SIZE, GameState, Stone};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SmartAgent {
    stone: Stone,
}

impl SmartAgent {
    pub fn new(stone: Stone) -> Self {
        SmartAgent { stone }
    }

    pub fn make_move(&self, game_state: &GameState) -> Option<(usize, usize)> {
        let mut best_move = None;
        let mut best_score = -1;

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if game_state.board[row][col].is_none() {
                    let score = self.evaluate_position(game_state, row, col);
                    if score > best_score {
                        best_score = score;
                        best_move = Some((row, col));
                    }
                }
            }
        }

        best_move
    }

    fn evaluate_position(&self, game_state: &GameState, row: usize, col: usize) -> i32 {
        let mut score = 0;

        // 定义方向：横、竖、左斜、右斜
        let directions = [(1, 0), (0, 1), (1, 1), (1, -1)];

        for &(dx, dy) in &directions {
            let mut my_count = 1; // 当前落子
            let mut opp_count = 0;
            let mut open_ends = 0;

            // 统计当前方向的己方和对方棋子数量，同时判断是否为活棋
            let mut blocked = (false, false);

            for i in 1..=4 {
                let nx = row as isize + i * dx;
                let ny = col as isize + i * dy;
                if nx >= 0 && ny >= 0 && nx < GRID_SIZE as isize && ny < GRID_SIZE as isize {
                    match game_state.board[nx as usize][ny as usize] {
                        Some(stone) if stone == self.stone => my_count += 1,
                        Some(_) => {
                            blocked.0 = true;
                            break;
                        }
                        None => {
                            open_ends += 1;
                            break;
                        }
                    }
                } else {
                    blocked.0 = true;
                }
            }

            for i in 1..=4 {
                let nx = row as isize - i * dx;
                let ny = col as isize - i * dy;
                if nx >= 0 && ny >= 0 && nx < GRID_SIZE as isize && ny < GRID_SIZE as isize {
                    match game_state.board[nx as usize][ny as usize] {
                        Some(stone) if stone == self.stone => my_count += 1,
                        Some(_) => {
                            blocked.1 = true;
                            break;
                        }
                        None => {
                            open_ends += 1;
                            break;
                        }
                    }
                } else {
                    blocked.1 = true;
                }
            }

            // 计算棋型得分
            if my_count >= 5 {
                return 100000; // 五连直接胜利
            }
            if my_count == 4 {
                if open_ends == 2 {
                    score += 10000; // 活四
                } else {
                    score += 5000; // 冲四
                }
            } else if my_count == 3 {
                if open_ends == 2 {
                    score += 5000; // 活三
                } else {
                    score += 500; // 眠三
                }
            } else if my_count == 2 {
                if open_ends == 2 {
                    score += 200; // 活二
                } else {
                    score += 50; // 眠二
                }
            }

            // 防守对方棋形
            let mut opp_my_count = 0;
            let mut opp_open_ends = 0;
            let mut opp_blocked = (false, false);

            for i in 1..=4 {
                let nx = row as isize + i * dx;
                let ny = col as isize + i * dy;
                if nx >= 0 && ny >= 0 && nx < GRID_SIZE as isize && ny < GRID_SIZE as isize {
                    match game_state.board[nx as usize][ny as usize] {
                        Some(stone) if stone != self.stone => opp_my_count += 1,
                        Some(_) => {
                            opp_blocked.0 = true;
                            break;
                        }
                        None => {
                            opp_open_ends += 1;
                            break;
                        }
                    }
                } else {
                    opp_blocked.0 = true;
                }
            }

            for i in 1..=4 {
                let nx = row as isize - i * dx;
                let ny = col as isize - i * dy;
                if nx >= 0 && ny >= 0 && nx < GRID_SIZE as isize && ny < GRID_SIZE as isize {
                    match game_state.board[nx as usize][ny as usize] {
                        Some(stone) if stone != self.stone => opp_my_count += 1,
                        Some(_) => {
                            opp_blocked.1 = true;
                            break;
                        }
                        None => {
                            opp_open_ends += 1;
                            break;
                        }
                    }
                } else {
                    opp_blocked.1 = true;
                }
            }

            if opp_my_count >= 4 {
                return 90000; // 必须防守
            }
            if opp_my_count == 3 {
                if opp_open_ends == 2 {
                    score += 8000; // 对方活三
                } else {
                    score += 1000; // 对方眠三
                }
            } else if opp_my_count == 2 {
                if opp_open_ends == 2 {
                    score += 300; // 对方活二
                } else {
                    score += 100; // 对方眠二
                }
            }
        }

        score
    }

    pub fn get_stone(&self) -> Stone {
        self.stone
    }

    pub fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }
}
