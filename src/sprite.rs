

use agb::{
    display::{
        object::{ObjectUnmanaged, SpriteLoader, SpriteVram},
    },
    hash_map::HashMap,
};

use crate::{SNAKE_HEAD, APPLE};

#[derive(PartialEq, Eq, Hash)]
pub enum SpriteType {
    APPLE,
    HEAD,

}

pub struct SpriteCache {
    pub sprites: HashMap<SpriteType, SpriteVram>,
}

impl SpriteCache {
    pub fn new(sprite_loader: &mut SpriteLoader) -> Self {
        let mut map = HashMap::new();
        map.insert(
            SpriteType::HEAD,
            sprite_loader.get_vram_sprite(SNAKE_HEAD.sprite(0)),
        );
        map.insert(
            SpriteType::APPLE,
            sprite_loader.get_vram_sprite(APPLE.sprite(0)),
        );
        Self { sprites: map }
    }

    pub fn get_sprite_object(&self, sprite_type: SpriteType) -> ObjectUnmanaged {
        ObjectUnmanaged::new(self.sprites.get(&sprite_type).unwrap().clone())
    }
}
