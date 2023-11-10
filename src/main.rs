#![warn(clippy::nursery, clippy::pedantic)]
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::{error::Error, time::Duration};

mod utils;
use utils::AlternateBuffer;

fn main() -> Result<(), Box<dyn Error>> {
    let mut alt_buf = AlternateBuffer::new()?;
    loop {
        alt_buf.tick()?;
        if !event::poll(Duration::ZERO)? {
            continue;
        }
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return Ok(()),
            Event::Resize(..) => {
                alt_buf.resize()?;
            }
            _ => {}
        }
    }
}
