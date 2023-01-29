use std::{ops, fmt::Display};
use crossterm::style;
use rand;

use crate::drawterm;

pub struct Result {
    critfail : bool,
    crit : bool,
    hit : bool,
    value : u16,
    sides : u16,
    modifier : u16, 
}

impl Result {
    pub fn new(critfail: bool, crit: bool, hit: bool, value: u16, sides: u16, modifier: u16) -> Self {
        Self{ critfail, crit, hit, value, sides, modifier}
    }

    pub fn get_crit_fail(&self) -> bool {
        self.critfail
    }

    pub fn get_crit(&self) -> bool {
        self.crit
    }
    
    pub fn get_hit(&self) -> bool {
        self.hit
    }
    
    pub fn get_value(&self) -> u16 {
        self.value
    }

    pub fn get_sides(&self) -> u16 {
        self.sides
    }

    pub fn get_modifier(&self) -> u16 {
        self.modifier
    }
}

impl Clone for Result {
    fn clone(&self) -> Self {
        Self { critfail: self.critfail.clone(), 
            crit: self.crit.clone(),
            hit: self.hit.clone(), 
            value: self.value.clone(), 
            sides: self.sides.clone(),
            modifier: self.modifier.clone() }
    }
}

pub struct Summary {
    summaries : Vec<Summary>,
    results : Vec<Result>,
    hits : u16,
    crits : u16,
    total : u16,
    total_modifier : u16,
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

    pub fn add_result(&mut self, result : Result) {
        self.hits += if result.get_hit() { 1 } else { 0 };
        self.crits += if result.get_crit() { 1 } else { 0 };
        self.total += result.get_value();
        self.total_modifier += result.get_modifier();
        self.results.push(result);
    }

    pub fn get_results(&self) -> &Vec<Result> {
        &self.results
    }

    pub fn get_hits(&self) -> u16 {
        self.hits
    }

    pub fn get_crits(&self) -> u16 {
        self.crits
    }

    pub fn get_total(&self) -> u16 {
        self.total
    }

    pub fn get_total_modifier(&self) -> u16 {
        self.total_modifier
    }

    pub fn get_glitch(&self) -> bool {
        let mut fails = 0;
        for result in &self.results {
            if result.get_crit_fail() {
                fails += 1;
            }
            if fails * 2 > self.results.len() {
                return true;
            }
        }
        return false;
    }

    pub fn print(&self, verbose : bool) {
        if self.get_results().is_empty() && !self.summaries.is_empty() {
            println!("{}", chrono::Local::now());
            println!("____________________________________")
        }
        else if self.get_results().is_empty() {
            println!("____________________________________")
        }
        else {
            if verbose {self.print_dice()};
            println!("Hits:\t\t{}", self.get_hits());
            println!("Total (+{}):\t{}", self.get_total_modifier(), self.get_total());
            self.print_glitch();
            println!("____________________________________")
        }
        for summary in &self.summaries {
            summary.print(verbose);
        }
    }

    fn print_dice(&self){
        for result in self.get_results() {
            if result.get_modifier() != 0 {
                print!(" d{} (+{})\t", result.get_sides(), result.get_modifier());
            }
            else {
                print!(" d{}\t\t", result.get_sides());
            }
            drawterm::print("\t".to_string());
            if result.get_hit() && !result.get_crit() {
                drawterm::print_green(result.get_value().to_string());
            }
            else if result.get_crit() {
                drawterm::print_color(result.get_value().to_string(), style::Color::DarkYellow).unwrap();
            }
            else if result.critfail {
                drawterm::print_red(result.get_value().to_string());
            }
            else {
                drawterm::print(result.get_value().to_string());
            }
            drawterm::print("\n".to_string());
        }
    }

    fn print_glitch(&self){
        if self.get_glitch() {
            if self.get_hits() == 0 {
                drawterm::print_red("Critical glitch!\n".to_string());
            }
            else {
                drawterm::print_color("Glitch!\n".to_string(), style::Color::DarkYellow).unwrap();
            }
        }
    }
}

impl Clone for Summary {
    fn clone(&self) -> Self {
        Self { summaries: self.summaries.clone(), 
            results: self.results.clone(), 
            hits: self.hits.clone(), 
            crits: self.crits.clone(), 
            total: self.total.clone(), 
            total_modifier: self.total_modifier.clone() }
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(true);
        Ok(())
    }
}

