// trait Atoms {
//     fn tuples_in_same_orbit
//     (first : Vec<Self>, second : Vec<Self>, support : Vec<Self>) -> bool
//     where
//         Self: Sized;

//     fn tuple_orbit_rep(n : u8) -> Vec<Vec<Self>>
//     where
//         Self : Sized;
// }

use std::collections::BTreeSet;

pub trait AtomsWithOrd : Ord {

    fn sets_in_same_orbit(first : &BTreeSet<Self>, second : &BTreeSet<Self>) -> bool
    where
        Self: Sized;
    
    fn orbit_reps(n : usize) -> 
    Vec<Vec<Self>>
    where
        Self : Sized;
}