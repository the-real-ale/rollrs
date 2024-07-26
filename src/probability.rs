use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::Stdout,
    ops::{self, MulAssign},
};

use crossterm::style::Stylize;
use debug::debugln;
use itertools::Itertools;

use crate::{
    components::{Component, ComponentData, Hist1D, TextBox},
    drawterm::get_horizontal_fraction,
    roll::DiceGroup,
};

#[derive(Debug)]
pub struct Polynomial {
    coefficients: HashMap<u16, f64>,
}

impl Polynomial {
    pub fn new() -> Self {
        Self {
            coefficients: HashMap::new(),
        }
    }

    pub fn get_coefficient(&self, exponent: u16) -> f64 {
        *self.coefficients.get(&exponent).unwrap_or(&0.0)
    }

    pub fn get_coefficients(&self) -> &HashMap<u16, f64> {
        &self.coefficients
    }

    pub fn set_coefficient(&mut self, exponent: u16, value: f64) {
        self.coefficients.insert(exponent, value);
    }

    pub fn pow(&self, exponent: u16) -> Polynomial {
        let copy = self.clone();
        let mut result = self.clone();
        if exponent != 0 {
            for _ in 0..exponent - 1 {
                result.mul_assign(copy.clone());
            }
        } else {
            result = Polynomial::new();
        }
        result
    }
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Self {
            coefficients: self.coefficients.clone(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_coefficients().fmt(f)
    }
}

impl ops::AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        let mut new = Polynomial::new();
        for i in self.get_coefficients().keys() {
            new.set_coefficient(*i, self.get_coefficient(*i) + rhs.get_coefficient(*i));
        }
        for i in rhs.get_coefficients().keys() {
            new.set_coefficient(*i, self.get_coefficient(*i) + rhs.get_coefficient(*i));
        }
        self.clone_from(&new);
    }
}

impl ops::Add for Polynomial {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut p = self.clone();
        p += rhs;
        p
    }
}

impl ops::SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        let mut new = Polynomial::new();
        for i in self.get_coefficients().keys() {
            new.set_coefficient(*i, self.get_coefficient(*i) - rhs.get_coefficient(*i));
        }
        for i in rhs.get_coefficients().keys() {
            new.set_coefficient(*i, self.get_coefficient(*i) - rhs.get_coefficient(*i));
        }
        self.clone_from(&new);
    }
}

impl ops::Sub for Polynomial {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut p = self.clone();
        p -= rhs;
        p
    }
}

impl ops::MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        let mut zeroed = Polynomial::new();
        for i in self.get_coefficients().keys() {
            for j in rhs.get_coefficients().keys() {
                let cprod = self.get_coefficient(*i) * rhs.get_coefficient(*j);
                if cprod != 0.0 {
                    zeroed.set_coefficient(*i + *j, zeroed.get_coefficient(*i + *j) + cprod);
                }
            }
        }
        self.clone_from(&zeroed);
    }
}

impl ops::Mul for Polynomial {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut p = self.clone();
        p *= rhs;
        p
    }
}

impl PartialEq<Polynomial> for Polynomial {
    fn eq(&self, other: &Polynomial) -> bool {
        self.coefficients == other.coefficients
    }
}

pub trait Probability {
    fn from_dice(dice: &DiceGroup) -> Self;
    fn get_mean(&self) -> u16;
    fn get_mean_probability(&self) -> f64;
    fn get_probability_of(&self, value: u16) -> f64;
    fn get_probability_of_gt(&self, value: u16) -> f64;
    fn to_data(&self) -> Vec<(f32, f32)>;
}

pub struct Total {
    polynomial: Polynomial,
    dice: DiceGroup,
}

pub struct TotalGraph {
    total: Total,
    component: ComponentData,
    show_border: bool,
}

impl TotalGraph {
    pub fn new(total: Total, component: ComponentData, show_border: bool) -> Self {
        let mut newcomponent = component.clone();
        while Self::to_chart_height(newcomponent.height) < 32 {
            newcomponent.height += 1;
        }
        Self {
            total,
            component: newcomponent,
            show_border,
        }
    }

