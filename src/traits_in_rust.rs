pub mod traits_in_rust_module {
    pub trait Shape {
        fn area(&self) -> f64;
    }

    pub trait Fruitiness {
        fn is_sweet(&self) -> bool {
            self.sweetness() >= 0.5
        }

        fn sweetness(&self) -> f32;
    }

    pub struct Circle {
        pub radius: f64,
    }

    pub struct Square {
        pub side: f64,
    }

    pub struct Pear {}

    pub struct Apple {}

    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius.powi(2)
        }
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side.powi(2)
        }
    }

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

    //static dispatch
    pub fn print_area<S: Shape>(shape: S) {
        println!("Area: {}", shape.area());
    }

    pub fn print_sweetness(id: &str, fruit: impl Fruitiness) {
        println!("{} is sweet? {}", id, fruit.is_sweet());
    }
}
