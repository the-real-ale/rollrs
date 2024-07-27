use crossterm::style::Stylize;
use std::{
    fmt::{Display, Write},
    ops,
};

#[derive(Clone, Copy)]
pub struct Result {
    pub critfail: bool,
    pub crit: bool,
    pub hit: bool,
    pub value: u16,
    pub sides: u16,
    pub modifier: u16,
}

#[derive(Clone)]
pub struct Summary {
    summaries: Vec<Summary>,
    results: Vec<Result>,
    pub hits: u16,
    pub crits: u16,
    pub total: u16,
    pub total_modifier: u16,
}

impl Summary {
    pub fn new() -> Self {
        Self {
            summaries: vec![],
            results: vec![],
            hits: 0,
            crits: 0,
            total: 0,
            total_modifier: 0,
        }
    }

    pub fn add_result(&mut self, result: Result) {
        self.hits += if result.hit { 1 } else { 0 };
        self.crits += if result.crit { 1 } else { 0 };
        self.total += result.value;
        self.total_modifier += result.modifier;
        self.results.push(result);
    }

    pub fn get_results(&self) -> &[Result] {
        &self.results
    }

    pub fn get_glitch(&self) -> bool {
        let mut fails = 0;
        for result in &self.results {
            if result.critfail {
                fails += 1;
            }
            if fails * 2 > self.results.len() {
                return true;
            }
        }
        false
    }

    pub fn print(&self, verbose: bool, f: &mut std::fmt::Formatter<'_>) {
        if self.get_results().is_empty() && !self.summaries.is_empty() {
            f.write_str(format!("{}\n", chrono::Local::now()).as_str())
                .ok();
            f.write_str(format!("____________________________________\n").as_str())
                .ok();
        } else if self.get_results().is_empty() {
            f.write_str(format!("____________________________________\n").as_str())
                .ok();
        } else {
            if verbose {
                self.print_dice(f)
            };
            f.write_str(format!("Hits:\t\t{}\n", self.hits).as_str())
                .ok();
            f.write_str(format!("Total (+{}):\t{}\n", self.total_modifier, self.total).as_str())
                .ok();
            self.print_glitch(f);
            f.write_str(format!("____________________________________\n").as_str())
                .ok();
        }
        for summary in &self.summaries {
            summary.print(verbose, f);
        }
    }

    fn print_dice(&self, f: &mut std::fmt::Formatter<'_>) {
        for result in self.get_results() {
            if result.modifier != 0 {
                f.write_str(format!(" d{} (+{})\t", result.sides, result.modifier).as_str())
                    .ok();
            } else {
                f.write_str(format!(" d{}\t\t", result.sides).as_str()).ok();
            }
            f.write_char('\t').ok();
            if result.hit && !result.crit {
                f.write_str(format!("{}", result.value.to_string().green()).as_str())
                    .ok();
            } else if result.crit {
                f.write_str(format!("{}", result.value.to_string().dark_yellow()).as_str())
                    .ok();
            } else if result.critfail {
                f.write_str(format!("{}", result.value.to_string().dark_red()).as_str())
                    .ok();
            } else {
                f.write_str(format!("{}", result.value).as_str()).ok();
            }
            f.write_char('\n').ok();
        }
    }

    fn print_glitch(&self, f: &mut std::fmt::Formatter<'_>) {
        if self.get_glitch() {
            if self.hits == 0 {
                f.write_str(format!("{}", "Critical glitch!\n".dark_red()).as_str())
                    .ok();
                // drawterm::print_red("Critical glitch!\n".to_string());
            } else {
                f.write_str(format!("{}", "Glitch!\n".dark_yellow()).as_str())
                    .ok();
                // drawterm::print_color("Glitch!\n".to_string(), style::Color::DarkYellow).unwrap();
            }
        }
    }
}

impl ops::Add<Summary> for Summary {
    type Output = Self;

    fn add(self, rhs: Summary) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl ops::AddAssign<Summary> for Summary {
    fn add_assign(&mut self, rhs: Summary) {
        self.summaries.push(rhs);
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(true, f);
        Ok(())
    }
}

pub struct Die {
    crit: bool,
    sides: u16,
    modifier: u16,
}

impl Die {
    pub fn roll(&self, nsc: bool) -> u16 {
        let num;
        if self.crit && nsc {
            num = self.modifier + self.sides;
        } else {
            num = 1 + self.modifier + rand::random::<u16>() % self.sides;
        }
        num
    }
}

impl Clone for Die {
    fn clone(&self) -> Self {
        Self {
            crit: self.crit,
            sides: self.sides,
            modifier: self.modifier,
        }
    }
}

impl Default for Die {
    fn default() -> Self {
        Self {
            crit: false,
            sides: 1,
            modifier: 0,
        }
    }
}

pub struct DiceGroup {
    pub dice: Vec<Die>,
    hit: u16,
}

impl DiceGroup {
    pub fn new(dice: Vec<Die>, hit: u16) -> Self {
        Self { dice, hit }
    }

