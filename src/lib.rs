#![no_std]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;
use agb::{input::ButtonController, println};

mod game;

pub fn main(mut gba: agb::Gba) -> ! {
    let mut game = game::Game::new();
    let mut input = ButtonController::new();
    loop {
        input.update();
        game.update(&input);
    }
}