    fn to_chart_height(rows: u16) -> u32 {
        (rows as u32 - 2) * 4
    }
}

impl Probability for Total {
    fn from_dice(dice: &DiceGroup) -> Self {
        let num = dice.get_count();
        let sides = dice.get_sides().unwrap_or(1);
        let mut poly = Polynomial::new();
        for i in 1..=sides {
            poly.set_coefficient(i, 1.0 / sides as f64);
        }
        poly = poly.pow(num);
        Total {
            polynomial: poly,
            dice: dice.clone(),
        }
    }

    fn get_mean(&self) -> u16 {
        todo!()
    }

    fn get_mean_probability(&self) -> f64 {
        todo!()
    }

    fn get_probability_of(&self, value: u16) -> f64 {
        self.polynomial.get_coefficient(value)
    }

    fn get_probability_of_gt(&self, value: u16) -> f64 {
        self.polynomial
            .get_coefficients()
            .keys()
            .sorted() // TODO: This 'sorted' may be unnecesary
            .filter(|i| **i >= value)
            .fold(0.0, |total, i| total + self.get_probability_of(*i))
    }

    fn to_data(&self) -> Vec<(f32, f32)> {
        let mut vec = vec![];
        vec.push((
            (self.dice.get_count() + self.dice.get_total_modifier() - 1) as f32,
            0.,
        ));
        for entry in self.polynomial.get_coefficients().keys().sorted() {
            vec.push((
                (self.dice.get_total_modifier() + *entry) as f32,
                100. * self.get_probability_of(*entry) as f32,
            ));
        }
        vec.clone()
    }
}

