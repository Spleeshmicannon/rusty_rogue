extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

mod sprite;
mod engine;

use crate::engine::*;

use allegro::*;

allegro_main!
{
    let mut engine = Engine::new(1920, 1080, 144.0);
    
    let player_id = engine.create_sprite("assets/mc.bmp", 50, 50);
    engine.set_sprite_x(player_id, 500);
    engine.set_sprite_y(player_id, 500);


    while engine.is_running()  {
        engine.redraw();
        engine.manage_events();
    }
}




