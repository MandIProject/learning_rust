use std::fmt;
use std::mem::size_of;

// let's rust compiler generate Debug trait for the struct
#[derive(Debug)]
struct Footballer {
    name: String,
    country: String,
    goals_scored: u32,
    matches_played: u32,
    trophies_won: Vec<String>, // To store a list of trophies
}

impl fmt::Display for Footballer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) - Goals: {}, Matches: {}, Trophies: {}",
            self.name,
            self.country,
            self.goals_scored,
            self.matches_played,
            self.trophies_won.join(", ")
        )
    }
}

fn main() {
    let arr: [u8; 5] = [0; 5];

    println!("{:#?}", arr);

    println!("Number: {}", 42);
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Padding and alignment
    println!("Right-aligned: {:>10}", "hello"); // Pad with spaces on the left
    println!("Left-aligned: {:<10}", "hello"); // Pad with spaces on the right
    println!("Centered:     {:^10}", "hello"); // Pad with spaces on both sides

    let player = Footballer {
        name: "Lionel Messi".to_string(),
        country: "Argentina".to_string(),
        goals_scored: 796,    // Updated goal count
        matches_played: 1073, // Updated match count
        trophies_won: vec![
            "Champions League".to_string(),
            "La Liga".to_string(),
            "Copa del Rey".to_string(),
            "World Cup".to_string(),
        ],
    };

    println!("{}", player);

    println!("Data type sizes in Rust:");

    println!("bool:        {} bytes", size_of::<bool>());
    println!("char:        {} bytes", size_of::<char>());
    println!("i8:          {} bytes", size_of::<i8>());
    println!("i16:         {} bytes", size_of::<i16>());
    println!("i32:         {} bytes", size_of::<i32>());
    println!("i64:         {} bytes", size_of::<i64>());
    println!("u8:          {} bytes", size_of::<u8>());
    println!("u16:         {} bytes", size_of::<u16>());
    println!("u32:         {} bytes", size_of::<u32>());
    println!("u64:         {} bytes", size_of::<u64>());
    println!("f32:         {} bytes", size_of::<f32>());
    println!("f64:         {} bytes", size_of::<f64>());
    println!("usize:       {} bytes", size_of::<usize>());
    println!("isize:       {} bytes", size_of::<isize>());
}
