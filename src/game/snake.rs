use agb::fixnum::Vector2D;
use alloc::collections::VecDeque;

use super::Direction;

pub struct BodyPart {
    pub direction: Direction,
    pub position: Vector2D<u8>,
}

pub struct Snake {
    pub direction: Direction,
    pub parts: VecDeque<BodyPart>,
}

impl Snake {
    pub fn new(start_direction: Direction, start_position: Vector2D<u8>) -> Self {
        let head = BodyPart {
            direction: start_direction.clone(),
            position: start_position,
        };
        Self {
            parts: VecDeque::from([head]),
            direction: start_direction.clone(),
        }
    }
}
