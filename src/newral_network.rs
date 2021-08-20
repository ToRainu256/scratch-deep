extern crate ndarray;
use ndarray::prelude::*;

pub fn step_funtion(x: Array1:<f64>) -> Array1:<u64> {
   x.mapv(|a| (a > 0) as u64);
   x
}
