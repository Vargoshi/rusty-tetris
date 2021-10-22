mod block;
mod board;
mod draw;
mod game;

use crossterm::{event, terminal};
use game::Game;

fn main() -> crossterm::Result<()> {
    let mut game = Game::Start;
    let mut last_step = std::time::Instant::now();

    terminal::enable_raw_mode()?;

    loop {
        game.draw()?;

        if event::poll(std::time::Duration::from_millis(250))? {
            // Handle keyboard event
            use event::{Event, KeyCode, KeyEvent, KeyModifiers};

            if let Event::Key(KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
            }) = event::read()?
            {
                if let KeyCode::Char('q') = code {
                    break;
                }
                game.input(code);
            }
        }

        let now = std::time::Instant::now();
        if now > last_step + std::time::Duration::from_millis(1000) {
            last_step = now;
            game.step();
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
