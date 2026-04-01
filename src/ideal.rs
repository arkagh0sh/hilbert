use std::hash::Hash;
use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, Ordering::{Equal, Less, Greater}, max};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter, Result};
use itertools::Itertools;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use num_traits::{Zero, identities::One, Pow, ConstOne};

use crate::mon::*;

pub struct Ideal<F, X : Ord + Hash + Clone> {
    // set of generators
    gens : Vec<Polynomial<F, X>>
}

impl <F, X : Ord + Hash + Clone> Ideal<F,X> {

// Have two "new" functions.
// The first one is just taking a vector in a BTreeSet.
// The Second one should compute a Grobner basis and add that to the set of generators

    pub fn new(&self, new_gens : Vec<Polynomial<F,X>>) -> Ideal<F,X> {
        Ideal {gens : new_gens}
    }
}

pub fn buchberger<
F : AddAssign + MulAssign + Clone + ConstOne + Default + Zero + Eq + Div<Output = F> + Neg<Output = F> + Sub,
X : Ord + Hash + Clone>(bad_base : &Vec<Polynomial<F,X>>) -> Vec<Polynomial<F,X>> {
    let mut remainders : Vec<Polynomial<F,X>> = bad_base.clone();
    let mut maybe_good_base : Vec<Polynomial<F,X>> = bad_base.clone();
    let mut SPoly : Vec<Polynomial<F,X>> = vec![];
    // implement reduction by a set
    while !remainders.is_empty() {
        SPoly = maybe_good_base.clone().iter().cartesian_product(maybe_good_base.clone()).map(|(f,g)| f.clone().syzygy(&g)).collect();
        remainders = SPoly.into_iter().map(|f| f.reduce_by(&maybe_good_base)).collect();
    }
    return maybe_good_base;
}