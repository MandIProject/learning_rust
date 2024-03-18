pub mod player_database {
    use std::fmt;
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
