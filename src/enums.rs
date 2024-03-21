pub mod enums_module {
    pub enum Direction {
        Up(u32),
        Down(u32),
        Right(u32),
        Left(u32),
    }

    pub fn move_body(direction: Direction) {
        match direction {
            Direction::Up(velocity) => println!("Moving up with velocity {} m/s", velocity),
            Direction::Down(velocity) => println!("Moving down with velocity {} m/s", velocity),
            Direction::Left(velocity) => println!("Moving left with velocity {} m/s", velocity),
            Direction::Right(velocity) => println!("Moving right with velocity {} m/s", velocity),
        }
    }
}
