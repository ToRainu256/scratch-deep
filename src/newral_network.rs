extern crate ndarray;
use ndarray::{OwnedRepr, prelude::*};

pub fn step_funtion(x: Array1<f64>) -> ArrayBase<OwnedRepr<f64>,Dim<[usize;1]>>
{
   let x = x.mapv(|a| ((a > 0.0) as u64)as f64);
   x
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn testing_step_function(){
        let x: Array1<f64> = array![-1.0,2.0,0.0];
        let y: Array1<f64> = array![0.,1.,0.];
        assert_eq!(step_funtion(x),y);
    }
}



