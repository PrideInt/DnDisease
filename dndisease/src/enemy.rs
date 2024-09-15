use rand::Rng;

pub struct Opponent {
    name: String,
    d: u32,
}

pub fn new_opponent(name: &str, d: u32) -> Opponent {
    return Opponent {
        name: name.to_string(),
        d: d
    }
}

impl Opponent {
    pub fn save(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("{} saving throw! Rolling... {}", self.name, roll);
        return roll;
    }
    pub fn attack(&self) -> u32 {
        let roll: u32 = self.roll();
        println!("{} attack! Rolling... {}", self.name, roll);
        return roll;
    }
    pub fn roll(&self) -> u32 {
        return rand::thread_rng().gen_range(1..self.d);
    }
}