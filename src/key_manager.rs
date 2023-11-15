use crate::Engine;
use allegro::KeyCode;


const SPEED_MULT:f32 = 0.70710678118;

pub struct KeyManager {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    pub speed: i32,
    pub speed2d: i32,
}

impl KeyManager {
    #[inline]
    pub fn new() -> Self {
        KeyManager {
            up: false,
            down: false,
            right: false,
            left: false,
            speed: 2,
            speed2d: (2.0 * SPEED_MULT).floor() as i32,
        }
    }

    #[inline]
    pub fn update_speed(&mut self, speed: i32) {
        self.speed = speed;
        self.speed2d = (2.0 * SPEED_MULT).floor() as i32;
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
        else if self.down { engine.inc_sprite_y(id, self.speed) }
        else if self.left { engine.inc_sprite_x(id, -self.speed) }
        else if self.right { engine.inc_sprite_x(id, self.speed) }

        if self.up && self.left {
            engine.inc_sprite_x(id, -self.speed2d);
            engine.inc_sprite_y(id, -self.speed2d);
        }
        else if self.up && self.right {
            engine.inc_sprite_x(id, self.speed2d);
            engine.inc_sprite_y(id, -self.speed2d);
        }
        else if self.down && self.left {
            engine.inc_sprite_x(id, -self.speed2d);
            engine.inc_sprite_y(id, self.speed2d);
        }
        else if self.down && self.right {
            engine.inc_sprite_x(id, self.speed2d);
            engine.inc_sprite_y(id, self.speed2d);
        }
    }
}


