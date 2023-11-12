extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

mod keymanager;
mod sprite;
mod engine;

use crate::engine::*;
use crate::keymanager::*;
use allegro::*;

allegro_main!
{
    let mut engine = Engine::new(1920, 1080, 120.0);
    
    let player_id = engine.create_sprite("assets/mc.bmp", 50, 50);
    engine.set_sprite_x(player_id, 1920 / 2);
    engine.set_sprite_y(player_id, 1080 / 2);

    let mut keydown:KeyManager = KeyManager::new();
    while engine.is_running()  {
        if engine.redraw() {
            keydown.key_press(&mut engine, player_id);
        }
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
    }
}




