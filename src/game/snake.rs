

use agb::{
    display::object::{OamIterator, ObjectUnmanaged},
    fixnum::Vector2D,
};
use alloc::{collections::VecDeque};

use crate::sprite::{SpriteCache, SpriteType};

use super::{Direction, PositionDirection, TILE_SIZE};

#[derive(Debug)]
pub struct BodyPart {
    pub direction: Direction,
    pub position: Vector2D<u8>,
    pub sprite_obj: ObjectUnmanaged,
}

impl BodyPart {
    pub fn new(
        direction: Direction,
        position: Vector2D<u8>,
        r#type: SpriteType,
        sprite_cache: &SpriteCache,
    ) -> Self {
        Self {
            direction,
            position,
            sprite_obj: sprite_cache.get_sprite_object(r#type),
        }
    }

    pub fn update(&mut self) {
        let position_on_map = Vector2D::new(
            (self.position.x * TILE_SIZE) as i32,
            (self.position.y * TILE_SIZE) as i32,
        );
        self.sprite_obj.set_position(position_on_map);
    }

    pub fn change_sprite(&mut self, r#type: SpriteType, sprite_cache: &SpriteCache) {
        self.sprite_obj = sprite_cache.get_sprite_object(r#type);
    }
}

pub struct Snake {
    pub speed: f32,
    pub direction: Direction,
    pub parts: VecDeque<BodyPart>,
}

impl Snake {
    pub fn new(
        start_direction: Direction,
        start_position: Vector2D<u8>,
        sprite_cache: &SpriteCache,
    ) -> Self {
        let head = BodyPart::new(
            start_direction.clone(),
            start_position,
            crate::sprite::SpriteType::HEAD,
            sprite_cache,
        );
        Self {
            speed: 0.4,
            parts: VecDeque::from([head]),
            direction: start_direction.clone(),
        }
    }

    pub fn get_position(&self) -> Vector2D<u8> {
        self.parts.front().unwrap().position
    }

    pub fn is_snake(&self, position: &Vector2D<u8>) -> bool {
        for part in self.parts.iter() {
            if part.position == *position {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self, sprite_cache: &SpriteCache) {
        let (direction, new_position) = {
            let last_part = self.parts.back().unwrap();
            (
                last_part.direction.clone(),
                match last_part.direction {
                    Direction::UP => last_part.position + Vector2D::new(0, 1),
                    Direction::DOWN => last_part.position - Vector2D::new(0, 1),
                    Direction::LEFT => last_part.position + Vector2D::new(1, 0),
                    Direction::RIGHT => last_part.position - Vector2D::new(1, 0),
                },
            )
        };

        let new_part = BodyPart::new(
            direction.clone(),
            new_position,
            SpriteType::TAIL,
            sprite_cache,
        );
        match self.parts.len() {
            1 => self.parts.push_back(new_part),
            _ => {
                self.parts.back_mut().unwrap().change_sprite(SpriteType::BODY, sprite_cache);
                self.parts.push_back(new_part);
            }
        };
    }

    pub fn update(&mut self, map_directions: &mut VecDeque<PositionDirection>) {
        // println!("map_directions: {:?}", map_directions);
        // Update direction of parts
        let parts_len = self.parts.len();
        for (i, part) in self.parts.iter_mut().enumerate() {
            map_directions
                .iter()
                .filter(|m| part.position == m.position)
                .for_each(|f| part.direction = f.direction.clone());

            // Remove last map direction - if snake tail touch it
            if i == parts_len - 1 {
                if let Some(last_map_direction) = map_directions.back() {
                    if part.position == last_map_direction.position {
                        map_directions.pop_back();
                    }
                }
            }

            // Update position
            part.position = match part.direction {
                Direction::UP => part.position - Vector2D::new(0, 1),
                Direction::DOWN => part.position + Vector2D::new(0, 1),
                Direction::LEFT => part.position - Vector2D::new(1, 0),
                Direction::RIGHT => part.position + Vector2D::new(1, 0),
            };
            part.update();
        }
    }

    pub fn render(&mut self, oam: &mut OamIterator) {
        for part in self.parts.iter_mut() {
            oam.next().unwrap().set(&part.sprite_obj.show());
        }
    }
}
