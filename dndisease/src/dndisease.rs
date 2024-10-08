use std::{thread, time};
use std::io;
use pathogen::Pathogen;
use pathogen::Bacteria;
use enemy::Opponent;
use animation::TextAnimationBuilder;
use colored::*;

mod pathogen;
mod enemy;
mod animation;

fn main() {
    // Clear terminal prior to starting game
    print!("{}[2J", 27 as char);

    let handler = thread::spawn(|| {
        TextAnimationBuilder::animate("Welcome to DnDisease!".to_string(), 50, "cyan".to_string(), "bold".to_string());
        println!("{}{}\n", "Commands: ".yellow(), "attack, guard".yellow().italic());

        let mut bacteria: Bacteria = pathogen::new_bacteria();
        let enemy: Opponent = enemy::new_opponent("United States", 20, 20);

        while bacteria.get_health() > 0 && enemy.get_health() > 0 {
            let mut input_read: bool = true;
            // let curr_time = time::Instant::now();

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
                    thread::sleep(time::Duration::from_millis(1000));
                },
                "guard" => {
                    bacteria.guard();
                    thread::sleep(time::Duration::from_millis(1000));
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
                    println!();
                } else {
                    println!("{}", "Enemy attack misses!".red());
                    println!();
                }
                thread::sleep(time::Duration::from_millis(1000));
                println!("Bacteria health: {}", bacteria.get_health().to_string().yellow());
            }
            println!();
        }
    });
    handler.join().unwrap();
}