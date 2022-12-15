use crate::terminal::Terminal;
use std::io::stdout;
use termion::{event::Key, raw::IntoRawMode};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        // FIX asdfasgasgdasd
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed tio initialize terminal..."),
        }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();

        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            println!("Good bye. \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0)
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                println!("Glenn editor -- version {}/r", VERSION);
            } else {
                println!("~\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{:?}", e);
}
