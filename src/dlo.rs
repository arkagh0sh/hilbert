use rug::Rational;
use std::collections::BTreeSet;
use itertools::Itertools;
use super::atoms::*;

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DLO {
    val : Rational
}

impl DLO {
    fn new(q : Rational) -> DLO {
        DLO {val : q}
    }
}

/* 
impl AtomsWithOrd for DLO {
    fn sets_in_same_orbit(
        first   : BTreeSet<Self>,
        second  : BTreeSet<Self>,
        support : BTreeSet<Self>) -> bool
        where
            Self: Sized {

                let n = first.len();
                
                if n != second.len() {
                    return false;
                }

                let first_diff : Vec<DLO> = first.difference(&support).cloned().collect();
                let second_diff : Vec<DLO> = second.difference(&support).cloned().collect();

                if first_diff.len() != second_diff.len() {
                    return false;
                }

                let first_counts: Vec<usize> = first_diff.iter()
        .tuple_windows()
        .map(|(&low, &high)| a.range(low..high).count())
        .collect();
    }
}
    */