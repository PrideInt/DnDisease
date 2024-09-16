use rand::Rng;
use colored::*;

pub struct Bacteria { name: String, d: u32, h: u32, ac: u32, oac: u32 }

pub struct Virus { name: String, d: u32, h: u32, ac: u32, oac: u32 }

pub struct Fungi { name: String, d: u32, h: u32, ac: u32, oac: u32 }

pub struct Protozoa { name: String, d: u32, h: u32, ac: u32, oac: u32 }

pub struct Prion { name: String, d: u32, h: u32, ac: u32, oac: u32 }

pub trait Pathogen {
    fn attack(&self) -> u32;
    fn save(&self) -> u32;
    fn roll(&self) -> u32;
    fn take_damage(&mut self, damage: u32) -> u32;
    fn guard(&mut self);
    fn remove_guard(&mut self);

    fn get_name(&self) -> String;
    fn get_health(&self) -> u32;
    fn get_armor_class(&self) -> u32;
}

/**
Make new Bacteria
*/
pub fn new_bacteria() -> Bacteria {
    return Bacteria {
        name: "Bacteria".to_string(),
        d: 6,
        h: 10,
        ac: 1,
        oac: 1
    }
}

/**
Make new Virus
*/
pub fn new_virus() -> Virus {
    return Virus {
        name: "Virus".to_string(),
        d: 10,
        h: 5,
        ac: 0,
        oac: 0
    }
}

/**
Make new Fungus
*/
pub fn new_fungi() -> Fungi {
    return Fungi {
        name: "Fungi".to_string(),
        d: 8,
        h: 8,
        ac: 2,
        oac: 2
    }
}

/**
Make new Protozoan
*/
pub fn new_protozoa() -> Protozoa {
    return Protozoa {
        name: "Protozoa".to_string(),
        d: 12,
        h: 15,
        ac: 0,
        oac: 0
    }
}

/**
Make new Prion
*/
pub fn new_prion() -> Prion {
    return Prion {
        name: "Prion".to_string(),
        d: 20,
        h: 3,
        ac: 3,
        oac: 3
    }
}

impl Pathogen for Bacteria {
    fn attack(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("Bacteria attack! Rolling... {}", roll);
        return roll;
    }
    fn save(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("Bacteria saving throw! Rolling... {}", roll);
        return roll;
    }
    fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(1..self.d);
    }
    fn take_damage(&mut self, damage: u32) -> u32 {
        if self.h - damage <= 0 {
            self.h = 0;
            return 0;
        }
        self.h -= damage;
        return self.h - damage;
    }
    fn guard(&mut self) {
        self.ac += 2;
        println!("{}", "Bacteria guards! Armor class increased by 2.".yellow());
    }
    fn remove_guard(&mut self) { 
        if self.ac - 2 >= 0 {
            self.ac -= 2;
        }
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_health(&self) -> u32 {
        return self.h;
    }
    fn get_armor_class(&self) -> u32 {
        return self.ac;
    }
    fn get_original_armor_class(&self) -> u32 {
        return self.oac;
    }
}

impl Pathogen for Virus {
    fn attack(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("Virus attack! Rolling... {}", roll);
        return roll;
    }
    fn save(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("Virus saving throw! Rolling... {}", roll);
        return roll;
    }
    fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(1..self.d);
    }
    fn take_damage(&mut self, damage: u32) -> u32 {
        if self.h - damage <= 0 {
            self.h = 0;
            return 0;
        }
        self.h -= damage;
        return self.h - damage;
    }
    fn guard(&mut self) {
        self.ac += 1;
        println!("{}", "Virus guards! Armor class increased by 1.".yellow());
    }
    fn remove_guard(&mut self) { 
        if self.ac - 1 >= 0 {
            self.ac -= 1;
        }
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
    fn get_health(&self) -> u32 {
        return self.h;
    }
    fn get_armor_class(&self) -> u32 {
        return self.ac;
    }
    fn get_original_armor_class(&self) -> u32 {
        return self.oac;
    }
}