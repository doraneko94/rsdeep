use ndarray::*;

fn main() {
    let mut a = Array2::zeros((1000, 1000));
    for i in 0..1000 {
        for j in 0..1000 {
            a[[i, j]] = 1;
        }
    }
}