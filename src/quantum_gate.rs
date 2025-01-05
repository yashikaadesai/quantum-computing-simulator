use ndarray::prelude::*;
use ndarray::array;

pub fn hadamard() -> Array2<f64> {
    array![
        [1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()],
        [1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt()]
    ]
}

pub fn pauli_x() -> Array2<f64> {
    array![
        [0.0, 1.0],
        [1.0, 0.0]
    ]
}

pub fn cnot() -> Array2<f64> {
    array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.0]
    ]
}