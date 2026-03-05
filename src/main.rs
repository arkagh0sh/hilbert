mod mon;
mod helpers;
mod atoms;
mod dlo;
mod set_with_atoms;
use rug::Rational;

use std::collections::BTreeSet;
use num_traits::{Zero, One, Pow};

use mon::*;

use crate::{atoms::AtomsWithOrd, dlo::DLO, helpers::count_between};

fn main() {
    let d1 = DLO::new(Rational::from(1));
    let d2 = DLO::new(Rational::from(2));
    let x : Polynomial<i8,DLO> = Polynomial::var(d1);
    let y : Polynomial<i8,DLO> = Polynomial::var(d2);
    print!("{}",(y.clone() + x).reduce_by(&y));
}