pub mod number_module {
    pub fn number_print(arr: &[u8]) {
        println!("{:#?}", arr);
    }

    pub fn display_prints() {
        println!("Number: {}", 42);
        println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

        // Padding and alignment
        println!("Right-aligned: {:>10}", "hello"); // Pad with spaces on the left
        println!("Left-aligned: {:<10}", "hello"); // Pad with spaces on the right
        println!("Centered:     {:^10}", "hello"); // Pad with spaces on bothside
    }
}
