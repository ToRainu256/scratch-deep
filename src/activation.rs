extern crate ndarray;
use ndarray::{OwnedRepr, prelude::*};

pub fn step_funtion(x: Array1<f64>) -> ArrayBase<OwnedRepr<f64>,Dim<[usize;1]>>
{
   let x = x.mapv(|a| ((a > 0.0) as u64)as f64);
   x
}

pub fn sigmoid(x: Array1<f64>) -> ArrayBase<OwnedRepr<f64>,Dim<[usize;1]>>
{
    let x = x.mapv(|a| 1.0/(1.0 + (-1.0f64*a).exp()));
    x
}

pub fn relu(x: Array1<f64>) -> ArrayBase<OwnedRepr<f64>,Dim<[usize;1]>>
{
    let x = x.mapv(|a| max(a,0.0) );
    x
}
fn max<T: std::cmp::PartialOrd>(x:T, y:T) -> T{
    if x >= y {
        x
    }else{
        y
    }
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

    #[test]
    fn testinf_sigmoid(){
        let _sigmoid = |x:f64| 1.0/(1.0 + (-1.0f64*x).exp());
        let x: Array1<f64> = array![2.0,3.0,-1.0];
        let y: Array1<f64> = array![_sigmoid(2.0),_sigmoid(3.0),_sigmoid(-1.0)];
        assert_eq!(sigmoid(x),y);
    }
}



