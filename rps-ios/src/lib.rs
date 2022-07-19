use rps_core;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn create_game() -> *mut rps_core::Game {
    let boxed = Box::new(rps_core::Game::new());
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern "C" fn destroy_game(game: *mut rps_core::Game) {
    if !game.is_null() {
        Box::from_raw(game);
    }
}

#[no_mangle]
pub unsafe extern "C" fn make_score_board(game: *mut rps_core::Game) -> *mut c_char {
    if !game.is_null() {
        let game = &*game;
        let score = game.make_score_board();
        CString::new(score).unwrap().into_raw()
    } else {
        panic!("nullptr argument");
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_score_board(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn play_game(game: *mut rps_core::Game, choice: u8) {
    if game.is_null() {
        panic!("received nullptr argument in play_game function.");
    }

    let game = &mut *game;
    let choice = rps_core::u8_to_choice(choice);

    game.play(choice);
}
