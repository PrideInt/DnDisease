use rand::Rng;

pub struct Bacteria { name: String, d: u32, }

pub struct Virus { name: String, d: u32, }

pub struct Fungi { name: String, d: u32, }

pub struct Protozoa { name: String, d: u32, }

pub struct Prion { name: String, d: u32, }

pub fn new_bacteria() -> Bacteria {
    return Bacteria {
        name: "Bacteria".to_string(),
        d: 6
    }
}

pub fn new_virus() -> Virus {
    return Virus {
        name: "Virus".to_string(),
        d: 10
    }
}

pub fn new_fungi() -> Fungi {
    return Fungi {
        name: "Fungi".to_string(),
        d: 8
    }
}

pub fn new_protozoa() -> Protozoa {
    return Protozoa {
        name: "Protozoa".to_string(),
        d: 12
    }
}

pub fn new_prion() -> Prion {
    return Prion {
        name: "Prion".to_string(),
        d: 20
    }
}

impl Pathogen for Bacteria {
    fn attack(&self) {
        println!("Bacteria attack! Rolling... {}", self.roll());
    }
    fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(1..self.d);
    }
}

impl Pathogen for Virus {
    fn attack(&self) {
        println!("Virus attack! Rolling... {}", self.roll());
    }
    fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(1..self.d);
    }
}

pub trait Pathogen {
    fn attack(&self);
    fn roll(&self) -> u32;
}