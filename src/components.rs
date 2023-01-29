use std::{io::{Write, Stdout}, borrow::Cow};

use crossterm::{Result, queue, cursor, style::{Print}};

use crate::drawterm::{self};

pub struct ComponentData{
    pub height: u16,
    pub width: u16,
    pub position: (u16, u16),
}

impl ComponentData{
    pub fn new(x0: u16, y0: u16, width: u16, height: u16) -> Self{
        let new: Self = Self{
            height: height,
            width: width,
            position: (x0, y0)
        };
        new
    }
}

impl Clone for ComponentData {
    fn clone(&self) -> Self {
        Self { height: self.height.clone(), width: self.width.clone(), position: self.position.clone() }
    }
}

pub struct Hist1D{
    show_border:bool,
    show_value:bool,
    bins: u16,
    data: Vec<f32>,
    limits: (f32, f32),
    component: ComponentData
}

impl Hist1D{
    #[allow(dead_code)]
    pub fn new(bins: u16, limits: (f32, f32), component: ComponentData, show_value: bool, show_border: bool) -> Self{
        let new: Self = Self{
            show_border,
            show_value,
            bins,
            limits,
            data: vec![0.0; bins as usize],
            component
        };
        new
    }

    fn draw_bars(&self, stdout: &Stdout) -> Result<()>{
        let bar_width = ((self.get_width() - 3) as f32 / self.bins as f32).round() as u16;
        let height_factor = (self.get_height() - 1) as f32 / Self::get_max(self.get_data());
        let x0 = self.get_position().0 + 1;
        let y0 = self.get_position().1 + self.get_height() - 1;
        for x in 0 .. self.bins {
            let value: f32 = self.get_data()[x as usize] * height_factor;
            for delta in 0 .. bar_width {
                drawterm::draw_vertical_line((x * bar_width) + delta + x0, y0, value, stdout)?;
            }
        }

        Ok(())
    }

    fn draw_border(&self, stdout: &Stdout) -> Result<()> {
        if self.show_border{
            drawterm::draw_bordered_rec(
                self.get_position().0, 
                self.get_position().1, 
                self.get_width(), 
                self.get_height(), 
                stdout)?;
        }
        else {
            drawterm::draw_rectangle(
                self.get_position().0, 
                self.get_position().1, 
                self.get_width(), 
                self.get_height(), 
                stdout)?;
        }
        Ok(())
    }

    fn draw_horizontal_values(&self, stdout: &Stdout) -> Result<()>{
        if self.show_value {
            let x0 = self.get_position().0;
            let y0 = self.get_position().1 + self.get_height();
            for x in x0 + 1..x0 + self.get_width(){
                if x % 5 == 0{
                    drawterm::draw_char_at(x, y0, '┬', stdout)?;
                }
            }
        }
        Ok(())
    }

    fn draw_vertical_values(&self, stdout: &Stdout) -> Result<()>{
        if self.show_value {
            let x0 = self.get_position().0;
            let y0 = self.get_position().1;
            for y in y0 + 1..y0 + self.get_height(){
                drawterm::draw_char_at(x0, y, '┤', stdout)?;
            }
        }
        Ok(())
    }

    fn get_max(vec: &Vec<f32>) -> f32{
        let mut max = f32::NEG_INFINITY;
        for i in vec {
            if *i > max {
                max = *i;
            }
        }
        max
    }
}

pub trait Component{
    fn draw(&self, stdout: &Stdout) -> Result<()>;
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_position(&self) -> (u16, u16);
    fn set_width(&mut self, width: u16);
    fn set_height(&mut self, height: u16);
    fn set_position(&mut self, pos: (u16, u16));
}

pub trait Graph1D{
    fn get_data(&self) -> &Vec<f32>;
    fn get_bins(&self) -> u16;
    fn get_limits(&self) -> (f32, f32);
    fn get_value(&self, x: f32) -> f32;
    fn set_data(&mut self, data: Vec<f32>);
    fn set_bins(&mut self, bins: u16);
    fn set_bin(&mut self, bin: u16, value: f32);
    fn set_limits(&mut self, limits: (f32, f32));
    fn set_value(&mut self, x: f32, value: f32);
}

