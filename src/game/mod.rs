use agb::fixnum::Vector2D;
use agb::input::{Button, ButtonController};
use alloc::collections::VecDeque;

use self::snake::Snake;

mod snake;

const MAP_WIDTH: u8 = 30;
const MAP_HEIGHT: u8 = 20;

#[derive(PartialEq, Eq, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Game {
    pub directions: VecDeque<Direction>,
    pub snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        let start_position = Vector2D::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);

        Self {
            directions: VecDeque::new(),
            snake: Snake::new(Direction::RIGHT, start_position), // TODO: random
        }
    }

    pub fn update(&mut self, input: &ButtonController) {
        let direction = self.input_to_direction(&input);
        self.add_direction_on_map(direction);
    }

    fn add_direction_on_map(&mut self, direction: Option<Direction>) {
        if let Some(i) = direction {
            match self.directions.front() {
                Some(dir) => {
                    if dir != &i {
                        self.directions.push_front(i)
                    }
                }
                None => self.directions.push_front(i),
            }
        }
    }

    fn input_to_direction(&self, input: &ButtonController) -> Option<Direction> {
        if input.is_just_pressed(Button::LEFT) {
            Some(Direction::LEFT)
        } else if input.is_just_pressed(Button::RIGHT) {
            Some(Direction::RIGHT)
        } else if input.is_just_pressed(Button::UP) {
            Some(Direction::UP)
        } else if input.is_just_pressed(Button::DOWN) {
            Some(Direction::DOWN)
        } else {
            None
        }
    }
}
