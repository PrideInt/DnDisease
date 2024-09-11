use std::thread;
use pathogen::Pathogen;
use pathogen::Bacteria;
use enemy::Opponent;
use colored::*;

mod pathogen;
mod enemy;

fn main() {
    let handler = thread::spawn(|| {
        let bacteria: Bacteria = pathogen::new_bacteria();
        let enemy: Opponent = enemy::new_opponent("United States", 20);

        for _ in 0..10 {
            let attack_roll = bacteria.attack();
            let saving_roll = enemy.save();

            if attack_roll > saving_roll {
                println!("{}", "Bacteria attack hits!".blue());
            } else {
                println!("{}", "Bacteria attack misses!".red());
            }
        }
    });
    handler.join().unwrap();
}
