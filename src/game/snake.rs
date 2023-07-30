use core::cell::RefCell;

use agb::{
    display::object::{OamIterator, ObjectUnmanaged},
    fixnum::Vector2D,
    println,
};
use alloc::{collections::VecDeque, rc::Rc};

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
    pub parts: VecDeque<Rc<RefCell<BodyPart>>>,
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
            parts: VecDeque::from([Rc::new(RefCell::new(head))]),
            direction: start_direction.clone(),
        }
    }

    pub fn get_position(&self) -> Vector2D<u8> {
        self.parts.front().unwrap().borrow().position
    }

    pub fn is_snake(&self, position: &Vector2D<u8>) -> bool {
        for part in self.parts.iter() {
            if part.borrow().position == *position {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self, sprite_cache: &SpriteCache) {
        let (direction, new_position) = {
            let last_part = self.parts.back().unwrap().borrow();
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
            1 => self.parts.push_back(Rc::new(RefCell::new(new_part))),
            _ => {
                self.parts
                    .back()
                    .unwrap()
                    .borrow_mut()
                    .change_sprite(SpriteType::BODY, sprite_cache);
                self.parts.push_back(Rc::new(RefCell::new(new_part)));
            }
        };
    }

    pub fn update(&mut self, map_directions: &mut VecDeque<PositionDirection>) {
        // println!("map_directions: {:?}", map_directions);
        // Update direction of parts
        let parts = &mut self.parts;
        for (i, part) in parts.clone().iter_mut().enumerate() {
            map_directions
                .iter()
                .filter(|m| part.borrow().position == m.position)
                .for_each(|f| part.borrow_mut().direction = f.direction.clone());

            // Remove last map direction - if snake tail touch it
            if i == parts.len() - 1 {
                if let Some(last_map_direction) = map_directions.back() {
                    if part.borrow().position == last_map_direction.position {
                        map_directions.pop_back();
                    }
                }
            }

            // Update position
            let borrow_part = &mut part.borrow_mut();
            borrow_part.position = match borrow_part.direction {
                Direction::UP => borrow_part.position - Vector2D::new(0, 1),
                Direction::DOWN => borrow_part.position + Vector2D::new(0, 1),
                Direction::LEFT => borrow_part.position - Vector2D::new(1, 0),
                Direction::RIGHT => borrow_part.position + Vector2D::new(1, 0),
            };

            borrow_part.update();
        }
    }

    pub fn render(&mut self, oam: &mut OamIterator) {
        for part in self.parts.iter_mut() {
            let mut borrow_part = part.borrow_mut();
            oam.next().unwrap().set(&borrow_part.sprite_obj.show());
        }
    }
}
