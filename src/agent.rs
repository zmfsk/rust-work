use crate::game::{GameState, Stone, GRID_SIZE};
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct RandomAgent {
    stone: Stone,
}

impl RandomAgent {
    pub fn new(stone: Stone) -> Self {
        RandomAgent { stone }
    }

    pub fn make_move(&self, game_state: &GameState) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut attempts = 0;
        let max_attempts = 100;

        while attempts < max_attempts {
            let row = rng.gen_range(0..=GRID_SIZE);
            let col = rng.gen_range(0..=GRID_SIZE);

            if game_state.board[row][col].is_none() {
                return Some((row, col));
            }

            attempts += 1;
        }

        None
    }

    pub fn get_stone(&self) -> Stone {
        self.stone
    }

    pub fn set_stone(&mut self, stone: Stone) {
        self.stone = stone;
    }
} 