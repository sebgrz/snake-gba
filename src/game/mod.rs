use agb::input::{ButtonController, Button};
use agb::println;
use alloc::vec::{Vec};
use alloc::vec;

mod snake;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub struct Game {
    pub directions: Vec<Direction>
}

impl Game {
    pub fn new() -> Self {
        Self { directions: vec![] }
    }

    pub fn update(&self, input: &ButtonController) {
        if input.is_just_pressed(Button::LEFT) {
            println!("LEFT");
        }
    }
}
