use std::io::stdout;

use crate::{
    components::{Component, ComponentData},
    drawterm,
    probability::{self, HitsGraph, Probability, SummaryDisplay, TotalGraph},
    roll::DiceGroup,
};

pub fn plot_dice_totals(dice: &DiceGroup) {
    let height_row = (drawterm::get_height() as f32 * 0.48) as u16;
    let prob = probability::Total::from_dice(dice);
    let totalbox = TotalGraph::new(
        prob,
        ComponentData::new(
            0,
            drawterm::get_height() - height_row - 2,
            drawterm::get_width() - 1,
            height_row,
        ),
        true,
    );
    totalbox.draw(&stdout()).unwrap();
}

pub fn plot_dice_hits(dice: &DiceGroup) {
    let height_row = (drawterm::get_height() as f32 * 0.48) as u16;
    let prob = probability::Hits::from_dice(dice);
    let hitsbox = HitsGraph::new(
        prob,
        ComponentData::new(
            0,
            drawterm::get_height() - height_row - 2,
            drawterm::get_width() - 1,
            height_row,
        ),
        true,
    );
    hitsbox.draw(&stdout()).unwrap();
}

pub fn show_summary(dice: &DiceGroup, nhits: Option<u16>, ntotal: Option<u16>) {
    let hitsum = SummaryDisplay::new(
        ComponentData::new(
            0,
            drawterm::get_height() - 4 - 2,
            drawterm::get_width() - 1,
            4,
        ),
        dice,
        nhits,
        ntotal,
        true,
    );
    hitsum.draw(&stdout()).unwrap();
}

#[allow(dead_code)]
pub fn demo() {
    let height_row = (drawterm::get_height() as f32 * 0.48) as u16;
    todo!()
    // textplots::Chart::new(
    //     2 * drawterm::get_width() as u32 - 10,
    //     2 * height_row as u32,
    //     -5.0,
    //     5.0,
    // )
    // .lineplot(&textplots::Shape::Continuous(Box::new(|x| {
    //     libm::exp(-(x as f64 * x as f64)) as f32
    // })))
    // .lineplot(&textplots::Shape::Continuous(Box::new(|x| {
    //     (1.0 + libm::cos(x as f64) - libm::sin(x as f64 * 2.0)) as f32 + 1.0
    // })))
    // .nice()
}
