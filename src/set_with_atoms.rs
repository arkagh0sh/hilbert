use std::collections::{BTreeSet, HashSet};
use super::atoms::*;
pub trait SetWithAtoms<A> where A : AtomsWithOrd {

    /*
    Checks if two tuples are in the same orbit
    */

    fn tuples_in_same_orbit(first : Vec<Self>, second : Vec<Self>, support : BTreeSet<A>) -> bool
    where
        Self: Sized;
    
    /*
    Given two elements first_rep and second_rep, and a support, gives representatives of the elements of the orbits inside the product of orbits of first_rep and second_rep
    */

    fn prod_orbit_rep(first_rep : Self, second_rep : Self, support : BTreeSet<A>) -> 
    BTreeSet<Self>
    where
        Self : Sized;
}