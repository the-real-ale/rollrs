use std::io::Stdout;

use crossterm::Result;

pub trait Component {
    fn draw(&self, stdout: &Stdout) -> Result<()>;
}

pub trait Graph1D {
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