pub struct Die {
    crit : bool,
    sides : u16,
    modifier : u16,
}

impl Die {
    pub fn roll(&self, nsc: bool) -> u16 {
        let num;
        if self.crit && nsc {
            num = self.modifier + self.sides;
        }
        else {
            num = 1 + self.modifier + rand::random::<u16>() % self.sides;
        }
        num
    }
}

impl Clone for Die {
    fn clone(&self) -> Self {
        Self { crit: self.crit.clone(), sides: self.sides.clone(), modifier: self.modifier.clone() }
    }
}

impl Default for Die {
    fn default() -> Self {
        Self {crit: false, sides: 1, modifier: 0}
    }
}

pub struct DiceGroup {
    pub dice : Vec<Die>,
    hit: u16,
}

impl DiceGroup {
    pub fn new(dice: Vec<Die>, hit: u16) -> Self {
        Self {dice, hit}
    }

    pub fn from_previous(dice_args: &String, default: u16, crits: u16, hit: u16, no_shitty_crit: bool) -> Option<Self> {
        let new_args = dice_args.replace("x", &default.to_string());
        Self::from(&new_args, crits, hit, no_shitty_crit)
    }

    pub fn from(dice_args: &String, crits: u16, hit: u16, no_shitty_crit: bool) -> Option<Self> {
        let mut dice_vec = vec![];
        let mut crits = crits;
        if !dice_args.contains('d') {
            return None;
        }

        if dice_args.contains('*') {
            let mut multiple_split = dice_args.split('*');
            let mut rolls = multiple_split.next().unwrap_or("1").parse::<u16>().unwrap_or(1);
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
        }
        else {
            let mut d_split = dice_args.split('d');
            let mut rolls = 1;
            let dice = d_split.next().unwrap_or("0").parse::<u16>().unwrap_or(0);
            let side = d_split.next().unwrap_or("1");
            if no_shitty_crit {
                rolls -= if crits > 0 {rolls} else {0};
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
        let mut temp = self.dice.get(0).unwrap_or(&default);
        for die in &self.dice {
            if die.sides != temp.sides {
                return Option::None;
            }
            temp = &die;
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
        Self { dice: self.dice.clone(), hit: self.hit.clone() }
    }
}

fn fill_dice(dice: u16, side: &str, crit: bool, dice_vec: &mut Vec<Die>) {
    let mut modifier = 0;
    let sides;

    if side.contains("+") {
        let mut plus_split = side.split('+');
        sides = plus_split.next().unwrap_or("1").parse::<u16>().unwrap_or(1);
        modifier = plus_split.next().unwrap_or("0").parse::<u16>().unwrap_or(0);
    }
    else {
        sides = side.parse::<u16>().unwrap_or(1);
    }


    for _ in 0..dice {
        dice_vec.push(Die {crit, sides, modifier});
        modifier = 0;
    }
}

impl Default for DiceGroup {
    fn default() -> Self {
        Self { dice: vec![], hit: u16::MAX }
    }
}

pub struct Roller {
    dice : DiceGroup,
    critval: u16,
    success : u16,
    reroll : u16,
    summary : Summary,
}

impl Roller {
    pub fn from_dice_group(dice: DiceGroup, critval: u16, success: u16, reroll: u16) -> Self {
        Self {dice, critval, success, reroll, summary: Summary::new()}
    }

    pub fn roll(&mut self, nsc: bool){
        let mut reroll = self.add_results(&self.dice.dice.to_owned(), nsc);

        while !reroll.is_empty() {
            reroll = self.add_results(&reroll, nsc);
        }
    }

    fn add_results(&mut self, dice : &Vec<Die>, nsc: bool) -> Vec<Die> {
        let mut reroll_result = vec![];
        for die in dice {
            let num = die.roll(nsc);
            let crit = num - die.modifier == die.sides && num - die.modifier == self.critval;
            if num >= self.reroll {
                reroll_result.push(die.clone());
            }
            self.summary.add_result(
                Result::new(num == 1, crit, num >= self.success, num, die.sides, die.modifier));
        }
        reroll_result
    }

    pub fn get_summary(self) -> Summary{
        self.summary
    }
}
