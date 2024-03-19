pub mod player_database {
    use std::fmt;
    use std::fs::{read_to_string, write};
    use std::str::FromStr;
    #[derive(Debug)]
    pub struct Footballer {
        name: String,
        country: String,
        goals_scored: u32,
        matches_played: u32,
        trophies_won: Vec<String>, // To store a list of
        address: Addressoffootballer,
    }

    #[derive(Debug)]
    pub struct Addressoffootballer {
        street: String,
        city: String,
        postal_code: u16,
    }

    impl Footballer {
        pub fn new(
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

        pub fn write_player_data(&self) -> std::io::Result<()> {
            let mut output_buffer: String = String::new();
            output_buffer.push_str(&self.name);
            output_buffer.push('\n');
            output_buffer.push_str(&self.country);
            output_buffer.push('\n');
            output_buffer.push_str(&self.goals_scored.to_string());
            output_buffer.push('\n');
            output_buffer.push_str(&self.matches_played.to_string());
            output_buffer.push('\n');
            for trophy in &self.trophies_won {
                output_buffer.push_str(trophy);
                output_buffer.push('\n');
            }
            output_buffer.push_str(&self.address.city);
            output_buffer.push('\n');
            output_buffer.push_str(&self.address.street);
            output_buffer.push('\n');
            output_buffer.push_str(&self.address.postal_code.to_string());
            write("player_data.txt", output_buffer)
        }

        pub fn read_player_data() -> Result<self::Footballer, std::io::Error> {
            let input: String = read_to_string("player_data.txt")?;
            let mut lines: std::str::Split<'_, char> = input.split('\n');
            let name: String = lines.next().unwrap_or("").to_string();
            let country: String = lines.next().unwrap_or("").to_string();
            let goals_scored: u32 = u32::from_str(lines.next().unwrap_or("0")).unwrap_or(0);
            let matches_played: u32 = u32::from_str(lines.next().unwrap_or("0")).unwrap_or(0);
            let trophies_won: Vec<String> = vec![
                lines.next().unwrap_or("").to_string(),
                lines.next().unwrap_or("").to_string(),
                lines.next().unwrap_or("").to_string(),
            ];
            let city: String = lines.next().unwrap_or("").to_string();
            let street: String = lines.next().unwrap_or("").to_string();
            let pincode: u16 = u16::from_str(lines.next().unwrap_or("0")).unwrap_or(0);

            let player_address: Addressoffootballer =
                Addressoffootballer::new(street, city, pincode);
            let player_data: Footballer = Footballer::new(
                name,
                country,
                goals_scored,
                matches_played,
                trophies_won,
                player_address,
            );

            Ok(player_data)
        }

        pub fn displayplayerdata(&self) {
            println!("{}", self);
        }
    }

    impl Addressoffootballer {
        pub fn new(street: String, city: String, postal_code: u16) -> Addressoffootballer {
            Addressoffootballer {
                street,
                city,
                postal_code,
            }
        }

        pub fn displaylayeraddress(&self) {
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
}
