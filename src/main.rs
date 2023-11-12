extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

mod sprite;
mod engine;

use crate::engine::*;

use allegro::*;

const SPEED:i32 = 4; 


struct KeyManager {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager {
            up: false,
            down: false,
            right: false,
            left: false
        }
    }

    pub fn set_key(&mut self, keycode: KeyCode, is_down: bool) {
        match keycode {
            KeyCode::W => self.up = is_down, 
            KeyCode::S => self.down = is_down,
            KeyCode::A => self.left = is_down,
            KeyCode::D => self.right = is_down,
            _ => {}
        }    
    }
}

#[inline]
fn key_press(engine: &mut Engine, id: usize, keys: &KeyManager) {
    if keys.up { engine.inc_sprite_y(id, -SPEED) }
    if keys.down { engine.inc_sprite_y(id, SPEED) }
    if keys.left { engine.inc_sprite_x(id, -SPEED) }
    if keys.right { engine.inc_sprite_x(id, SPEED) }
}



allegro_main!
{
    let mut engine = Engine::new(1920, 1080, 120.0);
    
    let player_id = engine.create_sprite("assets/mc.bmp", 50, 50);
    engine.set_sprite_x(player_id, 500);
    engine.set_sprite_y(player_id, 500);

    let mut keydown:KeyManager = KeyManager::new();
    while engine.is_running()  {
        engine.redraw();
        match engine.manage_events() {
            Some(event) => {
                match event {
                    Event::KeyUp { keycode, ..} => keydown.set_key(keycode, false),
                    Event::KeyDown { keycode, .. }=> keydown.set_key(keycode, true),
                    _ => {},
                }
            }
            None => continue,
        }
        key_press(&mut engine, player_id, &keydown);
    }
}




