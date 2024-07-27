use std::io::stdout;

use clap::{command, Arg, ArgAction, ArgMatches, Command};
use crossterm::{
    execute, queue,
    style::{Print, Stylize},
    terminal,
};
use layout::{plot_dice_hits, plot_dice_totals, show_summary};
use roll::{DiceGroup, Roller, Summary};

mod components;
mod drawterm;
mod first_names;
mod flair;
mod last_names;
mod layout;
mod name;
mod probability;
mod roll;

fn main() {
    let matches = get_matches();
    size_console();
    if matches.subcommand_matches("help-dice").is_some() {
        run_demo()
    } else if matches.subcommand_matches("sim").is_some() {
        run_sim(&matches);
    } else {
        run_roll(&matches);
    }
}

fn run_sim(matches: &ArgMatches) {
    let sim_match = matches.subcommand_matches("sim").unwrap();
    let success: u16 = matches
        .get_one::<String>("Success")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .unwrap_or(u16::MAX);
    let numhits: Option<u16> = sim_match
        .get_one::<String>("Hits")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .ok();
    let numtotal: Option<u16> = sim_match
        .get_one::<String>("Total")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .ok();
    let dice_args = matches.get_many::<String>("Dice").unwrap_or_else(|| {
        show_dice_warning();
        clap::parser::ValuesRef::default()
    });

    if !sim_match.get_flag("NoBS") {
        flair::print_silly_shit();
    }
    for dice in dice_args {
        if dice.trim().is_empty() {
            show_dice_warning();
            continue;
        }

        let d = DiceGroup::from(dice, 0, success, matches.get_flag("NSC")).unwrap_or_default();
        if sim_match.get_flag("Show Totals") {
            plot_dice_totals(&d, numtotal);
        }
        if sim_match.get_flag("Show Hits") {
            plot_dice_hits(&d, numhits);
        }
        if !sim_match.get_flag("Hide Summary") {
            show_summary(&d, numhits, numtotal);
        }
    }
}

fn run_roll(matches: &ArgMatches) {
    let mut previous = Summary::new();
    let mut total = Summary::new();
    let success: u16 = matches
        .get_one::<String>("Success")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .unwrap_or(u16::MAX);
    let reroll: u16 = matches
        .get_one::<String>("Reroll")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .unwrap_or(u16::MAX);
    let critval: u16 = matches
        .get_one::<String>("Crit")
        .unwrap_or(&u16::MAX.to_string())
        .parse()
        .unwrap_or(u16::MAX);
    let no_shitty_crits = matches.get_flag("NSC");
    let dice_args = matches.get_many::<String>("Dice").unwrap_or_else(|| {
        show_dice_warning();
        clap::parser::ValuesRef::default()
    });
    for dice in dice_args {
        if dice.trim().is_empty() {
            show_dice_warning();
            continue;
        }
        roll(
            dice,
            &mut previous,
            success,
            no_shitty_crits,
            critval,
            reroll,
            &mut total,
        );
    }

    queue!(stdout(), Print(total)).ok();
}

fn run_demo() {
    let mut previous = Summary::new();
    let mut total = Summary::new();

    println!("\n{}", "Dice Format Tutorial".bold().underlined());
    println!("{}",wrap(
        &format!("Specify the number and type of dice in \'{}\' format. For example five six-sided dice is 5d6.", "x*ndm+c".bold())));
    println!("\n>> {}\n -->", "roll -v -d \"5d6\"".bold());
    roll(
        "5d6",
        &mut Summary::new(),
        u16::MAX,
        false,
        u16::MAX,
        u16::MAX,
        &mut total,
    );

    queue!(stdout(), Print(total)).ok();
    total = Summary::new();

    println!("\n{}", wrap("Multiple arguments may be listed with spaces by surrounding the dice with quotations: '-d \"3d4 6d6...\"' \
Dice arguments may contain a constant modifier by using a plus sign at the end of the dice. '2d6+4' Rolls two six-sided dice with a +4 modifier. \
The modifier may be applied to multiple dice using the multiplication operator. '2*1d20+8' rolls two twenty-sided dice and applies a +8 modifier to each roll."));
    println!("\n>> {}\n -->", "roll -v -d \"2*1d20+8\" -s 20".bold());
    roll(
        "2*1d20+8",
        &mut Summary::new(),
        20,
        false,
        u16::MAX,
        u16::MAX,
        &mut total,
    );
    queue!(stdout(), Print(total)).ok();
    total = Summary::new();

    println!("\n{}", wrap("Arguments may contain a reference to the previous number of 'successes' using the letter 'x'. \
The dice sequence \"2*1d20+8 x*1d8+4\" rolls a d8 dice with a +4 modifier for every 'success' received on the previous set of twenty-sided dice."));
    println!(
        "\n>> {}\n -->",
        "roll -v -d \"3*1d20+8 x*1d8+4\" -s 14".bold()
    );
    roll(
        "3*1d20+8",
        &mut previous,
        14,
        false,
        u16::MAX,
        u16::MAX,
        &mut total,
    );
    roll(
        "x*1d8+4",
        &mut previous,
        14,
        false,
        u16::MAX,
        u16::MAX,
        &mut total,
    );
    queue!(stdout(), Print(total)).ok();
}

