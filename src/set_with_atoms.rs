use std::collections::{BTreeSet, HashSet};
use super::atoms::*;
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