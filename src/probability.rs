use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::Stdout,
    ops::{self, MulAssign},
};

use crossterm::{
    queue,
    style::{Print, PrintStyledContent, ResetColor, Stylize},
};
use itertools::Itertools;

use crate::{components::Component, drawterm::get_horizontal_fraction, roll::DiceGroup};

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
    fn get_probability_of(&self, value: u16) -> f64;
    fn get_probability_of_gt(&self, value: u16) -> f64;
    fn to_data(&self) -> Vec<(u16, f32)>;
}

pub struct Total {
    polynomial: Polynomial,
    dice: DiceGroup,
}

pub struct TotalGraph {
    totals: Total,
    total: u16,
}

impl TotalGraph {
    pub fn new(totals: Total, total: u16) -> Self {
        Self { totals, total }
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

    fn to_data(&self) -> Vec<(u16, f32)> {
        self.polynomial
            .get_coefficients()
            .keys()
            .sorted()
            .map(|entry| {
                (
                    *entry + self.dice.get_total_modifier(),
                    100. * self.get_probability_of(*entry) as f32,
                )
            })
            .collect()
    }
}

impl Component for TotalGraph {
    fn draw(&self, mut stdout: &Stdout) -> crossterm::Result<()> {
        let data = self.totals.to_data();
        data.iter()
            .filter(|i| i.1 > 0.1)
            .map(|i| {
                (
                    i.0,
                    format!(
                        "{:>3}:\t{:>5.1} {}\n",
                        i.0,
                        i.1,
                        get_horizontal_bar(
                            i.1,
                            Into::<usize>::into(
                                termsize::get()
                                    .unwrap_or(termsize::Size { rows: 0, cols: 0 })
                                    .cols
                                    / 2
                            )
                        )
                        .iter()
                        .collect::<String>()
                    ),
                )
            })
            .map(|(i, s)| {
                if self.total > 1 && i > self.total {
                    s.bold().green()
                } else if self.total > 1 && i == self.total {
                    s.bold().dark_yellow()
                } else {
                    s.reset()
                }
            })
            .for_each(|s| {
                queue!(stdout, PrintStyledContent(s));
                queue!(stdout, ResetColor);
            });
        Ok(())
    }
}

pub struct Hits {
    data: HashMap<u16, f64>,
}

pub struct HitsGraph {
    hits: Hits,
    hit: u16,
}

impl HitsGraph {
    pub fn new(hits: Hits, hit: u16) -> Self {
        Self { hits, hit }
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

    fn to_data(&self) -> Vec<(u16, f32)> {
        self.data
            .keys()
            .sorted()
            .map(|entry| (*entry, 100. * self.get_probability_of(*entry) as f32))
            .collect()
    }
}

impl Component for HitsGraph {
    fn draw(&self, mut stdout: &Stdout) -> crossterm::Result<()> {
        let data = self.hits.to_data();
        data.iter()
            .filter(|i| i.1 > 0.1)
            .map(|i| {
                (
                    i.0,
                    format!(
                        "{:>3}:\t{:>5.1} {}\n",
                        i.0,
                        i.1,
                        get_horizontal_bar(
                            i.1,
                            Into::<usize>::into(
                                termsize::get()
                                    .unwrap_or(termsize::Size { rows: 0, cols: 0 })
                                    .cols
                                    / 2
                            )
                        )
                        .iter()
                        .collect::<String>()
                    ),
                )
            })
            .map(|(i, s)| {
                if self.hit > 1 && i > self.hit {
                    s.bold().green()
                } else if self.hit > 1 && i == self.hit {
                    s.bold().dark_yellow()
                } else {
                    s.reset()
                }
            })
            .for_each(|s| {
                queue!(stdout, PrintStyledContent(s));
                queue!(stdout, ResetColor);
            });
        Ok(())
    }
}

pub struct SummaryDisplay {
    text: String,
}

impl SummaryDisplay {
    pub fn new(dice: &DiceGroup, hitnum: Option<u16>, totalnum: Option<u16>) -> Self {
        let hits = hitnum.unwrap_or(u16::MAX);
        let total = totalnum.unwrap_or(u16::MAX);
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
            text = format!(
                "\nProbability of glitch:\t\t{}%\nProbability of critical glitch:\t{}%",
                glitch.bold().dark_yellow(),
                critglitch.bold().dark_red()
            );
        }
        Self { text }
    }
}

impl Component for SummaryDisplay {
    fn draw(&self, mut stdout: &Stdout) -> crossterm::Result<()> {
        queue!(stdout, Print(self.text.as_str()))?;
        Ok(())
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

fn get_horizontal_bar(value: f32, width: usize) -> Vec<char> {
    let mut result = vec!['█'; (value * width as f32 / 50f32) as usize];
    let len = result.len();
    if len != 0 {
        result[len - 1] = get_horizontal_fraction(value % width as f32);
    }
    result
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
