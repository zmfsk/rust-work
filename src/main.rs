mod board;
mod game;
mod game_manager;
mod input;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use board::setup_board;
use game::GameState;
use game_manager::check_victory;
use input::place_stone;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.8, 0.6)))
        .insert_resource(GameState::new())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "五子棋".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_board)
        .add_systems(Update, place_stone.after(check_victory_system))
        .add_systems(Update, check_victory_system)
        .run();
}

/// 系统：检查胜负
fn check_victory_system(mut game_state: ResMut<GameState>) {
    if let Some(winner) = check_victory(&game_state) {
        println!("游戏结束！获胜者是: {:?}", winner);
        game_state.is_game_over = true; // 设置游戏结束标志
    }
}
