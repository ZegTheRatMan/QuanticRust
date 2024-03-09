use qubit::model::definition;
use qubit::helpers::print_qubit;

use crate::qubit::model::definition::{Complex, Qubit};

mod qubit;

fn main() {
    println!("Hello World");
    let my_alpha: Complex = definition::create_complex(0.0, 0.0);
    let my_beta: Complex = definition::create_complex(1.0, 0.0);
    let my_qubit: Qubit = definition::create_qubit(my_alpha, my_beta);
    print_qubit(&my_qubit);
}