extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    // 创建窗口
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        // 获取按键
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        // 更新画面
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // 重新开始
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
