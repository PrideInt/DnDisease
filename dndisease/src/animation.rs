use std::{thread, time};
use std::io::Write;
use colored::*;

pub struct TextAnimationBuilder {
    text: String,
    delay: u64
}

impl TextAnimationBuilder {
    pub fn build(text: String, delay: u64) -> TextAnimationBuilder {
        return TextAnimationBuilder {
            text: text,
            delay: delay
        }
    }
    pub fn new() -> TextAnimationBuilder {
        return TextAnimationBuilder {
            text: "".to_string(),
            delay: 0
        }
    }
    pub fn animate(text: String, delay: u64, color: String, other: String) {
        for c in text.chars() {
            let mut txt: ColoredString = c.to_string().normal();

            match other.trim() {
                "bold" => txt = txt.bold(),
                "italic" => txt = txt.italic(),
                "underline" => txt = txt.underline(),
                _ => {}
            }
            match color.trim() {
                "red" => print!("{}", txt.red()),
                "blue" => print!("{}", txt.blue()),
                "green" => print!("{}", txt.green()),
                "yellow" => print!("{}", txt.yellow()),
                "cyan" => print!("{}", txt.cyan()),
                "magenta" => print!("{}", txt.magenta()),
                "white" => print!("{}", txt.white()),
                "normal" => print!("{}", c),
                _ => print!("{}", c)
            }
            std::io::stdout().flush().expect("Flushing, Queens");
            thread::sleep(time::Duration::from_millis(delay))
        }
        println!();
    }
    pub fn animate_normal(text: String, delay: u64) {
        Self::animate(text, delay, "normal".to_string(), "".to_string());
    }
    pub fn animate_text(&self) {
        Self::animate_normal(self.text.clone(), self.delay);
    }
    pub fn get_text(&self) -> String {
        return self.text.clone();
    }
    pub fn get_delay(&self) -> u64 {
        return self.delay;
    }
}