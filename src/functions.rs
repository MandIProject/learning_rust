pub mod func_module {
    use std::str::FromStr;
    pub fn read_string() -> String {
        let mut name: String = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("cannot read user input!");
        name.trim().to_string()
    }

    pub fn number_loop(arr: &Vec<i32>) {
        for x in arr {
            print!("{x},");
        }
        println!();
    }

    pub fn read_number() -> Result<u8, String> {
        let input: String = read_string();

        if input.is_empty() {
            Err("You did not enter any data!".to_string())
        } else {
            u8::from_str(&input).or(Err("You have entered an invalid age!".to_string()))
        }
    }
}
