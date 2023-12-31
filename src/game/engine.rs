extern crate allegro;
extern crate allegro_font;

use allegro::*;
use allegro_font::*;
use allegro_image::*;
use std::option::Option;
use std::collections::VecDeque;
use std::vec::Vec;
use crate::game::sprite::Sprite;


pub struct Engine {
    running: bool,
    display: Display,
    timer: Timer,
    font: Font,
    core: Core,
    font_addon: FontAddon,
    queue: EventQueue,
    redraw: bool,
    draw_queue: VecDeque<Sprite>,
    draw_id: usize
}

impl Engine {
    /*
     * Initialises allegro subsystems and saves them as members
     * of the engine struct
     */
    pub fn new(width: i32, height: i32, tick_rate: f64) -> Self {

        // sets up the allegro core subsystems
        let core = Core::init().expect("Failed to initialise allegro core");
        core.install_keyboard().unwrap();
        core.install_mouse().unwrap();
        
        // sets up the font addon
        let font_addon = FontAddon::init(&core).expect("Failed to initialise allegro font");
        
        // initialises the image addon
        ImageAddon::init(&core).expect("Failed to initialise image addon");
        
        // sets up the main window
        let display = Display::new(&core, width, height).expect("Failed to initialise display");
        
        // sets up the timer
        let timer = Timer::new(&core, 1.0 / tick_rate).expect("Failed to initialise timer");
        
        // sets up a default font
        let font = Font::new_builtin(&font_addon).expect("Failed to initialise font");
        
        // sets up the event queue so events can be processed from registered inputs
        let queue = EventQueue::new(&core).expect("Failed to initialise event queue");
        queue.register_event_source(display.get_event_source());
        queue.register_event_source(timer.get_event_source());
        queue.register_event_source(core.get_keyboard_event_source().unwrap());
        queue.register_event_source(core.get_mouse_event_source().unwrap());
        
        // starts the timer so game update frequencies can be managed
        timer.start();
        
        Self {
            running: true,
            core,
            font_addon,
            timer,
            font,
            display,
            queue,
            redraw: true,
            draw_queue: VecDeque::new(),
            draw_id: 0
        }

    }

    /*
     * Manages low leve levents
     */
    #[inline]
    pub fn manage_events(&mut self) -> Option<Event> {
        let event = self.queue.wait_for_event();
        match event {
            DisplayClose{..} => self.running = false,
            TimerTick{..} => self.redraw = true,
            _ => return Some(event),
        }
        return None;
    }

    /*
     * redraws all registered sprites on the screen
     */
    #[inline]
    pub fn redraw(&mut self) -> bool {
        if self.redraw {
            self.redraw = false;
            for sprite in &self.draw_queue {
                self.core.draw_bitmap(&sprite.image, sprite.x as f32, sprite.y as f32, BitmapDrawingFlags::zero());
            }
            self.core.flip_display();
            self.core.clear_to_color(Color::from_rgb(0,0,0));
            return true;
        }
        return false;
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.running
    }

    /*
     * sets the running member to false (is_runnig -> false)
     */
    #[inline]
    pub fn stop(&mut self) {
        self.running = false;
    }

    /*
     * Registers a sprite and returns its index in the sprite deque
     */
    pub fn create_sprite(&mut self, path_to_image: &'static str, width: i32, height: i32) -> usize {
        self.draw_queue.push_back(Sprite::new(&self.core, path_to_image, width, height));
        let res_id = self.draw_id;
        self.draw_id += 1;
        return res_id;
    }

    pub fn check_collisions(&self, id:usize) -> Vec<usize> {
        let result: Vec<usize> = vec![];
        let pof_sprite: &Sprite; 
        match self.draw_queue.get(id) {
            Some(sprite) => pof_sprite = sprite,
            None => return result,
        }
        
        for i in 0..self.draw_queue.len() {
            
        }

        return result;
    }
    
    #[inline]
    pub fn set_sprite_x(&mut self, id: usize, x: i32) {
        self.draw_queue.get_mut(id).unwrap().x = x;
    }

    #[inline]
    pub fn set_sprite_y(&mut self, id: usize, y: i32) {
        self.draw_queue.get_mut(id).unwrap().y = y;
    }

    #[inline]
    pub fn inc_sprite_x(&mut self, id: usize, x: i32) {
        self.draw_queue.get_mut(id).unwrap().x += x;
    }

    #[inline]
    pub fn inc_sprite_y(&mut self, id: usize, y: i32) {
        self.draw_queue.get_mut(id).unwrap().y += y;
    }

    #[inline]
    pub fn set_sprite_width(&mut self, id: usize, width: i32) {
        self.draw_queue.get_mut(id).unwrap().width = width;
    }

    #[inline]
    pub fn set_sprite_height(&mut self, id: usize, height: i32) {
        self.draw_queue.get_mut(id).unwrap().height = height;
    }
}
