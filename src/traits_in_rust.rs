pub mod traits_in_rust_module {
    pub trait Fruitiness {
        fn is_sweet(&self) -> bool {
            self.sweetness() >= 0.5
        }

        fn sweetness(&self) -> f32;
    }

    pub struct Pear {}

    pub struct Apple {}
    
    impl Fruitiness for Pear {
        fn sweetness(&self) -> f32 {
            0.6
        }
    }

    impl Fruitiness for Apple {
        fn sweetness(&self) -> f32 {
            0.3
        }
    }

    pub fn print_sweetness(id: &str, fruit: impl Fruitiness) {
        println!("{} is sweet? {}", id, fruit.is_sweet());
    }
}