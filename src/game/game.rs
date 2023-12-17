use crate::Engine;
use crate::Event;
use crate::KeyManager;
use crate::LevelManager;
use allegro::KeyCode;
use std::vec::Vec;

pub struct Game {
    pub engine: Engine,
    pub levels: LevelManager,
    pub player: usize,
    pub walls: Vec<usize>,
    pub enemies: Vec<usize>,
    pub key_manager: KeyManager,
}

impl Game {
    /*
     * The setup function is essentially
     * a new function, I call it setup
     * as a call back to a c style
     * setup/update combo
     */
    #[inline]
    pub fn setup() -> Self {
        // The level manager is parsing the level data here (with include_bytes)
        let levels: LevelManager = LevelManager::new();

        // Now this will setup the window and initalise allegro subsystems
        let mut engine = Engine::new(1920, 1080, 120.0);

        // this creates the player and puts them in their starting position
        let player = engine.create_sprite("assets/mc.bmp", 50, 50);
        engine.set_sprite_x(player, 1920 / 2);
        engine.set_sprite_y(player, 1080 / 2);

        // here I'm returning the game object that represents all game data
        return Game {
            engine,
            levels,
            player,
            walls: vec![],
            enemies: vec![],
            key_manager: KeyManager::new(),
        };
    }

    /*
     * The update function handles events
     * and rendering the game.
     */
    #[inline]
    pub fn update(&mut self) {
        // drawing all the sprites, returns true at 120hz
        if self.engine.redraw() {
            self.key_manager.key_press(&mut self.engine, self.player);
        }

        // low level events are managed in the engine and any higher level events
        // are returned by the method so they can be handled here
        match self.engine.manage_events() {
            // parsing the option, there aren't always events to process
            Some(event) => {
                match event {
                    // Here key up and key down events are passed to the key manager
                    // and the key manager controls/manages player inputs and what
                    // affects they have
                    Event::KeyUp { keycode, .. } => self.key_manager.set_key(keycode, false),
                    Event::KeyDown { keycode, .. } => {
                        if keycode == KeyCode::Escape {
                            // if you press escape it the next iteration of
                            self.engine.stop(); // the next call of is_running is false
                        }
                        self.key_manager.set_key(keycode, true);
                    }
                    _ => return,
                }
            }
            None => return,
        }
    }
}
