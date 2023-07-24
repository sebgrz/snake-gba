use core::cell::RefCell;

use agb::{
    display::object::{OamIterator, ObjectUnmanaged},
    fixnum::Vector2D,
};
use alloc::{collections::VecDeque, rc::Rc};

use crate::sprite::SpriteCache;

use super::{Direction, PositionDirection, TILE_SIZE};

pub struct BodyPart {
    pub direction: Direction,
    pub position: Vector2D<u8>,
    pub sprite_obj: ObjectUnmanaged,
}

impl BodyPart {
    pub fn update(&mut self) {
        let position_on_map = Vector2D::new(
            (self.position.x * TILE_SIZE) as i32,
            (self.position.y * TILE_SIZE) as i32,
        );
        self.sprite_obj.set_position(position_on_map);
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
        let head = BodyPart {
            direction: start_direction.clone(),
            position: start_position,
            sprite_obj: sprite_cache.get_sprite_object(crate::sprite::SpriteType::HEAD),
        };
        Self {
            speed: 0.4,
            parts: VecDeque::from([Rc::new(RefCell::new(head))]),
            direction: start_direction.clone(),
        }
    }

    pub fn get_position(&self) -> Vector2D<u8> {
        self.parts.front().unwrap().borrow().position
    }

    pub fn update(&mut self, map_directions: &VecDeque<PositionDirection>) {
        // println!("map_directions: {:?}", map_directions);
        // Update direction of parts
        for part in self.parts.iter_mut() {
            map_directions
                .iter()
                .filter(|m| part.borrow_mut().position == m.position)
                .for_each(|f| part.borrow_mut().direction = f.direction.clone());

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
