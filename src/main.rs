extern crate allegro; // the core allegro libraries
extern crate allegro_font; // necessary for true type fonts
extern crate allegro_image; // necessary for image parsing

mod game; // essentially a misc category
mod input;
mod level; // all level related stuff

use crate::game::engine::*; // handles allegro calls and physics
use crate::game::game::Game;
use crate::input::key_manager::*; // handles key inputs
use crate::level::level_manager::*; // parses level files into a datastructure

use allegro::*; // some of allegro is exposed here (events)

allegro_main! {
    let mut game = Game::setup();
    while game.engine.is_running() {
        game.update();
    }
}
