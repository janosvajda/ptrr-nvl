// src/quark.rs

#[derive(Debug)]
pub enum QuarkFlavor {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Quark {
    pub charge: f64,
}

impl Quark {
    pub fn new(flavor: QuarkFlavor) -> Quark {
        let charge = match flavor {
            QuarkFlavor::Up => 2.0 / 3.0,  // Charge for Up quark
            QuarkFlavor::Down => -1.0 / 3.0, // Charge for Down quark
        };

        Quark { charge }
    }
}
