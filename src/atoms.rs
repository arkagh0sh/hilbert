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

    fn tuples_in_same_orbit(first : Vec<Self>, second : Vec<Self>) -> bool
    where
        Self: Sized;
    
    fn orbit_reps(n : u8) -> 
    Vec<Vec<Self>>
    where
        Self : Sized;
}