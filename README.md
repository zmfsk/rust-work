# Gobang Game (五子棋)

一个使用Rust和Bevy游戏引擎开发的五子棋游戏，具有智能AI对手和现代化UI界面。

## 功能特点

- 完整的五子棋游戏规则实现
- 智能AI对手，支持多级难度设置
- 玩家评分系统，评估每一步棋的质量
- 美观的游戏界面，包括主菜单和游戏界面
- 支持人机对战，可切换先后手
- 实时显示玩家评分和游戏状态
- 游戏结束后显示胜利窗口，可选择再玩一局

## 技术栈

- **Rust语言**: 高性能、内存安全的系统编程语言
- **Bevy引擎**: 现代化的数据驱动游戏引擎
- **bevy_prototype_lyon**: 用于绘制2D图形
- **Minimax算法**: AI决策的核心算法，带Alpha-Beta剪枝优化

## 项目结构

- `main.rs`: 程序入口，设置游戏窗口和系统
- `game.rs`: 游戏核心逻辑和状态管理
- `board.rs`: 棋盘渲染和交互
- `input.rs`: 用户输入处理
- `agent.rs`: AI智能体实现
- `evaluator.rs`: 棋盘局势评估
- `game_manager.rs`: 游戏流程管理
- `ui.rs`: 用户界面组件

## 安装与运行

### 前置要求

- Rust和Cargo (推荐使用[rustup](https://rustup.rs/)安装)

### 构建与运行

```bash
# 克隆仓库
git clone <仓库URL>
cd rust-work

# 开发模式运行
cargo run

# 发布模式构建
cargo build --release
```

发布版本的可执行文件将位于`target/release/gobang.exe`(Windows)或`target/release/gobang`(Linux/macOS)。

## 游戏玩法

1. 启动游戏后，在主菜单选择难度级别并点击"开始游戏"
2. 游戏默认玩家使用黑子先手，AI使用白子
3. 点击棋盘上的交叉点放置棋子
4. 使用界面右侧的按钮可以：
   - 重置游戏
   - 切换先后手
   - 查看游戏说明
5. 游戏会自动判断胜负，并在一方获胜时显示胜利窗口

## AI难度说明

游戏支持多个AI难度级别，通过调整Minimax算法的搜索深度实现：

- **简单**: 搜索深度为1，AI只考虑当前局面
- **中等**: 搜索深度为2，AI能预测玩家的下一步动作
- **困难**: 搜索深度为3，AI能进行更深层次的推理

## 开发笔记

- 使用Bevy ECS系统进行游戏开发
- AI使用Minimax算法带Alpha-Beta剪枝优化
- 棋型评估包括五连、活四、冲四、活三等多种情况
- 玩家评分系统基于每步棋与AI最优解的比较

## 故障排除

如果遇到Cargo缓存问题，可以尝试清理缓存：
```
del ~/.cargo/.package-cache  # Windows
rm ~/.cargo/.package-cache   # Linux/macOS
```

## 许可证

[MIT](LICENSE)
