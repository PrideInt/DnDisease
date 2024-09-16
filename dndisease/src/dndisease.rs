use std::thread;
use std::io;
use pathogen::Pathogen;
use pathogen::Bacteria;
use enemy::Opponent;
use colored::*;

mod pathogen;
mod enemy;

fn main() {
    println!("{}", "Welcome to DnDisease!".cyan().bold());
    println!("{}{}\n", "Commands: ".yellow(), "attack, guard".yellow().italic());

    let handler = thread::spawn(|| {
        let mut bacteria: Bacteria = pathogen::new_bacteria();
        let enemy: Opponent = enemy::new_opponent("United States", 20, 20);

        while bacteria.get_health() > 0 && enemy.get_health() > 0 {
            let mut input_read: bool = true;

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Enter command again.");

            println!("You entered: {}", input);

            match input.trim() {
                "attack" => {
                    let attack_roll: u32 = bacteria.attack();
                    let enemy_saving_roll: u32 = enemy.save();

                    if attack_roll > enemy_saving_roll {
                        println!("{}", "Bacteria attack hits!".blue());
                    } else {
                        println!("{}", "Bacteria attack misses!".red());
                    }
                },
                "guard" => {
                    bacteria.guard();
                },
                _ => {
                    println!("{}", "Invalid command.".red());
                    input_read = false;
                }
            }
            println!();

            if input_read {
                let enemy_attack_roll: u32 = enemy.attack();
                let saving_roll: u32 = bacteria.save();

                if enemy_attack_roll > saving_roll {
                    println!("{}", "Enemy attack hits!".blue());

                    let damage: u32 = 4;
                    bacteria.take_damage(damage - bacteria.get_armor_class());

                    if bacteria.get_armor_class() > bacteria.get_original_armor_class() {
                        bacteria.remove_guard();
                    }
                    println!("Bacteria health: {}", bacteria.get_health().to_string().yellow());
                } else {
                    println!("{}", "Enemy attack misses!".red());
                }
            }
            println!();
        }
    });
    handler.join().unwrap();
}
