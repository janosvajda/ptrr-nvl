// src/atom.rs

use crate::nucleus::Nucleus;

#[derive(Debug)]
pub struct Atom {
    pub nucleus: Nucleus,
    pub electrons: Vec<f64>, // Assuming each electron has a charge of -1
}

impl Atom {
    pub fn new(protons_count: usize, neutrons_count: usize, electrons_count: usize) -> Atom {
        let nucleus = Nucleus::new(protons_count, neutrons_count);
        let electrons = vec![-1.0; electrons_count]; // Each electron has a charge of -1

        Atom { nucleus, electrons }
    }

    pub fn total_charge(&self) -> f64 {
        let nucleus_charge = self.nucleus.charge();
        let electrons_charge: f64 = self.electrons.iter().sum();
        nucleus_charge + electrons_charge
    }

    pub fn display(&self) {
        println!("{:?}", self);
        println!("Total charge: {}", self.total_charge());
        self.nucleus.display(); // Display nucleus details
    }
}
