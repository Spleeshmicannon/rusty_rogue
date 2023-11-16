use std::vec::Vec;

/*
 * All objects that can appear in a level.
 */
pub enum LvlObj {
    Wall(i32,i32),
    EnemyBasic(i32,i32),
    Entry(i32,i32),
    Exit(i32,i32),
}

/*
 * Manages level object data
 * and level transitions.
 */
pub struct LevelManager {
    levels:Vec<Vec<LvlObj>>,
    pub level_index: usize
}

impl LevelManager {
    /*
     * include_bytes calls for each
     * level are here (gets them from the levels
     * folder) and then each level is parsed into
     * Vec<LvlObj>
     */
    pub fn new() -> Self {
        let mut levels: Vec<Vec<LvlObj>> = vec![];
        
        let bytes = include_bytes!("../../levels/Room1.lvl");

        levels.push(LevelManager::get_objects(&bytes[..]));

        return LevelManager {
            levels,
            level_index: 0
        };
    }

    /*
     * returns a reference to the objects to be used for
     * the current level being pointed to by the level_index
     */
    #[inline]
    pub fn get_level_object_types(&self) -> &Vec<LvlObj> {
        return &self.levels[self.level_index];
    }

    /*
     * Parses a single level file into a 
     * Vec<LvlObj>, takes the bytes of the file
     */
    fn get_objects(bytes: &[u8]) -> Vec<LvlObj> {
        
        let mut objs:Vec<LvlObj> = vec![];
        let mut x:i32 = 0;
        let mut y:i32 = 0;

        for byte in bytes {
            let char = *byte as char;
            match char {
                '+' => objs.push(LvlObj::Wall(x,y)),
                '=' => objs.push(LvlObj::EnemyBasic(x,y)),
                '@' => objs.push(LvlObj::Entry(x,y)),
                '*' => objs.push(LvlObj::Exit(x,y)),
                _ => {}
            }

            if char != '\n' {
                x += 1;
            } else {
                y += 1;
                x = 0;
            }
        }

        return objs;
    }
}
