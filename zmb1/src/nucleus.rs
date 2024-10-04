// src/nucleus.rs

use crate::proton::Proton;
use crate::neutron::Neutron;

#[derive(Debug)]
pub struct Nucleus {
    pub protons: Vec<Proton>,
    pub neutrons: Vec<Neutron>,
}

impl Nucleus {
    pub fn new(protons_count: usize, neutrons_count: usize) -> Nucleus {
        Nucleus {
            protons: (0..protons_count).map(|_| Proton::new()).collect(),
            neutrons: (0..neutrons_count).map(|_| Neutron::new()).collect(),
        }
    }

    pub fn charge(&self) -> f64 {
        self.protons.iter().map(|p| p.charge()).sum()
    }

    // Method to display protons and neutrons
    pub fn display(&self) {
        for proton in &self.protons {
            proton.display_quarks();
        }
        for neutron in &self.neutrons {
            neutron.display_quarks();
        }
    }
}