impl Graph1D for Hist1D{
    fn get_data(&self) -> &Vec<f32>{
        &self.data
    }
    fn get_bins(&self) -> u16{
        self.bins
    }
    fn get_limits(&self) -> (f32, f32){
        self.limits
    }
    fn get_value(&self, x: f32) -> f32{
        let width = (self.get_limits().1 - self.get_limits().0) / self.get_bins() as f32;
        self.get_data()[((x - self.get_limits().0) / width).round() as usize]
    }
    fn set_data(&mut self, data: Vec<f32>){
        self.data = data;
    }
    fn set_bins(&mut self, bins: u16){
        self.set_data(vec![0.0; bins as usize]);
        self.bins = bins;
    }
    fn set_bin(&mut self, bin: u16, value: f32){
        let index = if bin >= self.data.len() as u16 { self.data.len() - 1 } else { bin as usize };
        self.data[index] = value;
    }
    fn set_limits(&mut self, limits: (f32, f32)){
        self.limits = limits;
    }
    fn set_value(&mut self, x: f32, value: f32){
        let width = (self.get_limits().1 - self.get_limits().0) / self.get_bins() as f32;
        let index = (((x - self.get_limits().0) / width).ceil()) - 1.0;
        self.set_bin(index as u16, value);
    }
}

impl Component for Hist1D{
    fn draw(&self, mut stdout: &Stdout) -> Result<()>{
        drawterm::scroll(self.get_height() + 1)?;
        self.draw_border(stdout)?;
        self.draw_bars(stdout)?;
        self.draw_horizontal_values(stdout)?;
        self.draw_vertical_values(stdout)?;
        drawterm::reset_cursor(stdout)?;
        stdout.flush()?;
        Ok(())
    }
    fn get_width(&self) -> u16{
        self.component.width
    }
    fn get_height(&self) -> u16{
        self.component.height
    }
    fn get_position(&self) -> (u16, u16){
        self.component.position
    }
    fn set_width(&mut self, width: u16){
        self.component.width = width;
    }
    fn set_height(&mut self, height: u16){
        self.component.height = height;
    }
    fn set_position(&mut self, pos: (u16, u16)){
        self.component.position = pos;
    }
}

pub struct TextBox {
    text: String,
    show_border: bool,
    component: ComponentData
}

impl TextBox {
    pub fn new<T>(component: ComponentData, text: T, show_border: bool) -> Self where T: ToString{
        Self {text: text.to_string(), show_border, component}
    }

    fn draw_border(&self, stdout: &Stdout) -> Result<()> {
        if self.show_border{
            drawterm::draw_bordered_rec(
                self.get_position().0, 
                self.get_position().1, 
                self.get_width(), 
                self.get_height(), 
                stdout)?;
        }
        else {
            drawterm::draw_rectangle(
                self.get_position().0, 
                self.get_position().1, 
                self.get_width(), 
                self.get_height(), 
                stdout)?;
        }
        Ok(())
    }
    
    fn draw_formatted_lines(&self, mut stdout: &Stdout, lines: &Vec<Cow<'_, str>>) -> Result<()> {
        let mut y = 1;
        queue!(stdout,
            cursor::MoveTo(self.get_position().0 + 1, self.get_position().1 + y)
        )?;
        for line in lines {
            queue!(stdout, 
                Print(textwrap::indent(line, " ").to_string()),
                cursor::MoveTo(self.get_position().0 + 1, self.get_position().1 + y))?;
            y += 1;
        }
        Ok(())
    }
}

impl Component for TextBox {
    fn draw(&self, mut stdout: &Stdout) -> Result<()> {
        drawterm::scroll(self.get_height() + 1)?;
        self.draw_border(stdout)?;
        self.draw_formatted_lines(stdout, &textwrap::wrap(&self.text, self.get_width() as usize - 3))?;
        drawterm::reset_cursor(stdout)?;
        stdout.flush()?;
        drawterm::scroll(1)?;
        Ok(())
    }

    fn get_width(&self) -> u16 {
        self.component.width
    }

    fn get_height(&self) -> u16 {
        self.component.height
    }

    fn get_position(&self) -> (u16, u16) {
        self.component.position
    }

    fn set_width(&mut self, width: u16) {
        self.component.width = width;
    }

    fn set_height(&mut self, height: u16) {
        self.component.height = height;
    }

    fn set_position(&mut self, pos: (u16, u16)) {
        self.component.position = pos;
    }
}