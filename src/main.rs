use std::fmt;
use numbers::number_module;
use sizes::size_module;

pub mod numbers;
pub mod sizes;

// let's rust compiler generate Debug trait for the struct
#[derive(Debug)]
struct Footballer {
    name: String,
    country: String,
    goals_scored: u32,
    matches_played: u32,
    trophies_won: Vec<String>, // To store a list of
    address: Addressoffootballer,
}

#[derive(Debug)]
struct Addressoffootballer {
    street: String,
    city: String,
    postal_code: u16,
}

impl Footballer {
    fn new(
        name: String,
        country: String,
        goals_scored: u32,
        matches_played: u32,
        trophies_won: Vec<String>,
        address: Addressoffootballer,
    ) -> Footballer {
        Footballer {
            name,
            country,
            goals_scored,
            matches_played,
            trophies_won,
            address,
        }
    }

    fn displayplayerdata(&self) {
        println!("{}", self);
    }
}

impl Addressoffootballer {
    fn new(street: String, city: String, postal_code: u16) -> Addressoffootballer {
        Addressoffootballer {
            street,
            city,
            postal_code,
        }
    }

    fn displaylayeraddress(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Footballer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - Goals: {}, Matches: {}, Trophies: {}, Address: {}",
            self.name,
            self.country,
            self.goals_scored,
            self.matches_played,
            self.trophies_won.join(", "),
            self.address,
        )
    }
}

impl fmt::Display for Addressoffootballer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Street: {}, City: {}, Postal Code: {}",
            self.street, self.city, self.postal_code
        )
    }
}

fn read_string() -> String {
    let mut name: String = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("cannot read user input!");
    name.trim().to_string()
}

fn main() {
    let arr: [u8;5] = [0;5];
    number_module::number_print(&arr);
    number_module::display_prints();

    println!("Enter your name: ");
    let name: String = read_string();

    size_module::get_input(name);
    size_module::print_size();
    
    let player_address: Addressoffootballer =
        Addressoffootballer::new("Roman Street".to_string(), "Barcelona".to_string(), 40030);
    
    player_address.displaylayeraddress();
    
    let player_data: Footballer = Footballer::new(
        "Lionel Messi".to_string(),
        "Argentina".to_string(),
        700,
        1073,
        vec![
            "Champions League".to_string(),
            "La Liga".to_string(),
            "Copa del Rey".to_string(),
        ],
        player_address,
    );

    player_data.displayplayerdata();
}
