// electron.rs
#[derive(Debug)]
#[allow(dead_code)]
pub struct Electron {
    pub charge: f64,
    pub position: (f64, f64, f64), // x, y, z coordinates
    pub velocity: (f64, f64, f64), // velocity components
}

impl Electron {
    pub fn new() -> Electron {
        Electron {
            charge: -1.0,
            position: (0.0, 0.0, 0.0), // Starting at the nucleus
            velocity: (0.0, 0.0, 0.0), // Initial velocity can be randomized
        }
    }

    // Update position based on velocity
    pub fn update_position(&mut self, time_step: f64) {
        self.position.0 += self.velocity.0 * time_step;
        self.position.1 += self.velocity.1 * time_step;
        self.position.2 += self.velocity.2 * time_step;
    }
}
