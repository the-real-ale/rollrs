use std::io::stdout;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};

pub fn print_color_bg(str: String, foreground: Color, background: Color) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(foreground),
        SetBackgroundColor(background),
        Print(str),
        ResetColor
    )?;
    Ok(())
}

pub fn print_color(str: String, foreground: Color) -> Result<()> {
    print_color_bg(str, foreground, Color::Reset)
}

pub fn print_red(str: String) {
    print_color(str, Color::DarkRed).unwrap_or(());
}

pub fn print_green(str: String) {
    print_color(str, Color::Green).unwrap_or(());
}

pub fn print(str: String) {
    print_color(str, Color::Reset).unwrap_or(());
}

pub fn get_width() -> u16 {
    crossterm::terminal::size().unwrap().0
}

pub fn get_horizontal_fraction(normalize: f32) -> char {
    if normalize < 0.125 {
        ' '
    } else if (0.125..0.25).contains(&normalize) {
        return '▏';
    } else if (0.25..0.375).contains(&normalize) {
        return '▎';
    } else if (0.375..0.5).contains(&normalize) {
        return '▍';
    } else if (0.5..0.625).contains(&normalize) {
        return '▌';
    } else if (0.625..0.75).contains(&normalize) {
        return '▋';
    } else if (0.75..0.875).contains(&normalize) {
        return '▊';
    } else if (0.875..0.9315).contains(&normalize) {
        return '▉';
    } else {
        return '█';
    }
}
