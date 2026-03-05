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

/*
Since we mean to use it only for polynomials,
we only work with equivariant orbits.
*/
pub trait AtomsWithOrd : Ord {

    fn in_same_orbit(first : &Vec<Self>, second : &Vec<Self>) -> bool
    where
        Self: Sized;
    
/*
For this one we need repeating tuples also
*/

    fn orbit_reps(n : usize) -> 
    Vec<Vec<Self>>
    where
        Self : Sized;
}