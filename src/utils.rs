use ndarray::prelude::*;

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