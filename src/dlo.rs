use rug::Rational;
<<<<<<< HEAD
use std::{collections::BTreeSet, fmt::{Debug, Formatter, Result}};
=======
use std::{fmt::{Debug, Display, Formatter, Result}};
>>>>>>> 5854a52 (small changes)
use itertools::Itertools;
use super::atoms::*;

#[derive(Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

impl Display for DLO {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut to_view : String = String::new();
        to_view = to_view + "d(" + &self.val.to_string() + ")";
        write!(f,"{}",to_view)
    }
}
 

impl AtomsWithOrd for DLO {
    fn sets_in_same_orbit(first : BTreeSet<Self>, second : BTreeSet<Self>) -> bool
        where
            Self: Sized {
                let n = first.len();
                if n!= second.len() {
                    return false
                } else {
                    let mut answer = true;
                    for i in 1..n {
                        for j in 1..n {
                            answer = answer &&
                            ((first[i] < first[j]) == (second[i] < second[j])) &&
                            ((first[i] == first[j]) == (second[i] == second[j]));
                        }
                    }
                    return answer;
                }
    }

    fn orbit_reps(n : u8) -> 
        Vec<Vec<Self>>
        where
            Self : Sized {
        let nums : Vec<DLO>= (1..(n+1)).map(|i| DLO::new(Rational::from(i))).collect();
        return nums.iter().cloned().permutations(nums.len()).collect();
    }
}