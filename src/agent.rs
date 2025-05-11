use crate::game::{GRID_SIZE, GameState, Stone};
use crate::game_manager::check_victory;
use bevy::prelude::*;
use std::cmp;
use std::collections::HashSet; // 用于存储相关落子位置，避免重复

const WIN_SCORE: i32 = 100_000_000; // 获胜得分
const FIVE_SCORE: i32 = 1_000_000; // 五子连珠
const OPEN_FOUR_SCORE: i32 = 10_000; // 活四
const HALF_FOUR_SCORE: i32 = 1_000; // 冲四/死四
const OPEN_THREE_SCORE: i32 = 1_000; // 活三
const HALF_THREE_SCORE: i32 = 100; // 眠三/死三
const OPEN_TWO_SCORE: i32 = 50; // 活二
const HALF_TWO_SCORE: i32 = 10; // 眠二/死二
const POSITIONAL_WEIGHT: i32 = 1; // 靠近中心的微小加分

// 控制 AI 考虑的有效移动半径
// 只考虑距离现有棋子 MOVE_RADIUS 范围内的空位
const MOVE_RADIUS: usize = 2; // 可以调整这个值，越大AI考虑越多，但越慢

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

        // 获取相关联的有效落子位置（剪枝）
        let relevant_moves = self.get_relevant_moves(game_state);
        if relevant_moves.is_empty() {
            // 回退策略：如果没有相关落子位置，检查是否还有任何有效落子（可能是和棋或棋盘满了）
            let all_valid_moves = game_state.get_valid_moves();
            if all_valid_moves.is_empty() {
                return None; // 没有有效移动（和棋）
            } else {
                return None; // 棋盘未满但没有相关移动，视为找不到合适落子点
            }
        }

        // 如果只有一个相关移动，直接返回
        if relevant_moves.len() == 1 {
            return Some(relevant_moves[0]);
        }

        // 初始化最佳移动和分数
        // 可以将初始最佳移动设置为相关移动列表中的第一个
        let mut best_move = relevant_moves[0];
        let mut best_score = i32::MIN;

        // 初始化 Alpha-Beta 剪枝的 alpha 和 beta 值
        let mut alpha = i32::MIN; // Alpha: 最大化玩家能确保得到的最低分数
        let beta = i32::MAX; // Beta: 最小化玩家能限制最大化玩家得到的最高分数

        // --- 落子顺序优化：评估第一层的每个相关移动，并排序 ---
        // 使用一个简单的启发式：落子后的即时估值
        let mut moves_with_scores: Vec<((usize, usize), i32)> = relevant_moves
            .into_iter()
            .map(|m| {
                // 复制状态用于进行临时的落子和估值，不影响原始 game_state
                let mut temp_state = game_state.clone();
                let mut score = i32::MIN; // 默认给一个低分，如果落子失败

                if temp_state.apply_move(m.0, m.1, self.stone).is_ok() {
                    // 使用静态估值函数评估落子后的状态作为排序依据
                    score = self.evaluate_board(&temp_state);
                    // 注意：这里只评估了即时分数，更复杂的排序可以使用浅层搜索（比如深度1的 Minimax）
                }
                (m, score)
            })
            .collect();

        // 按估值从高到低排序（AI 是最大化玩家）
        moves_with_scores.sort_by(|a, b| b.1.cmp(&a.1));

        for ((r, c), _) in moves_with_scores {
            // 模拟在 (r, c) 处落子，使用克隆的状态
            if let Some(mut next_state) = game_state.make_move_simulated(r, c, self.stone) {
                // make_move_simulated 需要克隆

                // --- 立即获胜检查 ---
                // 如果这一步能直接获胜，就选择它
                if check_victory(&next_state) == Some(self.stone) {
                    //println!("AI 找到直接获胜点： ({}, {})", r, c); // Debug
                    return Some((r, c));
                }

                // --- 调用 Minimax ---
                // 从对手（最小化玩家）的角度开始递归搜索
                let score = self.minimax(
                    &mut next_state,       // 传入模拟后的可变状态引用
                    self.search_depth - 1, // 深度减 1
                    false,                 // 现在轮到对手 (Minimizing Player)
                    alpha,                 // 传递当前的 alpha
                    beta,                  // 传递当前的 beta
                );

                // --- 更新最佳移动 ---
                if score > best_score {
                    best_score = score;
                    best_move = (r, c);
                    //println!("AI 新的最佳移动： ({}, {}) 估值： {}", r, c, score); // Debug
                }

                // --- 更新 Alpha (最大化玩家的最佳保证) ---
                // Alpha 是 AI 在当前路径下能获得的最大分数，用于剪枝
                alpha = cmp::max(alpha, score);

                // --- Beta 剪枝 ---
                // 如果 Beta <= Alpha，表示最小化玩家在其他分支有更好的选择，
                // 最大化玩家不会选择这条路径，可以停止搜索该分支。
                if beta <= alpha {
                    // println!("在深度 {} 进行 Alpha-Beta 剪枝 (alpha={}, beta={})", self.search_depth, alpha, beta); // Debug
                    break; // 剪枝
                }
            } else {
            }
        }
        //println!("AI 最终选择的移动： ({}, {}) 估值： {}", best_move.0, best_move.1, best_score);
        Some(best_move) // 返回找到的最佳移动
    }

    /// Minimax 递归函数 (带有 Alpha-Beta 剪枝)
    /// 参数 game_state 现在是 可变引用 (&mut GameState)
    fn minimax(
        &self,
        game_state: &mut GameState, // 传入可变引用以进行 in-place 修改
        depth: u32,
        maximizing_player: bool, // true: 当前是 AI (最大化玩家) 的回合, false: 当前是对手 (最小化玩家) 的回合
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // --- 终止条件 (Base Cases) ---

        // 1. 检查游戏是否在本状态结束 (胜利/失败)
        // 注意：check_victory 应该检查传入的 game_state
        if let Some(winner) = check_victory(game_state) {
            return if winner == self.stone {
                // AI 获胜：分数高，且深度越高（越快获胜）分数相对越高
                WIN_SCORE + depth as i32
            } else {
                // 对手获胜：分数低（负分），深度越高（对手越快获胜）负分越绝对
                -(WIN_SCORE + depth as i32)
            };
        }

        // 2. 检查是否平局 (没有相关联的有效移动 left)
        // 这里我们继续使用 get_relevant_moves 来限制分支
        let relevant_moves = self.get_relevant_moves(game_state);
        if relevant_moves.is_empty() {
            let all_moves = game_state.get_valid_moves();
            if all_moves.is_empty() {
                return 0; // 和棋分数为 0
            }
            // 如果有有效移动但没有相关移动，评估当前状态
            return self.evaluate_board(game_state);
        }

        // 3. 检查是否达到搜索深度限制
        if depth == 0 {
            // 到达叶节点，评估当前棋盘状态
            return self.evaluate_board(game_state);
        }

        // --- 递归步骤 ---

        let current_player_stone = if maximizing_player {
            self.stone
        } else {
            self.stone.opponent()
        };

        // --- 落子顺序优化：评估当前层的所有相关移动，并排序 ---
        let mut moves_with_scores: Vec<((usize, usize), i32)> = relevant_moves
            .into_iter()
            .map(|m| {
                // 临时落子并评估即时分数用于排序
                let mut score = 0; // Default score
                if game_state
                    .apply_move(m.0, m.1, current_player_stone)
                    .is_ok()
                {
                    score = self.evaluate_board(game_state);
                    game_state.undo_move(m.0, m.1); // 立即悔棋
                } else {
                    // Should not happen with get_relevant_moves + apply_move
                    // eprintln!("apply_move for sorting failed for ({}, {})", m.0, m.1); // Error debug
                }
                (m, score)
            })
            .collect();

        // 排序移动：最大化玩家按估值从高到低排，最小化玩家按估值从低到高排
        if maximizing_player {
            moves_with_scores.sort_by(|a, b| b.1.cmp(&a.1));
        } else {
            moves_with_scores.sort_by(|a, b| a.1.cmp(&b.1));
        }
        //println!("Minimax 在深度 {} 的 {} 回合落子排序： {:?}", depth, if maximizing_player {"AI"} else {"对手"}, moves_with_scores); // Debug

        if maximizing_player {
            // --- AI (最大化玩家) 的回合 ---
            let mut max_eval = i32::MIN; // 初始化最大评分为负无穷

            // 遍历所有可能的移动 (已排序)
            for ((r, c), _) in moves_with_scores {
                // 在当前状态上“落子”（in-place 修改）
                if game_state.apply_move(r, c, current_player_stone).is_ok() {
                    // 递归调用 minimax，切换到最小化玩家的回合
                    let eval = self.minimax(game_state, depth - 1, false, alpha, beta);

                    // “悔棋”：恢复到修改前的状态
                    game_state.undo_move(r, c);

                    max_eval = cmp::max(max_eval, eval); // 更新最大评估值

                    // --- Alpha 更新 ---
                    // Alpha 是最大化玩家到目前为止能确保得到的最好分数
                    alpha = cmp::max(alpha, eval);

                    // --- Beta 剪枝 ---
                    // 如果 Beta <= Alpha，表示最小化玩家在之前已经找到了一条更差的路径
                    // （对于最大化玩家来说分数更低），所以最大化玩家不会选择当前这条路径。
                    // 可以停止搜索该分支。
                    if beta <= alpha {
                        // println!("Alpha-Beta 剪枝发生，深度 {} (alpha={}, beta={})", depth, alpha, beta); // Debug
                        break; // 剪枝
                    }
                } else {
                    // 理论上 get_relevant_moves + apply_move 不会失败
                    // eprintln!("apply_move 在深度 {} 的 ({}, {}) 失败", depth, r, c); // Error debug
                }
            }
            max_eval // 返回该节点的最大评估值
        } else {
            // --- 对手 (最小化玩家) 的回合 ---
            let mut min_eval = i32::MAX; // 初始化最小评分为正无穷

            // 遍历所有可能的移动 (已排序)
            for ((r, c), _) in moves_with_scores {
                // 在当前状态上“落子”（in-place 修改）
                if game_state.apply_move(r, c, current_player_stone).is_ok() {
                    // 递归调用 minimax，切换到最大化玩家的回合
                    let eval = self.minimax(game_state, depth - 1, true, alpha, beta);

                    // “悔棋”：恢复到修改前的状态
                    game_state.undo_move(r, c);

                    min_eval = cmp::min(min_eval, eval); // 更新最小评估值

                    // --- Beta 更新 ---
                    // Beta 是最小化玩家到目前为止能确保得到的最好分数（上限）
                    beta = cmp::min(beta, eval);

                    // --- Alpha 剪枝 ---
                    // 如果 Beta <= Alpha，表示最大化玩家在之前已经找到了一条更好的路径
                    // （对于最小化玩家来说分数更高），所以最小化玩家会避免当前这条路径。
                    // 可以停止搜索该分支。
                    if beta <= alpha {
                        // println!("Alpha-Beta 剪枝发生，深度 {} (alpha={}, beta={})", depth, alpha, beta); // Debug
                        break; // 剪枝
                    }
                } else {
                    // 理论上 get_relevant_moves + apply_move 不会失败
                    // eprintln!("apply_move 在深度 {} 的 ({}, {}) 失败", depth, r, c); // Error debug
                }
            }
            min_eval // 返回该节点的最小评估值
        }
    }

    /// 生成相关联的落子位置列表：距离现有棋子 MOVE_RADIUS 范围内的空位。
    /// 这可以显著剪枝搜索空间。
    fn get_relevant_moves(&self, game_state: &GameState) -> Vec<(usize, usize)> {
        let mut relevant_moves = HashSet::new(); // 使用 HashSet 避免重复位置

        // 定义一个位置周围的 8 个邻居方向
        let neighbor_dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let board_size = GRID_SIZE + 1;

        // 遍历棋盘上的每个位置
        for r in 0..board_size {
            for c in 0..board_size {
                // 如果当前位置有棋子
                if game_state.board[r][c].is_some() {
                    // 以这个棋子为中心，检查半径 MOVE_RADIUS 范围内的空位
                    for dr in -(MOVE_RADIUS as isize)..=(MOVE_RADIUS as isize) {
                        for dc in -(MOVE_RADIUS as isize)..=(MOVE_RADIUS as isize) {
                            // 跳过棋子本身的位置
                            if dr == 0 && dc == 0 {
                                continue;
                            }

                            let nr = r as isize + dr;
                            let nc = c as isize + dc;

                            // 检查是否在棋盘界限内
                            if nr >= 0
                                && nr < board_size as isize
                                && nc >= 0
                                && nc < board_size as isize
                            {
                                let nr_u = nr as usize;
                                let nc_u = nc as usize;

                                // 如果该位置是空的，就认为它是一个相关联的落子位置
                                if game_state.board[nr_u][nc_u].is_none() {
                                    relevant_moves.insert((nr_u, nc_u)); // 插入 HashSet，自动去重
                                }
                            }
                        }
                    }
                }
            }
        }

        // 特殊处理开局第一子的情况：如果棋盘上没有任何棋子（或相关半径内没有空位），
        // 应该至少提供中心点作为可选移动。
        let mut has_stone = false;
        for r in 0..board_size {
            for c in 0..board_size {
                if game_state.board[r][c].is_some() {
                    has_stone = true;
                    break;
                }
            }
            if has_stone {
                break;
            }
        }

        if !has_stone {
            // 如果棋盘完全是空的，只返回中心点作为第一个移动
            let center = GRID_SIZE / 2;
            relevant_moves.insert((center, center));
        } else if relevant_moves.is_empty() && !game_state.get_valid_moves().is_empty() {
        }

        relevant_moves.into_iter().collect() // 将 HashSet 转换为 Vec
    }

    /// 评估整个棋盘状态
    /// 从 AI 的角度计算分数，正分表示 AI 有利，负分表示对手有利。
    /// 遍历所有可能的 5 子棋型窗口进行评估。
    fn evaluate_board(&self, game_state: &GameState) -> i32 {
        let mut ai_score = 0;
        let mut opponent_score = 0;
        let opponent_stone = self.stone.opponent();
        let board_size = GRID_SIZE + 1;

        // 定义检查方向：水平、垂直、主对角线、副对角线
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)]; // (dr, dc)

        // 遍历棋盘上的每个点，作为潜在 5 子棋型线段的 **起点**
        // 一个 5 子棋型从 (r, c) 开始，沿 (dr, dc) 方向，结束点是 (r + 4*dr, c + 4*dc)
        // 因此，起点的范围需要确保结束点在棋盘内。
        for r in 0..board_size {
            for c in 0..board_size {
                for &(dr, dc) in &directions {
                    // 计算棋型线段的结束点
                    let end_r = r as isize + 4 * dr;
                    let end_c = c as isize + 4 * dc;

                    // 如果结束点在棋盘界限内，说明可以形成一个 5 子棋型窗口
                    if end_r >= 0
                        && end_r < board_size as isize
                        && end_c >= 0
                        && end_c < board_size as isize
                    {
                        // 评估从 (r, c) 开始，沿 (dr, dc) 方向的这个 5 子棋型窗口
                        // evaluate_pattern 函数会查看这 5 个位置以及它们两端的空/阻挡情况
                        ai_score += self.evaluate_pattern(game_state, r, c, dr, dc, self.stone);
                        opponent_score +=
                            self.evaluate_pattern(game_state, r, c, dr, dc, opponent_stone);
                    }
                }
            }
        }

        // --- 位置加分  ---
        // 让 AI 稍微倾向于占据中心位置
        let mut positional_score = 0;
        if POSITIONAL_WEIGHT > 0 {
            let center = (GRID_SIZE as i32) / 2; // 使用 i32 进行计算
            for r in 0..board_size {
                for c in 0..board_size {
                    let r_i32 = r as i32;
                    let c_i32 = c as i32;
                    // 计算曼哈顿距离到中心
                    let dist_from_center = (r_i32 - center).abs() + (c_i32 - center).abs();
                    // 距离中心越近，加分越多
                    let bonus =
                        (POSITIONAL_WEIGHT * (GRID_SIZE as i32 / 2) - dist_from_center).max(0);

                    if game_state.board[r][c] == Some(self.stone) {
                        positional_score += bonus;
                    } else if game_state.board[r][c] == Some(opponent_stone) {
                        positional_score -= bonus;
                    }
                }
            }
        }

        // --- 最终评估分数 ---
        // AI 的总棋型分数 - 对手的总棋型分数 + 位置分数
        ai_score - opponent_score + positional_score
    }

    /// 评估从 (r, c) 开始，沿 (dr, dc) 方向的一个潜在 5 子棋型窗口。
    /// 查看从 (r, c) 到 (r + 4*dr, c + 4*dc) 的 5 个位置以及它们两端的位置。
    fn evaluate_pattern(
        &self,
        game_state: &GameState,
        r: usize,
        c: usize,
        dr: isize,
        dc: isize,
        player_stone: Stone,
    ) -> i32 {
        let opponent_stone = player_stone.opponent();
        let board_size = GRID_SIZE + 1;

        let mut player_stones_in_pattern = 0;
        let mut opponent_stones_in_pattern = 0;
        // let mut empty_in_pattern = 0; // 当前评估中未使用

        // 检查潜在棋型内的 5 个位置
        for i in 0..5 {
            let curr_r = r as isize + i * dr;
            let curr_c = c as isize + i * dc;
            // 在 evaluate_board 中已经检查了结束点在界限内，所以这里不需要再次检查 curr_r/curr_c 的界限

            match game_state.board[curr_r as usize][curr_c as usize] {
                Some(stone) if stone == player_stone => player_stones_in_pattern += 1,
                Some(stone) if stone == opponent_stone => opponent_stones_in_pattern += 1,
                None => { /*empty_in_pattern += 1;*/ }
                _ => {} // 不应该出现
            }
        }

        // 如果这个 5 子窗口内有对手的棋子，则这个棋型被阻挡，对当前玩家没有价值
        if opponent_stones_in_pattern > 0 {
            return 0;
        }

        // 如果窗口内没有当前玩家的棋子，也没有价值
        if player_stones_in_pattern == 0 {
            return 0;
        }

        // 现在检查两端是否开放 (没有被对手的棋子阻挡 或 没有出界)
        let mut open_ends = 0;

        // 检查后方一格 ((r, c) 前一格)
        let br = r as isize - dr;
        let bc = c as isize - dc;
        // 如果后方位置在界限内且是空的
        if br >= 0 && br < board_size as isize && bc >= 0 && bc < board_size as isize {
            if game_state.board[br as usize][bc as usize].is_none() {
                open_ends += 1;
            }
            // 如果是对手棋子，则此端被阻挡，open_ends 不增加
        } else {
            // 如果出界，理论上也是被边界“阻挡”了，不算开放端
            // 我们只计算棋盘内的空位作为开放端
        }

        // 检查前方一格 (5子棋型结束点后一格)
        let fr = r as isize + 5 * dr;
        let fc = c as isize + 5 * dc;
        // 如果前方位置在界限内且是空的
        if fr >= 0 && fr < board_size as isize && fc >= 0 && fc < board_size as isize {
            if game_state.board[fr as usize][fc as usize].is_none() {
                open_ends += 1;
            }
            // 如果是对手棋子，则此端被阻挡
        } else {
            // 如果出界，不算开放端
        }

        // 根据窗口内连续棋子数和开放端数量给分
        match player_stones_in_pattern {
            5 => FIVE_SCORE, // 5 连 (理论上会被胜利检查捕获)
            4 => {
                match open_ends {
                    2 => OPEN_FOUR_SCORE, // 活四
                    1 => HALF_FOUR_SCORE, // 冲四/死四
                    _ => 0,               // 死四（两端都被封锁）
                }
            }
            3 => {
                match open_ends {
                    2 => OPEN_THREE_SCORE, // 活三
                    1 => HALF_THREE_SCORE, // 眠三/死三
                    _ => 0,                // 死三
                }
            }
            2 => {
                match open_ends {
                    2 => OPEN_TWO_SCORE, // 活二
                    1 => HALF_TWO_SCORE, // 眠二/死二
                    _ => 0,              // 死二
                }
            }
            _ => 0, // 1 个棋子在此棋型评估中无直接价值
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
        self.search_depth = depth.max(1);
    }

    pub fn get_difficulty(&self) -> u32 {
        self.search_depth
    }
}
