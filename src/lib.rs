#![no_std]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;
use agb::input::ButtonController;

mod game;

pub fn main(_gba: agb::Gba) -> ! {
    let vblank = agb::interrupt::VBlank::get();
    let mut game = game::Game::new();
    let mut input = ButtonController::new();
    let mut timer: i32 = 0;
    loop {
        input.update();
        game.update(&input, timer);
        vblank.wait_for_vblank();
        timer += 1;
    }
}
