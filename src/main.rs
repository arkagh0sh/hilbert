mod mon;
mod ideal;
mod helpers;
mod atoms;
mod dlo;
mod set_with_atoms;
use rug::Rational;

use std::collections::BTreeSet;
use num_traits::{Zero, One, Pow};

use mon::*;

use crate::{
    atoms::AtomsWithOrd, dlo::DLO, helpers::count_between, ideal::buchberger, set_with_atoms::{PartialAut, SetWithAtoms}
};

fn main() {
    let d1 = DLO::new(Rational::from(1));
    let d2 = DLO::new(Rational::from(2));
    let d3 = DLO::new(Rational::from(3));
    let d4 = DLO::new(Rational::from(4));
    let x : Polynomial<i8,DLO> = Polynomial::var(d1.clone());
    let y : Polynomial<i8,DLO> = Polynomial::var(d2.clone());
    let set_poly : Vec<Polynomial<i8,DLO>> = vec![x,y];
    print!("{:?}",buchberger(set_poly));
}