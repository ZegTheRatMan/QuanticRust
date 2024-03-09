// module of qubits definitions
pub mod definition {
    
    // COMPLEX definition
    #[derive(Debug)]
    pub struct Complex {
        pub real_part: f64,
        pub imag_part: f64
    }

    // QUBIT structure
    #[derive(Debug)]
    pub struct Qubit {
        pub zer_part: Complex,
        pub one_part: Complex
    }

    pub fn create_complex(real: f64, imag: f64) -> Complex {
        Complex { 
            real_part: real, imag_part: imag 
        }
    }

    pub fn create_qubit(alpha: Complex, beta: Complex) -> Qubit {
        Qubit { 
            zer_part: alpha, 
            one_part: beta 
        }
    }
}
