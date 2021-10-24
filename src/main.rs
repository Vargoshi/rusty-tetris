mod block;
mod board;
mod draw;
mod game;

use draw::{SCREEN_HEIGHT, SCREEN_WIDTH};
use game::Game;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("SDL2", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let timer = sdl_context.timer()?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::Start;
    let mut last_step = std::time::Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            use sdl2::{event::Event, keyboard::Keycode};
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => {
                    game.input(code);
                }
                _ => {}
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        canvas.clear();
        game.draw(&mut canvas)?;
        canvas.present();

        ::std::thread::sleep(std::time::Duration::from_millis(1000 / 30));

        let now = std::time::Instant::now();
        if now > last_step + std::time::Duration::from_millis(1000) {
            last_step = now;
            game.step();
        }
    }

    Ok(())
}
