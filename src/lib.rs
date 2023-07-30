#![no_std]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;


use agb::{
    display::{
        object::{Graphics, Tag},
        palette16::Palette16,
    },
    include_aseprite,
    input::ButtonController,
};

use sprite::SpriteCache;

mod game;
mod sprite;

const SPRITES: &Graphics = include_aseprite!("gfx/snake.aseprite");
const APPLE: &Tag = SPRITES.tags().get("Apple");
const SNAKE_HEAD: &Tag = SPRITES.tags().get("Head");
const SNAKE_TAIL: &Tag = SPRITES.tags().get("Tail");
const SNAKE_BODY: &Tag = SPRITES.tags().get("Body");

pub fn main(mut gba: agb::Gba) -> ! {
    let (_, mut vram) = gba.display.video.tiled0();
    vram.set_background_palettes(&[Palette16::new([u16::MAX; 16])]);
    let (mut object, mut sprite_loader) = gba.display.object.get_unmanaged();

    let vblank = agb::interrupt::VBlank::get();
    let sprite_ctrl = SpriteCache::new(&mut sprite_loader);
    let mut game = game::Game::new(sprite_ctrl);
    let mut input_ctrl = ButtonController::new();
    let mut timer: i32 = 0;

    loop {
        let oam = &mut object.iter();
        input_ctrl.update();
        game.update(&input_ctrl, timer);
        game.render(oam);
        vblank.wait_for_vblank();
        timer += 1;
    }
}
