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
    gens : BTreeSet<Polynomial<F, X>>
}

// create a ring trait to encapsulate all these traits
impl <F : Neg<Output = F> + Clone + Ord + AddAssign + Default + Zero + ConstOne,
      X : Ord + Hash + Clone> Ideal<F,X> {

// Have two "new" functions.
// The first one is just taking a vector in a BTreeSet.
// The Second one should compute a Grobner basis and add that to the set of generators

    pub fn new(&self, new_gens : Vec<Polynomial<F,X>>) -> Ideal<F,X> {
        Ideal {gens : new_gens.into_iter().collect()}
    }
}

pub fn buchberger<
F : AddAssign + MulAssign + Clone + ConstOne + Default + Zero + Eq + Div<Output = F> + Neg<Output = F> + Sub + Display + Ord,
X : Ord + Hash + Clone + Display>
(bad_base : Vec<Polynomial<F,X>>) -> Vec<Polynomial<F,X>> {
    let mut remainders : Vec<Polynomial<F,X>> = bad_base;
    let mut maybe_good_base : Vec<Polynomial<F,X>> = vec![];
    let mut SPoly : Vec<Polynomial<F,X>> = vec![];
    let zero_poly = Polynomial::zero();
    // implement reduction by a set
    while !remainders.is_empty() {
        maybe_good_base.extend(remainders);

        SPoly = maybe_good_base
        .clone()
        .iter()
        .cartesian_product(maybe_good_base.clone())
        .map(|(f,g)| f.clone().syzygy(&g))
        .collect();

        remainders = SPoly.into_iter().map(|h| h.remainder(&maybe_good_base)).filter(|h| *h != zero_poly).collect();
        println!("{:?}",remainders);
    }
    let good_base_set : BTreeSet<Polynomial<F,X>> = maybe_good_base.into_iter().collect();
    good_base_set.into_iter().collect()
}