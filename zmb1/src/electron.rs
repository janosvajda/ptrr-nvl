#[derive(Debug, Clone)] // Add Clone to the derive attributes
#[allow(dead_code)]
pub struct Electron {
    pub charge: f64,
    pub position: (f64, f64, f64), // x, y, z coordinates
    pub velocity: (f64, f64, f64), // velocity components
}

impl Electron {
    pub fn new() -> Electron {
        Electron {
            charge: -1.0, // Charge of the electron
            position: (0.0, 0.0, 0.0), // Starting at the nucleus
            velocity: (0.0, 0.0, 0.0), // Initial velocity can be randomized
        }
    }
}
