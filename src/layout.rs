use std::io::stdout;

use crate::{
    components::Component,
    probability::{self, HitsGraph, Probability, SummaryDisplay, TotalGraph},
    roll::DiceGroup,
};

pub fn plot_dice_totals(dice: &DiceGroup, total: Option<u16>) {
    let prob = probability::Total::from_dice(dice);
    let totalbox = TotalGraph::new(prob, total.unwrap_or_default());
    totalbox.draw(&stdout()).unwrap();
}

pub fn plot_dice_hits(dice: &DiceGroup, nhits: Option<u16>) {
    let prob = probability::Hits::from_dice(dice);
    let hitsbox = HitsGraph::new(prob, nhits.unwrap_or_default());
    hitsbox.draw(&stdout()).unwrap();
}

pub fn show_summary(dice: &DiceGroup, nhits: Option<u16>, ntotal: Option<u16>) {
    let hitsum = SummaryDisplay::new(dice, nhits, ntotal);
    hitsum.draw(&stdout()).unwrap();
}

#[allow(dead_code)]
pub fn demo() {
    todo!()
}
