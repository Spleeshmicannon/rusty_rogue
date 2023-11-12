use crate::Engine;
use allegro::KeyCode;

pub struct KeyManager {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    pub speed: i32
}

impl KeyManager {
    pub fn new() -> Self {
        KeyManager {
            up: false,
            down: false,
            right: false,
            left: false,
            speed: 4
        }
    }

    #[inline]
    pub fn set_key(&mut self, keycode: KeyCode, is_down: bool) {
        match keycode {
            KeyCode::W => self.up = is_down, 
            KeyCode::S => self.down = is_down,
            KeyCode::A => self.left = is_down,
            KeyCode::D => self.right = is_down,
            _ => {}
        }    
    }

    #[inline]
    pub fn key_press(&self, engine: &mut Engine, id: usize) {
        if self.up { engine.inc_sprite_y(id, -self.speed) }
        if self.down { engine.inc_sprite_y(id, self.speed) }
        if self.left { engine.inc_sprite_x(id, -self.speed) }
        if self.right { engine.inc_sprite_x(id, self.speed) }
    }
}


