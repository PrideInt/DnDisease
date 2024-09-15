pub struct Opponent {
    continents: Vec<Continent>,
}

struct NorthAmerica { countries: Vec<String>, }

struct SouthAmerica { countries: Vec<String>, }

struct Africa { countries: Vec<String>, }

struct Europe { countries: Vec<String>, }

struct MiddleEast { countries: Vec<String>, }

struct Asia { countries: Vec<String>, }

struct Australia { countries: Vec<String>, }

impl Continent for NorthAmerica {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for SouthAmerica {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for Africa {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for Europe {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for MiddleEast {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for Asia {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl Continent for Australia {
    fn countries(&self) -> Vec<String> {
        self.countries.clone()
    }
}

impl NorthAmerica {
    fn new() -> NorthAmerica {
        NorthAmerica {
            countries: vec![
                "Canada".to_string(),
                "United States".to_string(),
                "Mexico".to_string(),
            ],
        }
    }
}

impl SouthAmerica {
    fn new() -> SouthAmerica {
        SouthAmerica {
            countries: vec![
                "Brazil".to_string(),
                "Argentina".to_string(),
                "Peru".to_string(),
                "Chile".to_string(),
                "Colombia".to_string(),
                "Venezuela".to_string(),
                "Ecuador".to_string(),
                "Bolivia".to_string(),
                "Paraguay".to_string(),
                "Uruguay".to_string(),
                "Guyana".to_string(),
            ],
        }
    }
}

trait Continent {
    fn countries(&self) -> Vec<String>;
}