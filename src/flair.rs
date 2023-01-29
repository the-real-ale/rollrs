// this will have all the animated silly things that don't have actual data
use std::{thread, time};
use crossterm::style::Stylize;
use rand::{self, random};
use crate::drawterm;
use crate::name;

pub fn print_silly_shit(){
    let checks = [
        "Synchronized packet transmission...".to_string(), 
        "Multiple hops through coNET...".to_string(),
        "Scrambled exit node address...".to_string(),
        "Correct AAA headers from tunnel traffic...".to_string(),
        "Hacked the first firewall...".to_string()];
    let data = [
        "Downloading SatComm Telemetry...".to_string(), 
        "Compiling local context keywords...".to_string(),
        "Uploading local radio traffic to TacCon...".to_string(),
        "Downloading convolution matrix...".to_string(),
        "Analyzing real-time TacCon data...".to_string()];

    print_bar_checks(&["Establishing secure connection".to_string()]);
    print_name();
    let mut addr = print_qeh_link();
    println!("Confirming anonymizing techniques. If none of the following succeed, disconnect IMMEDIATELY!\n");
    print_status_checks(&checks);
    println!("\nBeginning TacCon aggregation sequence...");
    print_bar_checks(&data);
    print_qeh_break(&mut addr);
    print_signature();
}

struct Address {
    addrs : Vec<u16>,
    index : u8,
}

impl Address {
    pub fn size() -> usize {
        4
    }

    pub fn new() -> Self {
        let mut addrs = vec![];
        for _ in 0..Self::size() {
            addrs.push(random::<u16>());
        }
        Self {addrs, index: 0}
    }

    pub fn next(&mut self) -> u16 {
        if self.index == u8::MAX {
            self.index = 0
        }
        let result = *self.addrs.get(self.index as usize % Self::size() as usize).unwrap();
        self.index += 1;
        result
    }
}

fn print_status_checks(checks: &[String]){
    let len = get_max_length(checks);
    for check in checks {
        let millis = rand::random::<u16>() % 1000;
        let delay = time::Duration::from_millis(millis as u64);
        drawterm::print(check.to_string());
        print_buffer(len, &check);
        thread::sleep(delay);
        if rand::random::<u8>() % 100 < 90 {
            drawterm::print_green("Ok".to_string());
        }
        else {
            drawterm::print_red("Error!".to_string());
        }
        drawterm::print("\n".to_string());
    }
}

fn print_bar_checks(checks: &[String]){
    let width = drawterm::get_width();
    let len = get_max_length(checks);
    let mut barsize: u16 = width - (len as u16 + 4);
    barsize = if barsize > 40 { 40 } else { barsize };
    for check in checks {
        print_check(check, len);
        print_bar(barsize);
    }
}

fn print_check(check: &String, len: usize) {
    drawterm::print(check.to_string());
    print_buffer(len, check);
}

fn print_buffer(len: usize, check: &String) {
    let mut buffer: String = "  ".to_string();
    for _ in 0..len - check.len() {
        buffer = buffer + " ";
    }
    drawterm::print(buffer);
}

fn print_bar(barsize: u16) {
    for _ in 0..barsize {
        let delay = time::Duration::from_millis((rand::random::<u16>() % 100 + (100.0 / barsize as f32) as u16) as u64);
        if delay > time::Duration::from_millis(90) { thread::sleep(delay); }
        drawterm::print("|".to_string());
    }
    drawterm::print("\n".to_string());
}

fn print_name(){
    let delay = time::Duration::from_millis(500);
    println!("");
    println!("Secret key accepted. Welcome, {}!", name::random().bold().underlined());
    thread::sleep(delay);
}

fn print_signature(){
    let delay = time::Duration::from_millis(1000);
    let sig = "The North American Free Information Society";
    println!("");
    println!("This report was stolen for you by");
    println!(
"         ,-.
        / \\  `.  __..-,O
       :   \\ --''_..-'.'
       |    . .-' `. '.
       :     .     .`.'
        \\     `.  /  ..
         \\      `.   ' .
          `,       `.   \\
         ,|,`.        `-.\\
        '.||  ``-...__..-`
         |  |
         |__|
         /||\\
        //||\\\\
       // || \\\\
    __//__||__\\\\__
   '--------------' SSt"
    );
    println!("{}", sig.bold().underlined());
    println!("");
    thread::sleep(delay);
}

fn print_qeh_link() -> Address{
    let mut addr = Address::new();
    let delay = time::Duration::from_millis(200);
    let now = chrono::Utc::now();
    let later = now.checked_add_months(chrono::Months::new(52 * 12)).unwrap_or(now);
    let format = later.format("%m/%d/%Y %H:%M UTC ");
    println!("");
    thread::sleep(delay);
    println!("\t<<<<< QEH Signal Established >>>>>");
    thread::sleep(delay);
    println!("\tSecure link to {:0>4x}:{:0>4x}:{:0>4x}:{:0>4x}", addr.next(), addr.next(), addr.next(), addr.next()); 
    thread::sleep(delay);
    println!("\t{}\n", format);
    thread::sleep(delay);
    addr
}

fn print_qeh_break(addr: &mut Address) {
    let now = chrono::Utc::now();
    let delay = time::Duration::from_millis(200);
    let later = now.checked_add_months(chrono::Months::new(52 * 12)).unwrap_or(now);
    let format = later.format("%m/%d/%Y %H:%M UTC ");
    println!("");
    thread::sleep(delay);
    println!("\t<<<<< QEH Signal Invalid/Missing >>>>>");
    thread::sleep(delay);
    println!("\tDisconnected from {:0>4x}:{:0>4x}:{:0>4x}:{:0>4x} (Broken Pipe)", addr.next(), addr.next(), addr.next(), addr.next()); 
    thread::sleep(delay);
    println!("\t{}\n", format);
    thread::sleep(delay);
}

fn get_max_length(checks: &[String]) -> usize {
    let mut len: usize = 0;
    for check in checks {
        if check.len() > len { len = check.len(); }
    }
    len
}