impl Component for TotalGraph {
    fn draw(&self, stdout: &Stdout) -> crossterm::Result<()> {
        let data = self.total.to_data();
        debugln!("Data: {:?}", data);
        let text = Hist1D::new(
            data.len() as u16,
            (0., data.len() as f32),
            self.component.clone(),
            true,
            true,
        );
        // let text = Chart::new_with_y_range((self.get_width() as u32 - 8) * 2,
        //                                 Self::to_chart_height(self.get_height()),
        //                                 data[0].0/* - 1.*/,
        //                                 data[data.len() - 1].0 + 1.,
        //                                 0.,
        //                                 data.get_max() * 1.15)
        //     .lineplot(&textplots::Shape::Bars(data.as_slice()))
        //     .to_string();
        // let databox = TextBox::new(self.component.clone(), text, self.show_border);
        text.draw(stdout)?;
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

pub struct Hits {
    data: HashMap<u16, f64>,
}

pub struct HitsGraph {
    hits: Hits,
    component: ComponentData,
    show_border: bool,
}

impl HitsGraph {
    pub fn new(hits: Hits, component: ComponentData, show_border: bool) -> Self {
        let mut newcomponent = component.clone();
        while Self::to_chart_height(newcomponent.height) < 32 {
            newcomponent.height += 1;
        }
        Self {
            hits,
            component: newcomponent,
            show_border,
        }
    }

    fn to_chart_height(rows: u16) -> u32 {
        (rows as u32 - 2) * 4
    }

    fn get_horizontal_bar(value: f32, width: usize) -> Vec<char> {
        let mut result = vec!['█'; width];
        result[width] = get_horizontal_fraction(value % (width - 1) as f32);
        result
    }
}

impl Hits {
    // For n dice of r sides with success on s sides,
    // prob(x successes) = (n choose x)*((s/r)^x)*((r-s)/r)^(n-x)
    fn create_data(dice: &DiceGroup) -> HashMap<u16, f64> {
        let mut data = HashMap::new();
        let sides = dice.get_sides().unwrap_or(1);
        let success_sides = if dice.get_hit() <= sides {
            sides - (dice.get_hit() - 1)
        } else {
            0
        };
        let n = dice.get_count();
        for x in 0..=dice.get_count() {
            let mut coeff: f64 = 1.;

            for i in dice.get_count() - x + 1..=dice.get_count() {
                coeff *= i as f64;
            }
            for i in 1..=x {
                coeff /= i as f64;
            }

            let dice_factor =
                ((sides - success_sides) as f64 / sides as f64).powi(n as i32 - x as i32);
            let succ_factor = (success_sides as f64 / sides as f64).powi(x as i32);
            let value: f64 = coeff * succ_factor * dice_factor;
            data.insert(x, value);
        }
        data
    }
}

impl Probability for Hits {
    fn from_dice(dice: &DiceGroup) -> Self {
        Self {
            data: Self::create_data(dice),
        }
    }

    fn get_mean(&self) -> u16 {
        todo!()
    }

    fn get_mean_probability(&self) -> f64 {
        todo!()
    }

    fn get_probability_of(&self, value: u16) -> f64 {
        *self.data.get(&value).unwrap_or(&0.)
    }

    fn get_probability_of_gt(&self, value: u16) -> f64 {
        let mut total = 0.0;
        for i in self.data.keys().sorted() {
            if *i >= value {
                total += self.get_probability_of(*i);
            }
        }
        total
    }

    fn to_data(&self) -> Vec<(f32, f32)> {
        let mut vec = vec![];
        // vec.push((-1., 0.));
        for entry in self.data.keys().sorted() {
            vec.push((*entry as f32, 100. * self.get_probability_of(*entry) as f32));
        }
        vec
    }
}

impl Component for HitsGraph {
    fn draw(&self, stdout: &Stdout) -> crossterm::Result<()> {
        let data = self.hits.to_data();
        debugln!("Data: {:?}", data);
        debugln!("Data max: {:?}", data.get_max());
        data.iter().for_each(|i| {
            println!(
                "{}:\t{} {}",
                i.0,
                i.1,
                Self::get_horizontal_bar(
                    i.1,
                    termsize::get()
                        .unwrap_or(termsize::Size { rows: 0, cols: 0 })
                        .cols
                        .into()
                )
                .iter()
                .collect::<String>()
            )
        });
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

pub struct SummaryDisplay {
    text: String,
    component: ComponentData,
    show_border: bool,
}

impl SummaryDisplay {
    pub fn new(
        component: ComponentData,
        dice: &DiceGroup,
        hitnum: Option<u16>,
        totalnum: Option<u16>,
        show_border: bool,
    ) -> Self {
        let hits = hitnum.unwrap_or(u16::MAX);
        let total = totalnum.unwrap_or(u16::MAX);
        let mut newcomponent = component.clone();
        let hitsummary = Hits::from_dice(dice);
        let totalsummary = Total::from_dice(dice);
        let glitchdice = DiceGroup::new(dice.dice.clone(), dice.get_sides().unwrap_or(u16::MAX));
        let glitchsummary = Hits::from_dice(&glitchdice);
        let successchance_hit = hitsummary.get_probability_of_gt(hits);
        let successchance_total = totalsummary.get_probability_of_gt(total);
        let glitchchance =
            glitchsummary.get_probability_of_gt((dice.get_count() as f32 / 2.).round() as u16);
        let critglitchchance = (1.0 - successchance_hit) * glitchchance;
        let success_hit: String = format!("{:7.4}", successchance_hit as f32 * 100.);
        let success_total: String = format!("{:7.4}", successchance_total as f32 * 100.);
        let glitch: String = format!("{:7.4}", glitchchance as f32 * 100.);
        let critglitch: String = format!("{:7.4}", critglitchchance as f32 * 100.);
        let text: String;
        if hits != u16::MAX && total != u16::MAX {
            newcomponent.position.1 -= 1;
            newcomponent.height += 1;
            text = format!("\nProbability of {} total:\t\t{}%\nProbability of {} hits:\t\t{}%\nProbability of glitch:\t\t{}%\nProbability of critical glitch:\t{}%",
                total,
                success_total.bold(),
                hits,
                success_hit.bold(),
                glitch.bold().dark_yellow(),
                critglitch.bold().dark_red());
        } else if hits != u16::MAX {
            text = format!("\nProbability of success:\t\t{}%\nProbability of glitch:\t\t{}%\nProbability of critical glitch:\t{}%",
                success_hit.bold(),
                glitch.bold().dark_yellow(),
                critglitch.bold().dark_red());
        } else if total != u16::MAX {
            text = format!("\nProbability of success:\t\t{}%\nProbability of glitch:\t\t{}%\nProbability of critical glitch:\t{}%",
                success_total.bold(),
                glitch.bold().dark_yellow(),
                critglitch.bold().dark_red());
        } else {
            newcomponent.position.1 += 1;
            newcomponent.height -= 1;
            text = format!(
                "\nProbability of glitch:\t\t{}%\nProbability of critical glitch:\t{}%",
                glitch.bold().dark_yellow(),
                critglitch.bold().dark_red()
            );
        }
        Self {
            text,
            component: newcomponent,
            show_border,
        }
    }
}

impl Component for SummaryDisplay {
    fn draw(&self, stdout: &Stdout) -> crossterm::Result<()> {
        let databox = TextBox::new(self.component.clone(), self.text.clone(), self.show_border);
        databox.draw(stdout)?;
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

trait MinMax<T> {
    fn get_min(&self) -> T;
    fn get_max(&self) -> T;
}

impl MinMax<f32> for Vec<(f32, f32)> {
    fn get_min(&self) -> f32 {
        let mut low = f32::MAX;
        for entry in self {
            if entry.1 < low {
                low = entry.1;
            }
        }
        low
    }

    fn get_max(&self) -> f32 {
        let mut high = f32::MIN;
        for entry in self {
            if entry.1 > high {
                high = entry.1;
            }
        }
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min() {
        let a: Vec<(f32, f32)> = vec![(1., 1.), (2., 3.), (4., 2.)];
        assert_eq!(a.get_min(), 1.);
    }

    #[test]
    fn test_get_max() {
        let a: Vec<(f32, f32)> = vec![(1., 1.), (2., 3.), (4., 2.)];
        assert_eq!(a.get_max(), 3.);
    }

    #[test]
    fn test_polynomial_add() {
        let mut p1 = Polynomial::new();
        let mut p2 = Polynomial::new();
        let mut p3 = Polynomial::new();
        p1.set_coefficient(0, 1.);
        p2.set_coefficient(0, 0.);
        p2.set_coefficient(1, 2.);
        p2.set_coefficient(2, 4.);
        p3.set_coefficient(0, 1.);
        p3.set_coefficient(1, 2.);
        p3.set_coefficient(2, 4.);

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_polynomial_sub() {
        let mut p1 = Polynomial::new();
        let mut p2 = Polynomial::new();
        let mut p3 = Polynomial::new();
        p1.set_coefficient(0, 1.);
        p2.set_coefficient(0, 0.);
        p2.set_coefficient(1, 2.);
        p2.set_coefficient(2, 4.);
        p3.set_coefficient(0, 1.);
        p3.set_coefficient(1, -2.);
        p3.set_coefficient(2, -4.);

        assert_eq!(p1 - p2, p3);
    }

    #[test]
    fn test_polynomial_mul() {
        let mut p1 = Polynomial::new();
        let mut p2 = Polynomial::new();
        let mut p3 = Polynomial::new();
        p1.set_coefficient(0, 1.);
        p1.set_coefficient(1, 1.);
        p2.set_coefficient(1, 2.);
        p2.set_coefficient(2, 4.);
        p3.set_coefficient(1, 2.);
        p3.set_coefficient(2, 6.);
        p3.set_coefficient(3, 4.);

        assert_eq!(p1 * p2, p3);
    }

    #[test]
    fn test_polynomial_pow() {
        let mut p1 = Polynomial::new();
        let mut p2 = Polynomial::new();
        let mut p3 = Polynomial::new();
        p1.set_coefficient(0, 1.);
        p1.set_coefficient(1, 1.);
        p2.set_coefficient(0, 1.);
        p2.set_coefficient(1, 2.);
        p2.set_coefficient(2, 1.);
        p3.set_coefficient(0, 1.);
        p3.set_coefficient(1, 3.);
        p3.set_coefficient(2, 3.);
        p3.set_coefficient(3, 1.);
        assert_eq!(p1.pow(2), p2);
        assert_eq!(p1.pow(3), p3);
    }
}
