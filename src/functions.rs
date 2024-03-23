pub mod func_module {
    use std::f64::consts::PI;
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

    pub fn circle(radius: f64) -> (f64, f64) {
        let area: f64 = PI * radius.powi(2);
        let circumference: f64 = 2.0 * PI * radius;

        (area, circumference)
    }

    pub fn reverse_a_string(input: &str) -> String {
        input.chars().rev().collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test_reverse_string() {
            assert_eq!(reverse_a_string("Hello World"), "dlroW olleH");
        }
    }
}
