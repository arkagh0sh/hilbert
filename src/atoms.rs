// trait Atoms {
//     fn tuples_in_same_orbit
//     (first : Vec<Self>, second : Vec<Self>, support : Vec<Self>) -> bool
//     where
//         Self: Sized;

//     fn tuple_orbit_rep(n : u8) -> Vec<Vec<Self>>
//     where
//         Self : Sized;
// }
use std::hash::Hash;
use std::collections::BTreeSet;
use std::marker::PhantomData;
use super::elements_with_atoms::*;

pub struct Orbit<A : AtomsWithOrd, B> {
    rep : B,
    rep_support : BTreeSet<A>
}

impl<A, B>  PartialEq  for Orbit<A, B>
where
    A : AtomsWithOrd + Clone + Hash,
    B : ElementsWithAtoms<A>,
{
    fn eq(&self, other: &Self) -> bool {
        A::in_same_orbit(&self.rep_support,&other.rep_support)
    }
}

/*
Since we mean to use it only for polynomials,
we only work with equivariant orbits.
*/
pub trait AtomsWithOrd : Ord {

    fn in_same_orbit(first : &BTreeSet<Self>, second : &BTreeSet<Self>) -> bool
    where
        Self: Sized;
    
/*
For this one we need repeating tuples also
*/

    fn orbit(n : usize) -> 
    Vec<Orbit<BTreeSet<Self>>>
    where
        Self : Sized;
}


/*
  atoms to add
    1. Equality
    2. Ordered
    3. Bit-vectors
    4. Dense-tree
    5. Rado graph
    6. Ordered Rado
    7. Homogenous (as a subtrait)
    8. Finite sets
    9. Products
    10. Finitely supported (atoms where some values are fixed as constants)
    11. Nesting of atoms
*/