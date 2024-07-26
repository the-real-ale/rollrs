use std::io::{stdout, Stdout};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn print_color(str: String, foreground: Color) -> Result<()> {
    print_color_bg(str, foreground, Color::Reset)
}

#[allow(dead_code)]
pub fn print_red(str: String) {
    print_color(str, Color::DarkRed).unwrap_or(());
}

#[allow(dead_code)]
pub fn print_green(str: String) {
    print_color(str, Color::Green).unwrap_or(());
}

#[allow(dead_code)]
pub fn print(str: String) {
    print_color(str, Color::Reset).unwrap_or(());
}

pub fn draw_border(x0: u16, y0: u16, width: u16, height: u16, mut stdout: &Stdout) -> Result<()> {
    // Draw Corners
    queue!(stdout, cursor::MoveTo(x0, y0), Print("┌"))?;
    queue!(stdout, cursor::MoveTo(x0 + width, y0), Print("┐"))?;
    queue!(stdout, cursor::MoveTo(x0, y0 + height), Print("└"))?;
    queue!(stdout, cursor::MoveTo(x0 + width, y0 + height), Print("┘"))?;

    for y in y0 + 1..y0 + height {
        queue!(stdout, cursor::MoveTo(x0, y), Print("│"))?;
        queue!(stdout, cursor::MoveTo(x0 + width, y), Print("│"))?;
    }

    for x in x0 + 1..x0 + width {
        queue!(stdout, cursor::MoveTo(x, y0), Print("─"))?;
        queue!(stdout, cursor::MoveTo(x, y0 + height), Print("─"))?;
    }

    Ok(())
}

pub fn draw_rectangle(x0: u16, y0: u16, width: u16, height: u16, stdout: &Stdout) -> Result<()> {
    for y in y0..y0 + height {
        for x in x0..x0 + width {
            draw_char_at(x, y, ' ', stdout)?;
        }
    }

    Ok(())
}

pub fn draw_solid_rectangle(
    x0: u16,
    y0: u16,
    width: u16,
    height: u16,
    stdout: &Stdout,
) -> Result<()> {
    for y in y0..y0 + height {
        for x in x0..x0 + width {
            draw_char_at(x, y, '█', stdout)?;
        }
    }

    Ok(())
}

pub fn draw_char_at(x: u16, y: u16, c: char, mut stdout: &Stdout) -> Result<()> {
    queue!(stdout, cursor::MoveTo(x, y), Print(c),)?;
    Ok(())
}

pub fn draw_bordered_rec(x0: u16, y0: u16, width: u16, height: u16, stdout: &Stdout) -> Result<()> {
    draw_rectangle(x0 + 1, y0 + 1, width - 1, height - 1, stdout)?;
    draw_border(x0, y0, width, height, stdout)?;
    Ok(())
}

pub fn draw_vertical_line(x0: u16, y0: u16, height: f32, stdout: &Stdout) -> Result<()> {
    draw_solid_rectangle(x0, y0 - (height as u16) + 1, 1, height as u16, stdout)?;
    if height - height.floor() >= 0.125 {
        draw_char_at(
            x0,
            y0 - (height as u16),
            get_vertical_fraction(height - height.floor()),
            stdout,
        )?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn draw_horizontal_line(x0: u16, y0: u16, width: f32, stdout: &Stdout) -> Result<()> {
    draw_solid_rectangle(x0, y0, width.round() as u16, 1, stdout)?;
    Ok(())
}

pub fn reset_cursor(mut stdout: &Stdout) -> Result<()> {
    queue!(stdout, cursor::MoveTo(0, get_height()), ResetColor)?;
    Ok(())
}

pub fn get_height() -> u16 {
    crossterm::terminal::size().unwrap().1
}

pub fn get_width() -> u16 {
    crossterm::terminal::size().unwrap().0
}

pub fn scroll(height: u16) -> Result<()> {
    for _ in 0..height {
        println!("");
    }
    Ok(())
}

pub fn get_vertical_fraction(normalize: f32) -> char {
    if normalize < 0.125 {
        return ' ';
    } else if normalize >= 0.125 && normalize < 0.25 {
        return '▁';
    } else if normalize >= 0.25 && normalize < 0.375 {
        return '▂';
    } else if normalize >= 0.375 && normalize < 0.5 {
        return '▃';
    } else if normalize >= 0.5 && normalize < 0.625 {
        return '▄';
    } else if normalize >= 0.625 && normalize < 0.75 {
        return '▅';
    } else if normalize >= 0.75 && normalize < 0.875 {
        return '▆';
    } else if normalize >= 0.875 && normalize < 0.9315 {
        return '▇';
    } else {
        return '█';
    }
}

pub fn get_horizontal_fraction(normalize: f32) -> char {
    if normalize < 0.125 {
        return ' ';
    } else if normalize >= 0.125 && normalize < 0.25 {
        return '▏';
    } else if normalize >= 0.25 && normalize < 0.375 {
        return '▎';
    } else if normalize >= 0.375 && normalize < 0.5 {
        return '▍';
    } else if normalize >= 0.5 && normalize < 0.625 {
        return '▌';
    } else if normalize >= 0.625 && normalize < 0.75 {
        return '▋';
    } else if normalize >= 0.75 && normalize < 0.875 {
        return '▊';
    } else if normalize >= 0.875 && normalize < 0.9315 {
        return '▉';
    } else {
        return '█';
    }
}
