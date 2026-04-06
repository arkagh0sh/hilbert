use rug::Rational;
use std::{collections::{BTreeSet,BTreeMap} , fmt::{Debug, Formatter, Result, Display}};
use itertools::Itertools;
use crate::elements_with_atoms::{ElementsWithAtoms, PartialAut};
use contracts::{pre, post};

use super::{atoms::*, helpers::*};


// the trait for dense linear order without endpoints

#[derive(PartialEq, PartialOrd,Eq,Ord)]
pub struct DLO {
    val : Rational
}


impl DLO {
    pub fn new(q : Rational) -> DLO {
        DLO {val : q}
    }
}

impl Debug for DLO {
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