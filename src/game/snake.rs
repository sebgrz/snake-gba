use agb::fixnum::Vector2D;
use alloc::vec::Vec;

use super::Direction;

struct BodyPart {
    direction: Direction,
    position: Vector2D<u8>,
}

struct Snake {
    direction: Direction,
    parts: Vec<BodyPart>,
}
