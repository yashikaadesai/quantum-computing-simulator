use ndarray::prelude::*;
use ndarray::array;

pub fn hadamard_single() -> Array2<f64> {
    array![
        [1.0 / 2f64.sqrt(), 1.0 / 2f64.sqrt()],
        [1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt()]
    ]
}

pub fn hadamard(n: usize) -> Array2<f64> {
    let h = hadamard_single();
    let mut hadamard_gate = h.clone();

    for _ in 1..n {
        hadamard_gate = tensor_product(&hadamard_gate, &h);
    }

    hadamard_gate
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

pub fn tensor_product(a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
    let mut result = Array2::zeros((a.nrows() * b.nrows(), a.ncols() * b.ncols()));
    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            for k in 0..b.nrows() {
                for l in 0..b.ncols() {
                    result[(i * b.nrows() + k, j * b.ncols() + l)] = a[(i, j)] * b[(k, l)];
                }
            }
        }
    }
    result
}