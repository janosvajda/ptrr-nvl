// src/proton.rs

use crate::quark::{Quark, QuarkFlavor};

#[derive(Debug)]
pub struct Proton {
    pub quarks: [Quark; 3],
}

impl Proton {
    pub fn new() -> Proton {
        Proton {
            quarks: [
                Quark::new(QuarkFlavor::Up),    // First Up quark
                Quark::new(QuarkFlavor::Up),    // Second Up quark
                Quark::new(QuarkFlavor::Down),   // One Down quark
            ],
        }
    }

    pub fn charge(&self) -> f64 {
        self.quarks.iter().map(|q| q.charge).sum()
    }

    // New method to display quark information
    pub fn display_quarks(&self) {
        for quark in &self.quarks {
            println!("{:?}", quark);
        }
    }
}
