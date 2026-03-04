use core::panic;
use std::hash::Hash;
use std::clone::Clone;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use super::atoms::*;
use super::helpers::*;

pub struct PartialAut<A> where A : AtomsWithOrd {
    dom   : Vec<A>,
    range : Vec<A>
}

impl<A : Hash + AtomsWithOrd + Clone> PartialAut<A> {
    // add a check that the map is a bijection
    pub fn new(dom : Vec<A>, range : Vec<A>) -> Self {
        if dom.len() != range.len() {
            panic!("dom and range must be of same length");
        } else if !has_unique_elements(dom.clone()){
            panic!("dom must be a tuple of non-repeating elements");
        } else if !has_unique_elements(range.clone()) {
            panic!("range must be a tuple of non-repeating elements");
        } else if !AtomsWithOrd::tuples_in_same_orbit(dom.clone(), range.clone()) {
            panic!("dom and range must be in the same orbit");
        }
        else {
            PartialAut { dom, range}
        }
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