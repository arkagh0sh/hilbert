use rug::Rational;
use std::{collections::{BTreeSet,BTreeMap} , fmt::{Debug, Formatter, Result, Display}};
use itertools::Itertools;
use crate::elements_with_atoms::{ElementsWithAtoms, PartialAut};
use contracts::post;

use super::{atoms::*, helpers::*};


// the trait for dense linear order without endpoints
pub trait DLO : Ord {

    #[post(lower < ret)]
    #[post(upper > ret)]
    fn find_between(lower : Self, upper : Self) -> Self;

    // just to ensure that the set is non empty
    fn an_elem() -> Self;

    #[post(lower < ret)]
    fn bigger(lower : Self) -> Self;

    #[post(upper > ret)]
    fn smaller(upper : Self) -> Self;
}

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DLO_var {
    val : Rational
}

impl DLO_var {
    pub fn new(q : Rational) -> DLO_var {
        DLO_var {val : q}
    }
}

impl Debug for DLO_var {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut to_view : String = String::new();
        to_view = to_view + "d(" + &self.val.to_string() + ")";
        write!(f,"{}",to_view)
    }
}

impl Display for DLO_var {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut to_view : String = String::new();
        to_view = to_view + "d(" + &self.val.to_string() + ")";
        write!(f,"{}",to_view)
    }
}
 

impl <X : DLO> AtomsWithOrd for X    {

    fn in_same_orbit(first : &Vec<Self>, second : &Vec<Self>) -> bool
        where
            Self: Sized {
                let n = first.len();
                if n!= second.len() {
                    false
                } else {
                    let mut answer = true;
                    for i in 0..n {
                        for j in 0..n {
                            if ((first[i] == first[j]) != (second[i] == second[j])) || ((first[i] < first[j]) != (second[i] < second[j])) {
                                answer = false
                            }
                        }
                    }
                    answer
                }
    }

    fn orbit_reps(n : usize) -> 
        Vec<Vec<Self>>
        where
            Self : Sized {
        // needs to be fixed
        let nums : Vec<Self>= (1..(n+1)).map(|i| DLO::new(Rational::from(i))).collect();

        let all_rep : Vec<Vec<DLO>> = (0..n)
        .map(|_| nums.iter().cloned())
        .multi_cartesian_product()
        .collect();

        let mut unique_reps = Vec::new();
        for rep in all_rep {
            if !unique_reps.iter().any(|re| Self::in_same_orbit(re, &rep)) {
                unique_reps.push(rep);
            }
        }
        return unique_reps;
    }
}

impl ElementsWithAtoms<DLO> for BTreeSet<DLO> {

    fn support(&self) -> Vec<DLO> {
        self.clone().into_iter().collect()
    }

    fn apply_paut(&self, paut : PartialAut<DLO>) -> Self
        where
            Self : Sized {
                let mappings: BTreeMap<_, _> = paut.domain.clone().into_iter().zip(paut.range.into_iter()).collect();

                // maybe support should be calculated after initialisation to make it faster
                if !is_subvec(&self.support(), &paut.domain) {
                    panic!("Domain of PartialAut must be a superset of the support of the element")
                }


                self.clone()
                .into_iter()
                .filter_map(|key| mappings.get(&key))
                .cloned() // Copy or Clone the value out of the map
                .collect() 
    }
}