mod block;
mod board;

use board::Board;
use crossterm::{event, terminal};

fn main() -> crossterm::Result<()> {
    let mut board = Board::new();
    let mut last_step = std::time::Instant::now();

    terminal::enable_raw_mode()?;

    loop {
        board.draw()?;

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
                board.input(code);
            }
        }

        let now = std::time::Instant::now();
        if now > last_step + std::time::Duration::from_millis(1000) {
            last_step = now;
            board.step();
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
