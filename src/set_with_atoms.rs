use core::panic;
use std::hash::Hash;
use std::clone::Clone;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use super::atoms::*;
use super::helpers::*;

pub struct PartialAut<A> where A : AtomsWithOrd {
    pub domain : Vec<A>,
    pub range  : Vec<A>
}

impl<A : Hash + AtomsWithOrd + Clone + Copy> PartialAut<A> {
    // add a check that the map is a bijection
    pub fn new(domain : &Vec<A>, range : &Vec<A>) -> Self {
        if domain.len() != range.len() {
            panic!("dom and range must be of same length");
        } else if !has_unique_elements(domain.clone()){
            panic!("dom must be a tuple of non-repeating elements");
        } else if !has_unique_elements(range.clone()) {
            panic!("range must be a tuple of non-repeating elements");
        } else if !A::in_same_orbit(domain, range) {
            panic!("dom and range must be in the same orbit");
        }
        else {
            PartialAut {domain : domain.clone(), range : range.clone()}
        }
    }
}

pub trait ApplyPAut<A : AtomsWithOrd> {
    fn apply_paut(&self, paut : &PartialAut<A>) -> Self;
}

/*
Since we mean to use it only for polynomials,
we only work with equivariant orbits.
*/
pub trait SetWithAtoms<A : AtomsWithOrd> : ApplyPAut<A> {
    /*
    Checks if two tuples are in the same orbit
    */

    fn tuples_in_same_orbit(first : &BTreeSet<Self>, second : &BTreeSet<Self>) -> bool
    where
        Self: Sized;
    
    /*
    Given a list of representatives of orbits, output a list of representatives of the product of the orbits
    */

    fn prod_orbit_rep(orbits : &Vec<Self>) -> 
    Vec<Vec<Self>>
    where
        Self : Sized;

    /*
    Given a representative of an orbits and a support, output the list of all elements in the orbit supported by the given supprt
    */

    fn project_to_support(rep : &Self, support : &Vec<A>) -> Vec<Self>
    where
        Self : Sized;

    fn apply_PAut(&self, paut : PartialAut<A>) -> Self
    where
        Self : Sized;
}

// impl SetWithAtoms<A : AtomsWithOrd> for Vec<A> {

// }