    pub fn from_previous(
        dice_args: &str,
        default: u16,
        crits: u16,
        hit: u16,
        no_shitty_crit: bool,
    ) -> Option<Self> {
        let new_args = dice_args.replace('x', &default.to_string());
        Self::from(&new_args, crits, hit, no_shitty_crit)
    }

    pub fn from(dice_args: &str, crits: u16, hit: u16, no_shitty_crit: bool) -> Option<Self> {
        let mut dice_vec = vec![];
        let mut crits = crits;
        if !dice_args.contains('d') {
            return None;
        }

        if dice_args.contains('*') {
            let mut multiple_split = dice_args.split('*');
            let mut rolls = multiple_split
                .next()
                .unwrap_or("1")
                .parse::<u16>()
                .unwrap_or(1);
            let mut d_split = multiple_split.next().unwrap().split('d');
            let dice = d_split.next().unwrap_or("0").parse::<u16>().unwrap_or(0);
            let side = d_split.next().unwrap_or("1");
            if no_shitty_crit {
                rolls -= crits;
                crits *= 2;
            }
            for _ in 0..rolls {
                fill_dice(dice, side, false, &mut dice_vec);
            }
            for _ in 0..crits {
                fill_dice(dice, side, true, &mut dice_vec);
            }
        } else {
            let mut d_split = dice_args.split('d');
            let mut rolls = 1;
            let dice = d_split.next().unwrap_or("0").parse::<u16>().unwrap_or(0);
            let side = d_split.next().unwrap_or("1");
            if no_shitty_crit {
                rolls -= if crits > 0 { rolls } else { 0 };
                crits *= 2;
            }
            for _ in 0..rolls {
                fill_dice(dice, side, false, &mut dice_vec);
            }
            for _ in 0..crits {
                fill_dice(dice, side, true, &mut dice_vec);
            }
        }
        Some(Self::new(dice_vec, hit))
    }

    pub fn get_count(&self) -> u16 {
        self.dice.len() as u16
    }

    pub fn get_hit(&self) -> u16 {
        self.hit
    }

    pub fn get_sides(&self) -> Option<u16> {
        let default = Die::default();
        let mut temp = self.dice.first().unwrap_or(&default);
        for die in &self.dice {
            if die.sides != temp.sides {
                return Option::None;
            }
            temp = die;
        }
        Option::Some(temp.sides)
    }

    pub fn get_total_modifier(&self) -> u16 {
        let mut temp = 0;
        for die in &self.dice {
            temp += die.modifier;
        }
        temp
    }
}

impl Clone for DiceGroup {
    fn clone(&self) -> Self {
        Self {
            dice: self.dice.clone(),
            hit: self.hit,
        }
    }
}

fn fill_dice(dice: u16, side: &str, crit: bool, dice_vec: &mut Vec<Die>) {
    let mut modifier = 0;
    let sides;

    if side.contains('+') {
        let mut plus_split = side.split('+');
        sides = plus_split.next().unwrap_or("1").parse::<u16>().unwrap_or(1);
        modifier = plus_split.next().unwrap_or("0").parse::<u16>().unwrap_or(0);
    } else {
        sides = side.parse::<u16>().unwrap_or(1);
    }

    for _ in 0..dice {
        dice_vec.push(Die {
            crit,
            sides,
            modifier,
        });
        modifier = 0;
    }
}

impl Default for DiceGroup {
    fn default() -> Self {
        Self {
            dice: vec![],
            hit: u16::MAX,
        }
    }
}

pub struct Roller {
    dice: DiceGroup,
    critval: u16,
    success: u16,
    reroll: u16,
    summary: Summary,
}

impl Roller {
    pub fn from_dice_group(dice: DiceGroup, critval: u16, success: u16, reroll: u16) -> Self {
        Self {
            dice,
            critval,
            success,
            reroll,
            summary: Summary::new(),
        }
    }

    pub fn roll(&mut self, nsc: bool) {
        let mut reroll = self.add_results(&self.dice.dice.to_owned(), nsc);

        while !reroll.is_empty() {
            reroll = self.add_results(&reroll, nsc);
        }
    }

    fn add_results(&mut self, dice: &Vec<Die>, nsc: bool) -> Vec<Die> {
        let mut reroll_result = vec![];
        for die in dice {
            let num = die.roll(nsc);
            let crit = num - die.modifier == die.sides && num - die.modifier == self.critval;
            if num >= self.reroll {
                reroll_result.push(die.clone());
            }
            self.summary.add_result(Result {
                critfail: num == 1,
                crit,
                hit: num >= self.success,
                value: num,
                sides: die.sides,
                modifier: die.modifier,
            });
        }
        reroll_result
    }

    pub fn get_summary(self) -> Summary {
        self.summary
    }
}
