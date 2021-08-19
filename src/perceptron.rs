extern crate ndarray;
use ndarray::prelude::*;

// implement AND function by perceptron
// not using array
pub fn _AND(x1: f64, x2: f64) -> u64 {
    let (w1, w2, theta) = (0.5, 0.5, 0.7);
    let mut tmp = w1 * x1 + w2 * x2;
    if tmp <= theta {
        return 0;
    }
    1
}

// using array
pub fn AND(x1: f64, x2: f64) -> u64 {
    let x: Array1<f64> = array![x1, x2];
    let w: Array1<f64> = array![0.5, 0.5];
    let b = -0.7;

    let tmp = (w * x).sum() + b;
    if tmp <= 0. {
        return 0;
    }
    1
}
// implement NAND function by perceptron
pub fn NAND(x1: f64, x2: f64) -> u64 {
    let x: Array1<f64> = array![x1, x2];
    let w: Array1<f64> = array![-0.5, -0.5];
    let b = 0.7;

    let tmp = (w * x).sum() + b;
    if tmp <= 0. {
        return 0;
    }
    1
}

// implement OR function by perceptron
pub fn OR(x1: f64, x2: f64) -> u64 {
    let x: Array1<f64> = array![x1, x2];
    let w: Array1<f64> = array![0.5, 0.5];
    let b = -0.2;

    let tmp = (w * x).sum() + b;
    if tmp <= 0. {
        return 0;
    }
    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_AND() {
        assert_eq!(_AND(0., 0.), 0);
        assert_eq!(_AND(1., 0.), 0);
        assert_eq!(_AND(1., 1.), 1);
        assert_eq!(_AND(0., 1.), 0);
    }
    #[test]
    fn testing_weightedAND() {
        assert_eq!(AND(0., 0.), 0);
        assert_eq!(AND(1., 0.), 0);
        assert_eq!(AND(1., 1.), 1);
        assert_eq!(AND(0., 1.), 0);
    }
    #[test]
    fn testing_NAND() {
        assert_eq!(NAND(0., 0.), 1);
        assert_eq!(NAND(1., 0.), 1);
        assert_eq!(NAND(1., 1.), 0);
        assert_eq!(NAND(0., 1.), 1);
    }

    #[test]
    fn testing_OR() {
        assert_eq!(OR(0., 0.), 0);
        assert_eq!(OR(1., 0.), 1);
        assert_eq!(OR(1., 1.), 1);
        assert_eq!(OR(0., 1.), 1);
    }
}
