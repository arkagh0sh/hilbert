use std::collections::{BTreeMap, BTreeSet, HashSet};
use super::atoms::*;

pub struct PartialAut<A> where A : AtomsWithOrd {
    mappings : BTreeMap<A,A>
}

impl<A : AtomsWithOrd> PartialAut<A> {
    // add a check that the map is a bijection
    pub fn new(maps  : BTreeMap<A,A>) -> Self {
        PartialAut {mappings : maps}
    }

    pub fn apply(a : A) -> A {
        if 
    }
}

pub trait SetWithAtoms<A> where A : AtomsWithOrd {

    /*
    Checks if two tuples are in the same orbit
    */

    fn tuples_in_same_orbit(first : Vec<Self>, second : Vec<Self>) -> bool
    where
        Self: Sized;
    
    /*
    Given a list of representatives of orbits, output a list of representatives of the product of the orbits
    */

    fn prod_orbit_rep(first : Self, second : Self) -> 
    Vec<Vec<Self>>
    where
        Self : Sized;

    /*
    Given a representative of an orbits and a support, output the list of all elements in the orbit supported by the given supprt
    */

    fn project_to_support(rep : Self, support : Vec<A>) -> Vec<Self>
    where
        Self : Sized;

}