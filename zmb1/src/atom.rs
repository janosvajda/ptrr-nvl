use crate::nucleus::Nucleus;
use crate::electron::Electron; // Import the Electron struct

#[derive(Debug)]
pub struct Atom {
    pub nucleus: Nucleus,
    pub electrons: Vec<Electron>, // Store instances of Electron
}

impl Atom {
    pub fn new(protons_count: usize, neutrons_count: usize, electrons_count: usize) -> Atom {
        let nucleus = Nucleus::new(protons_count, neutrons_count);
        let electrons = vec![Electron::new(); electrons_count]; // Create electrons

        Atom { nucleus, electrons }
    }

    pub fn total_charge(&self) -> f64 {
        let nucleus_charge = self.nucleus.charge(); // Assuming the Nucleus struct has a charge() method
        let electrons_charge: f64 = self.electrons.iter().map(|e| e.charge).sum(); // Calculate total charge of electrons
        nucleus_charge + electrons_charge
    }

    pub fn display(&self) {
        println!("{:?}", self);
        println!("Total charge: {}", self.total_charge());
        self.nucleus.display(); // Display nucleus details
        // Optionally display details about each electron
        for electron in &self.electrons {
            println!("Electron: {:?}", electron);
        }
    }
}