fn roll(
    dice: &str,
    previous: &mut Summary,
    success: u16,
    no_shitty_crits: bool,
    critval: u16,
    reroll: u16,
    total: &mut Summary,
) {
    let d = DiceGroup::from_previous(
        dice,
        previous.hits,
        previous.crits,
        success,
        no_shitty_crits,
    )
    .unwrap_or_default();
    let mut roller = Roller::from_dice_group(d, critval, success, reroll);
    roller.roll(no_shitty_crits);
    let summary = roller.get_summary();
    *total += summary.to_owned();
    *previous = summary;
}

fn show_dice_warning() {
    println!("\n{} The following suggested arguments were not provided:\n\t{}\n\nThe dice roller has no dice to roll...\n", 
        "warning:".bold().dark_yellow(),
        "--dice <Dice>".green()
    )
}

fn size_console() {
    if drawterm::get_height() == 0 || drawterm::get_width() == 0 {
        execute!(&mut stdout(), terminal::SetSize(100, 60)).unwrap();
    }
}

fn wrap(string: &str) -> String {
    textwrap::fill(string, drawterm::get_width() as usize - 3)
}

fn get_matches() -> ArgMatches {
    let dice_help = format!(
        "The number and type of dice in \'x*ndm+c\' format. Type {} for more information.",
        "help-dice".to_string().bold()
    );
    command!()
        .arg(
            Arg::new("Dice")
                .short('d')
                .long("dice")
                .value_delimiter(' ')
                .action(ArgAction::Append)
                // .required(true)
                .help(dice_help)
        ).arg(
            Arg::new("Success")
                .short('s')
                .long("success")
                .help("Set the value of a success. This is used to report summary results and calculate predictions.")
                .action(ArgAction::Set)
        ).arg(
            Arg::new("Reroll")
                .short('r')
                .long("reroll")
                .help("Set the value to reroll at. For example, when rolling 5d6 with reroll 6, dice at 5 or 6 will be rerolled.")
                .action(ArgAction::Set)
        ).arg(
            Arg::new("Crit")
                .short('c')
                .long("count-crits")
                .help("Sets the value which counts as a critical and change variable dice behavior.")
                .action(ArgAction::Set)
        ).arg(
            Arg::new("NSC")
                .short('q')
                .long("no-shitty-crits")
                .help("Change crit behavior to be in line with the popular nsc homebrew rules.")
                .action(ArgAction::SetTrue)
        ).subcommand(
            Command::new("help-dice")
                .about("Show more information on dice syntax and behavior.")
        ).subcommand(
            Command::new("sim")
                .about("Simulate and predict probabilities of possible outcomes.")
                .arg(
                    Arg::new("Hits")
                        .short('n')
                        .long("nhits")
                        .help("Set the number of hits or total value that would count as a success for the simulated roll.")
                        .action(ArgAction::Set)
                ).arg(
                    Arg::new("Total")
                        .short('s')
                        .long("sum-total")
                        .help("Set the total value of success when calculating probabilities. (Cumulative dice value, not hits)")
                        .action(ArgAction::Set)
                ).arg(
                    Arg::new("NoBS")
                        .short('b')
                        .long("no-bullshit")
                        .help("Setting this flag will silence all the fake sci-fi flair at the beginning of a report.")
                        .action(ArgAction::SetTrue)
                ).arg(
                    Arg::new("Show Totals")
                        .short('t')
                        .long("show-totals")
                        .help("Setting this flag will show a graph of probabilities for the total values of the given dice groups.")
                        .action(ArgAction::SetTrue)
                ).arg(
                    Arg::new("Show Hits")
                        .short('p')
                        .long("show-hits")
                        .help("Setting this flag will show a graph of probabilities for the possible hits of the given dice groups.")
                        .action(ArgAction::SetTrue)
                ).arg(
                    Arg::new("Hide Summary")
                        .short('z')
                        .long("hide-summary")
                        .help("Setting this flag will hide the probability summaries.")
                        .action(ArgAction::SetTrue)
                )
        ).get_matches()
}
