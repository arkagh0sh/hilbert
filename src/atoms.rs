use std::collections::{BTreeSet, HashSet};

// trait Atoms {
//     fn tuples_in_same_orbit
//     (first : Vec<Self>, second : Vec<Self>, support : Vec<Self>) -> bool
//     where
//         Self: Sized;

//     fn tuple_orbit_rep(n : u8) -> Vec<Vec<Self>>
//     where
//         Self : Sized;
// }

pub trait AtomsWithOrd : Ord {

    fn sets_in_same_orbit(first : BTreeSet<Self>, second : BTreeSet<Self>, support : BTreeSet<Self>) -> bool
    where
        Self: Sized;
    
    fn set_orbit_rep(n : u8, support : BTreeSet<Self>) -> 
    BTreeSet<BTreeSet<Self>>
    where
        Self : Sized;
}