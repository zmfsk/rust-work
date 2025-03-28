use piston_window::*;

const GRID_SIZE: f64 = 30.0;
const MARGIN: f64 = 30.0;
const LINE_COUNT: usize = 15;
const PIECE_RADIUS: f64 = GRID_SIZE * 0.4;

struct GameState {
    board: [[i8; LINE_COUNT]; LINE_COUNT],
    current_player: i8,
}

impl GameState {
    fn new() -> Self {
        GameState {
            board: [[0; LINE_COUNT]; LINE_COUNT],
            current_player: 1,
        }
    }

    fn place_piece(&mut self, x: usize, y: usize) -> bool {
        // 添加调试输出
        println!("尝试落子位置: ({}, {})", x, y);
        
        if x >= LINE_COUNT || y >= LINE_COUNT {
            println!("超出边界: x={}, y={}", x, y);
            return false;
        }
        
        if self.board[y][x] != 0 {
            println!("位置已有棋子: ({}, {})", x, y);
            return false;
        }
        
        self.board[y][x] = self.current_player;
        self.current_player = 3 - self.current_player;
        true
    }
}

fn window_coord_to_board(pos: f64) -> Option<usize> {
    let adjusted_pos = pos - MARGIN;
    if adjusted_pos < -GRID_SIZE/2.0 || adjusted_pos > (LINE_COUNT as f64 - 0.5)*GRID_SIZE {
        None
    } else {
        let grid_pos = (adjusted_pos / GRID_SIZE).round();
        if grid_pos >= 0.0 && grid_pos < LINE_COUNT as f64 {
            Some(grid_pos as usize)
        } else {
            None
        }
    }
}

fn main() {
    let window_size = [
        MARGIN * 2.0 + GRID_SIZE * (LINE_COUNT - 1) as f64,
        MARGIN * 2.0 + GRID_SIZE * (LINE_COUNT - 1) as f64,
    ];

    // 明确指定OpenGL版本
    let mut window: PistonWindow = WindowSettings::new("五子棋", window_size)
        .exit_on_esc(true)
        .vsync(true)
        .graphics_api(OpenGL::V3_2)
        .build()
        .unwrap();

    let mut game_state = GameState::new();
    let mut cursor_pos = [0.0, 0.0];

    // 强制持续渲染
    window.set_ups(60);

    while let Some(event) = window.next() {
        // 更新光标位置
        if let Some(pos) = event.mouse_cursor_args() {
            cursor_pos = pos;
        }

        // 处理鼠标点击
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let (x, y) = (cursor_pos[0], cursor_pos[1]);
            println!("原始点击坐标: ({:.1}, {:.1})", x, y);
            
            if let (Some(col), Some(row)) = (
                window_coord_to_board(x),
                window_coord_to_board(y)
            ) {
                println!("转换后坐标: ({}, {})", col, row);
                if game_state.place_piece(col, row) {
                    println!("落子成功");
                }
            }
        }

        // 持续绘制（即使没有事件变化）
        window.draw_2d(&event, |c, g, _| {
            // 清空背景
            clear([0.94, 0.86, 0.72, 1.0], g);

            // 绘制棋盘线
            let line_color = [0.0, 0.0, 0.0, 1.0];
            for i in 0..LINE_COUNT {
                let pos = MARGIN + i as f64 * GRID_SIZE;
                // 水平线
                Line::new(line_color, 1.0).draw(
                    [MARGIN, pos, window_size[0]-MARGIN, pos],
                    &c.draw_state,
                    c.transform,
                    g
                );
                // 垂直线
                Line::new(line_color, 1.0).draw(
                    [pos, MARGIN, pos, window_size[1]-MARGIN],
                    &c.draw_state,
                    c.transform,
                    g
                );
            }

            // 绘制棋子（添加边界检查）
            for y in 0..LINE_COUNT {
                for x in 0..LINE_COUNT {
                    let state = game_state.board[y][x];
                    if state == 0 {
                        continue;
                    }
                    
                    // 计算实际坐标
                    let cx = MARGIN + x as f64 * GRID_SIZE;
                    let cy = MARGIN + y as f64 * GRID_SIZE;
                    
                    // 设置棋子颜色
                    let color = if state == 1 {
                        [0.0, 0.0, 0.0, 1.0] // 黑色
                    } else {
                        [1.0, 1.0, 1.0, 1.0] // 白色
                    };
                    
                    // 绘制带边框的棋子
                    ellipse(
                        color,
                        [cx-PIECE_RADIUS, cy-PIECE_RADIUS, 
                         PIECE_RADIUS*2.0, PIECE_RADIUS*2.0],
                        c.transform,
                        g
                    );
                }
            }

          
        });
    }
}