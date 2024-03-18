pub mod number_module {
    pub fn print_arr(arr: &[u8]) {
        println!("{:#?}", arr);
    }

    pub fn print_number(number: &u16) {
        println!("Number: {}", number);
        println!(
            "Binary: {:b}, Hex: {:x}, Octal: {:o}",
            number, number, number
        );
    }
}
