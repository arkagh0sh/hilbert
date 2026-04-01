mod mon;
mod helpers;
mod atoms;
mod dlo;
mod set_with_atoms;
use rug::Rational;

use std::collections::BTreeSet;
use num_traits::{Zero, One, Pow};

use mon::*;

use crate::{atoms::AtomsWithOrd, dlo::DLO, helpers::count_between, set_with_atoms::{PartialAut, SetWithAtoms}};

fn main() {
    let d1 = DLO::new(Rational::from(1));
    let d2 = DLO::new(Rational::from(2));
    let d3 = DLO::new(Rational::from(3));
    let d4 = DLO::new(Rational::from(4));
    let x : Polynomial<i8,DLO> = Polynomial::var(d1.clone());
    let y : Polynomial<i8,DLO> = Polynomial::var(d2.clone());
    let paut = PartialAut::new(&vec![d1.clone(),d3.clone()], &vec![d2.clone(),d4.clone()]);
    // print!("{}",(y.clone() + x).reduce_by(&y));
    // print!("{:?}",DLO::orbit_reps(3));
    let to_move : BTreeSet<DLO> =  vec![d4.clone()].into_iter().collect();
    print!("{:?}", to_move.apply_paut(paut));
}