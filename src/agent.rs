use crate::game::{GRID_SIZE, GameState, Stone};
use crate::game_manager::check_victory;
use bevy::prelude::*;
use std::cmp;

const WIN_SCORE: i32 = 100_000_000; // 获胜得分
const FIVE_SCORE: i32 = 1_000_000; // 五子连珠 
const OPEN_FOUR_SCORE: i32 = 10_000; // 活四
const HALF_FOUR_SCORE: i32 = 1_000; // 冲四/死四 
const OPEN_THREE_SCORE: i32 = 1_000; // 活三
const HALF_THREE_SCORE: i32 = 100; // 眠三/死三
const OPEN_TWO_SCORE: i32 = 50; // 活二
const HALF_TWO_SCORE: i32 = 10; // 眠二/死二
const POSITIONAL_WEIGHT: i32 = 1; // 靠近中心的微小加分 

#[derive(Resource)]
pub struct SmartAgent {
    stone: Stone,
    search_depth: u32, // Minimax 搜索深度，控制 AI 强度
}

impl SmartAgent {
    pub fn new(stone: Stone, depth: u32) -> Self {
        SmartAgent {
            stone,
            search_depth: depth.max(1),
        }
    }

    pub fn make_move(&self, game_state: &GameState) -> Option<(usize, usize)> {
        if game_state.is_game_over {
            return None;
        }

        // 获取所有可以落子的位置
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            return None; // 没有有效移动（平局或棋盘已满）
        }

        if valid_moves.len() == 1 {
            return Some(valid_moves[0]);
        }

        // 初始化最佳移动和分数
        // TODO: 优化 - 将初始最佳移动设置为更中心的位置
        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;

        // 初始化 Alpha-Beta 剪枝的 alpha 和 beta 值
        let mut alpha = i32::MIN; // Alpha: Maximizer 能确保得到的最低分数
        let beta = i32::MAX; // Beta: Minimizer 能确保得到的最高分数 (从 Maximizer 角度看)

