pub mod figure {
    pub enum Shape {
        Circle(f64),
        Rectangle { width: f64, height: f64 },
    }
}

pub mod math_utils {

    pub fn max_of_two(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    pub fn min_of_two(a: f64, b: f64) -> f64 {
        if a < b { a } else { b }
    }
}
