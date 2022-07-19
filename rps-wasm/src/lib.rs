mod utils;

use rps_core;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
struct Game {
    game: rps_core::Game,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Game {
            game: rps_core::Game::new(),
        }
    }

    pub fn make_score_board(&self) -> String {
        self.game.make_score_board()
    }

    pub fn play_game(&mut self, choice: u8) {
        let choice = rps_core::u8_to_choice(choice);
        self.game.play(choice);
    }
}
