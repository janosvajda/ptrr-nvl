mod quark;
mod proton;
mod neutron;
mod nucleus;
mod atom;
mod electron; // Ensure to declare the electron module

use atom::Atom;

fn main() {
    let hydrogen = Atom::new(1, 0, 1); // 1 proton, 0 neutrons, 1 electron
    hydrogen.display();

    let deuterium = Atom::new(1, 1, 1); // 1 proton, 1 neutron, 1 electron
    deuterium.display();

    let helium = Atom::new(2, 2, 2); // 2 protons, 2 neutrons, 2 electrons
    helium.display();
}
