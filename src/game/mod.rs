use agb::display::object::OamIterator;
use agb::fixnum::Vector2D;
use agb::input::{Button, ButtonController};

use agb::println;
use alloc::collections::VecDeque;

use crate::sprite::SpriteCache;

use self::apple::Apple;
use self::snake::Snake;

mod apple;
mod snake;

const MAP_WIDTH: u8 = 30;
const MAP_HEIGHT: u8 = 20;
const TILE_SIZE: u8 = 8;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct PositionDirection {
    direction: Direction,
    position: Vector2D<u8>,
}

impl PositionDirection {
    pub fn new(direction: Direction, position: Vector2D<u8>) -> Self {
        Self {
            direction,
            position,
        }
    }
}

pub struct Game {
    pub directions: VecDeque<PositionDirection>,
    pub snake: Snake,
    pub apple: Option<Apple>,
    pub current_timer: i32,
    pub sprite_ctrl: SpriteCache,
}

impl Game {
    pub fn new(sprite_ctrl: SpriteCache) -> Self {
        let start_position = Vector2D::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        Self {
            directions: VecDeque::new(),
            snake: Snake::new(Direction::RIGHT, start_position, &sprite_ctrl), // TODO: random
            apple: None,
            current_timer: -1,
            sprite_ctrl,
        }
    }

    pub fn update(&mut self, input: &ButtonController, timer: i32) {
        let delta_time = timer - self.current_timer;

        let current_position = self.snake.get_position();
        let direction = self.input_to_direction(&input);
        self.add_direction_on_map(direction.clone(), current_position.clone());

        if let Some(i) = direction {
            self.snake.direction = i.clone();
        }

        if delta_time > 30 { // 30 FPS = 0.5 sec
            self.eat_apple();
            self.snake.update(&mut self.directions);
        }

        if delta_time > 30 {
            self.current_timer = timer;
        }

        if let None = self.apple {
            self.apple = Some(self.create_apple());
        }

    }

    pub fn render(&mut self, oam: &mut OamIterator) {
        self.snake.render(oam);
        if let Some(apple) = &mut self.apple {
            apple.render(oam);
        }
    }

    fn create_apple(&self) -> Apple {
        loop {
            let x = agb::rng::gen() as u8 % MAP_WIDTH;
            let y = agb::rng::gen() as u8 % MAP_HEIGHT;
            let apple_position = Vector2D::new(x, y);
            println!("Apple pos: {:?} {}", apple_position, agb::rng::gen() as u8);
            if !self.snake.is_snake(&apple_position) {
                return Apple::new(apple_position, &self.sprite_ctrl);
            }
        }
    }

    fn eat_apple(&mut self) {
        if let Some(apple) = &self.apple {
            if self.snake.get_position() == apple.position {
                self.snake.grow(&self.sprite_ctrl);
                self.apple = None;
            }
        }
    }

    fn add_direction_on_map(&mut self, direction: Option<Direction>, position: Vector2D<u8>) {
        if let Some(i) = direction {
            match self.directions.front() {
                Some(dir) => {
                    if dir.direction != i {
                        self.directions
                            .push_front(PositionDirection::new(i, position))
                    }
                }
                None => self
                    .directions
                    .push_front(PositionDirection::new(i, position)),
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
