use agb::{display::object::{ObjectUnmanaged, OamIterator}, fixnum::Vector2D};

use crate::sprite::SpriteCache;

use super::TILE_SIZE;

pub struct Apple {
    pub position: Vector2D<u8>,
    pub sprite_obj: ObjectUnmanaged,
}

impl Apple {
    pub fn new(position: Vector2D<u8>, sprite_cache: &SpriteCache) -> Self {
        Self {
            position,
            sprite_obj: sprite_cache
                .get_sprite_object(crate::sprite::SpriteType::APPLE)
                .set_position(Vector2D::new((position.x  * TILE_SIZE) as i32, (position.y * TILE_SIZE) as i32))
                .clone(),
        }
    }

    pub fn render(&mut self, oam: &mut OamIterator) {
        oam.next().unwrap().set(&self.sprite_obj.show());
    }
}