        // --- 迭代 AI 的第一步所有可能移动 ---
        for (r, c) in valid_moves {
            // 模拟在 (r, c) 处落子
            // 使用克隆的状态进行模拟，避免修改原始 game_state
            if let Some(mut next_state) = game_state.make_move_simulated(r, c, self.stone) {
                // --- 立即获胜检查 ---
                // 如果这一步能直接获胜，就选择它
                if check_victory(&next_state) == Some(self.stone) {
                    //println!("AI found immediate win at ({}, {})", r, c); // Debug
                    return Some((r, c));
                }

                // --- 调用 Minimax ---
                // 从对手（Minimizer）的角度开始递归搜索
                let score = self.minimax(
                    &mut next_state,       // 传入模拟后的状态
                    self.search_depth - 1, // 深度减 1
                    false,                 // 现在轮到对手 (Minimizing Player)
                    alpha,                 // 传递当前的 alpha
                    beta,                  // 传递当前的 beta
                );

                // --- 更新最佳移动 ---
                if score > best_score {
                    best_score = score;
                    best_move = (r, c);
                    //println!("New best move for AI: ({}, {}) with score {}", r, c, score); // Debug
                }

                // --- 更新 Alpha (Maximizer 的最佳保证) ---
                // Alpha 是 AI 在当前路径下能获得的最大分数
                alpha = cmp::max(alpha, best_score);
            }
        }
        //println!("AI chose move: ({}, {}) with score {}", best_move.0, best_move.1, best_score); // Final choice debug
        Some(best_move) // 返回找到的最佳移动
    }

    /// Minimax 递归函数 (带有 Alpha-Beta 剪枝)
    fn minimax(
        &self,
        game_state: &mut GameState, // 传入可变引用或在内部克隆
        depth: u32,
        maximizing_player: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // --- 终止条件 (Base Cases) ---

        // 1. 检查游戏是否在本状态结束 (胜利/失败)
        if let Some(winner) = check_victory(game_state) {
            return if winner == self.stone {
                WIN_SCORE + depth as i32 // 优先选择更快的胜利 (depth 越高，分数越高)
            } else {
                -(WIN_SCORE + depth as i32) // 优先阻止对手更快的胜利 (depth 越高，负分越绝对)
            };
        }

        // 2. 检查是否平局 (没有有效移动)
        let valid_moves = game_state.get_valid_moves();
        if valid_moves.is_empty() {
            return 0; // 平局分数为 0
        }

        // 3. 检查是否达到搜索深度限制
        if depth == 0 {
            // 到达叶节点，评估当前棋盘状态
            return self.evaluate_board(game_state);
        }

        // --- 递归步骤 ---
        if maximizing_player {
            // --- AI (Maximizer) 的回合 ---
            let mut max_eval = i32::MIN; // 初始化最大评分为负无穷

            // 遍历所有可能的移动
            // 优化点: 可以尝试根据启发式评估对 `valid_moves` 进行排序，优先搜索好的移动
            for (r, c) in valid_moves {
                // 模拟移动 (使用克隆状态保证纯净)
                if let Some(mut next_state) = game_state.make_move_simulated(r, c, self.stone) {
                    // 递归调用 minimax，切换到 Minimizer 的回合
                    let eval = self.minimax(&mut next_state, depth - 1, false, alpha, beta);
                    max_eval = cmp::max(max_eval, eval); // 更新最大评估值

                    // --- Alpha 更新 ---
                    alpha = cmp::max(alpha, eval); // 更新 Maximizer 能确保的最佳分数

                    // --- Beta 剪枝 ---
                    // 如果 Beta <= Alpha，表示 Minimizer 在其他分支有更好的选择，
                    // Maximizer 不会选择这条路径，可以停止搜索该分支。
                    if beta <= alpha {
                        break;
                    }
                }
            }
            max_eval // 返回该节点的最大评估值
        } else {
            // --- 对手 (Minimizer) 的回合 ---
            let mut min_eval = i32::MAX; // 初始化最小评分为正无穷
            let opponent_stone = self.stone.opponent(); // 获取对手棋子颜色

            for (r, c) in valid_moves {
                if let Some(mut next_state) = game_state.make_move_simulated(r, c, opponent_stone) {
                    // 递归调用 minimax，切换到 Maximizer 的回合
                    let eval = self.minimax(&mut next_state, depth - 1, true, alpha, beta);
                    min_eval = cmp::min(min_eval, eval); // 更新最小评估值

                    // --- Beta 更新 ---
                    beta = cmp::min(beta, eval); // 更新 Minimizer 能确保的最佳分数 (上限)

                    // --- Alpha 剪枝 ---
                    // 如果 Beta <= Alpha，表示 Maximizer 在其他分支有更好的选择，
                    if beta <= alpha {
                        break;
                    }
                }
            }
            min_eval // 返回该节点的最小评估值
        }
    }

    /// 评估整个棋盘状态
    /// 从 AI 的角度计算分数，正分表示 AI 有利，负分表示对手有利。
    fn evaluate_board(&self, game_state: &GameState) -> i32 {
        let mut ai_score = 0;
        let mut opponent_score = 0;
        let opponent_stone = self.stone.opponent();

        // 定义检查方向：水平、垂直、主对角线、副对角线
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)]; // (dr, dc)

        // 遍历棋盘上的每个点，作为潜在棋型的起点
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                // 只需评估从有棋子的位置开始的线
                // if game_state.board[r][c].is_none() { continue; }

                for &(dr, dc) in &directions {
                    ai_score += self.evaluate_line(game_state, r, c, dr, dc, self.stone);
                    opponent_score += self.evaluate_line(game_state, r, c, dr, dc, opponent_stone);
                }
            }
        }

        // --- 位置加分  ---
        // 让 AI 稍微倾向于占据中心位置
        let mut positional_score = 0;
        if POSITIONAL_WEIGHT > 0 {
            let center = GRID_SIZE / 2;
            for r in 0..GRID_SIZE {
                for c in 0..GRID_SIZE {
                    if game_state.board[r][c] == Some(self.stone) {
                        let dist =
                            (r as i32 - center as i32).abs() + (c as i32 - center as i32).abs();
                        positional_score +=
                            (POSITIONAL_WEIGHT * (GRID_SIZE as i32 / 2) - dist).max(0); // 离中心越近，加分越多
                    } else if game_state.board[r][c] == Some(opponent_stone) {
                        let dist =
                            (r as i32 - center as i32).abs() + (c as i32 - center as i32).abs();
                        positional_score -=
                            (POSITIONAL_WEIGHT * (GRID_SIZE as i32 / 2) - dist).max(0); // 对手占中心则减分
                    }
                }
            }
        }

        // --- 最终评估分数 ---
        // AI 总分 - 对手总分 + 位置分数
        ai_score - opponent_score + positional_score
    }

    /// 评估从 (r, c) 开始，沿 (dr, dc) 方向的单一棋型线
    fn evaluate_line(
        &self,
        game_state: &GameState,
        r: usize,
        c: usize,
        dr: isize,
        dc: isize,
        player_stone: Stone,
    ) -> i32 {
        // 优化：如果起点不是 player_stone，则此方向由此起点开始的 player_stone 棋型得分为 0
        if game_state.board[r][c] != Some(player_stone) {
            return 0;
        }

        let opponent_stone = player_stone.opponent();
        let mut consecutive = 0; // 连续棋子数
        let mut open_ends = 0; // 开放端数量 (0, 1, 或 2)
        let line_len = 5; // 我们关心最多 5 子连线

        // --- 检查 "后面" 是否开放 ---
        let br = r as isize - dr; // Backward row
        let bc = c as isize - dc; // Backward col
        // 如果出界或是空格，则认为是开放的
        if !(br >= 0
            && br < GRID_SIZE as isize
            && bc >= 0
            && bc < GRID_SIZE as isize
            && game_state.board[br as usize][bc as usize] == Some(opponent_stone))
        {
            open_ends += 1;
        }

        // --- 检查 "前面" 的连续棋子和是否开放 ---
        for i in 0..GRID_SIZE {
            // 检查从起点开始的棋子 (use usize loop)
            let nr = r as isize + (i as isize) * dr; // Next row
            let nc = c as isize + (i as isize) * dc; // Next col

            // 检查边界
            if nr < 0 || nr >= GRID_SIZE as isize || nc < 0 || nc >= GRID_SIZE as isize {
                // 到达边界，视为非对手阻挡
                // 只有在连续棋子中断时才增加开放端
                //if i > 0 { // 确保不是起点本身就在边界
                //    open_ends += 1;
                //}
                break; // Hit boundary
            }

            let nr_u = nr as usize;
            let nc_u = nc as usize;

            // 判断当前位置状态
            if game_state.board[nr_u][nc_u] == Some(player_stone) {
                consecutive += 1;
            } else {
                // 遇到空格或对手棋子，则连续中断
                // 如果是空格，则这一端是开放的
                if game_state.board[nr_u][nc_u].is_none() {
                    open_ends += 1;
                }
                // 如果是对手棋子，则此端不开放 (open_ends 不增加)
                break; // 中断检查
            }

            // 如果已经找到足够长的连子，可以提前停止（可选优化）
            // if consecutive >= line_len { break; }
        }

        // 根据实际连续长度和两端情况给分
        match consecutive {
            5.. => FIVE_SCORE, // 5 个或更多 (理论上会被胜利检查捕获)
            4 => {
                match open_ends {
                    2 => OPEN_FOUR_SCORE, // 活四
                    1 => HALF_FOUR_SCORE, // 死四/冲四
                    _ => 0,               // 被完全封锁的四子无价值
                }
            }
            3 => {
                match open_ends {
                    2 => OPEN_THREE_SCORE, // 活三
                    1 => HALF_THREE_SCORE, // 眠三
                    _ => 0,
                }
            }
            2 => {
                match open_ends {
                    2 => OPEN_TWO_SCORE, // 活二
                    1 => HALF_TWO_SCORE, // 眠二
                    _ => 0,
                }
            }
            _ => 0, // 1 个棋子无直接棋型价值
        }
    }

    // --- Getters 和 Setters ---

    /// 获取 AI 当前使用的棋子颜色
    pub fn get_stone(&self) -> Stone {
        self.stone
    }

    /// 设置 AI 使用的棋子颜色
    pub fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }

    /// 设置 AI 的搜索深度
    pub fn set_depth(&mut self, depth: u32) {
        // 确保深度至少为 1
        self.search_depth = depth.max(1);
        //println!("AI depth set to: {}", self.search_depth); // Debug
    }
}
