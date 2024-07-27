use std::io::Stdout;

use crossterm::Result;

pub trait Component {
    fn draw(&self, stdout: &Stdout) -> Result<()>;
}
