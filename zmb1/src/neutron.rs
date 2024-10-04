// src/neutron.rs

use crate::quark::{Quark, QuarkFlavor};

#[derive(Debug)]
pub struct Neutron {
    pub quarks: [Quark; 3],
}

impl Neutron {
    pub fn new() -> Neutron {
        Neutron {
            quarks: [
                Quark::new(QuarkFlavor::Up),    // One Up quark
                Quark::new(QuarkFlavor::Down),  // First Down quark
                Quark::new(QuarkFlavor::Down),  // Second Down quark
            ],
        }
    }

    // New method to display quark information
    pub fn display_quarks(&self) {
        for quark in &self.quarks {
            println!("{:?}", quark);
        }
    }
}
