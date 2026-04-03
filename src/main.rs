mod mon;
mod ideal;
mod helpers;
mod atoms;
mod dlo;
mod elements_with_atoms;
mod bits;
use rug::Rational;

use std::collections::BTreeSet;
use num_traits::{Zero, One, Pow};

use mon::*;
use bits::*;

use crate::{
    atoms::AtomsWithOrd, dlo::DLO, helpers::count_between, ideal::buchberger, elements_with_atoms::{PartialAut, ElementsWithAtoms}
};

fn main() {
    let d1 = DLO::new(Rational::from(1));
    let d2 = DLO::new(Rational::from(2));
    let d3 = DLO::new(Rational::from(3));
    let d4 = DLO::new(Rational::from(4));
    let x : Polynomial<F2,String> = Polynomial::var("x".to_string());
    let y : Polynomial<F2,String> = Polynomial::var("y".to_string());
    let z : Polynomial<F2,String> = Polynomial::var("z".to_string());
    let f = x.clone() + y.clone()*z.clone();
    let g = x.clone()*y.clone();
    let vec_poly = vec![g,f];
    println!("{:?}",buchberger(vec_poly));
}