use core::cell::RefCell;

use agb::{fixnum::Vector2D, println};
use alloc::{collections::VecDeque, rc::Rc};

use super::{Direction, PositionDirection, TILE_SIZE};

pub struct BodyPart {
    pub direction: Direction,
    pub position: Vector2D<u8>,
}

pub struct Snake {
    pub speed: f32,
    pub direction: Direction,
    pub parts: VecDeque<Rc<RefCell<BodyPart>>>,
}

impl Snake {
    pub fn new(start_direction: Direction, start_position: Vector2D<u8>) -> Self {
        let head = BodyPart {
            direction: start_direction.clone(),
            position: start_position,
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
        println!("map_directions: {:?}", map_directions);
        // Update direction of parts
        for part in self.parts.iter_mut() {
            map_directions
                .iter()
                .filter(|m| part.borrow_mut().position == m.position)
                .for_each(|f| part.borrow_mut().direction = f.direction.clone());

            // Update position
            let mut raw_part = part.borrow_mut();
            raw_part.position = match raw_part.direction {
                Direction::UP => raw_part.position + Vector2D::new(0, 1),
                Direction::DOWN => raw_part.position - Vector2D::new(0, 1),
                Direction::LEFT => raw_part.position - Vector2D::new(1, 0),
                Direction::RIGHT => raw_part.position + Vector2D::new(1, 0),
            }
        }
    }

    pub fn render(&self) {
        let head = self.parts.front().unwrap().borrow();
        println!("HEAD position: {:?}", head.position);
    }
}
