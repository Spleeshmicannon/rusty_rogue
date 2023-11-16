use allegro::Bitmap;
use allegro::Core;

pub struct Sprite { 
    pub width: i32, 
    pub height: i32, 
    pub x: i32,
    pub y: i32,
    pub image: Bitmap
}

impl Sprite {
    /*
     * Loads the bitmap and puts default/input values into the sprite.
     */
    pub fn new(core: &Core, path:&'static str, width: i32, height: i32) -> Self {
        Sprite {
            x: 0,
            y: 0,
            width,
            height,
            image: Bitmap::load(&core, path).expect("Failed to create sprite, probably a bad path")
        }
    }
}
