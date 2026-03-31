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

/*
Since we mean to use it only for polynomials,
we only work with equivariant orbits.
*/
pub trait SetWithAtoms<A : AtomsWithOrd + Clone + Hash + Copy> : Eq {

    // Applies a partial automorphism to an element
    fn apply_paut(&self, paut : PartialAut<A>) -> Self
    where
        Self : Sized;

    //Gives the support of an element.
    fn support(&self) -> Vec<A>;

    /*
    Checks if two tuples are in the same orbit
    */

    fn in_same_orbit(first : &Vec<Self>, second : &Vec<Self>) -> bool
    where
        Self: Sized;
    
    /*
    Given a list of representatives of orbits, output a list of representatives of the product of the orbits
    */

    fn prod_orbit_rep(orbits : &Vec<Self>) -> 
    Vec<Vec<Self>>
    where
        Self : Sized {

            let support_list : Vec<Vec<A>> = orbits.iter().map(|x| x.support()).collect();

            let support_len_list : Vec<usize> = orbits.iter().map(|x| x.support().len()).collect();

            let all_rep : Vec<Vec<A>> = A::orbit_reps(support_len_list.clone().iter().sum());

            let all_rep_split : Vec<Vec<Vec<A>>> = all_rep.into_iter().map(|inner_vec| {
                let mut sub_vectors = Vec::new();
                let mut current_slice = &inner_vec[..];
                
                for len in support_len_list.clone() {
                    if current_slice.len() >= len {
                        let (first, second) = current_slice.split_at(len);
                        sub_vectors.push(first.to_vec());
                        current_slice = second; // Move the pointer forward
                        }
                    }
                    if !current_slice.is_empty() {
                        panic!("This part of the code should not be reached");
                    }
                    sub_vectors
                }).collect();

                let required_reps_split : Vec<Vec<Vec<A>>> = all_rep_split.into_iter().filter(|x| are_pointwise_equivalent(x, &support_list, A::in_same_orbit)).collect();

                let product_rep_list : Vec<Vec<Self>> =
                    required_reps_split.into_iter().map(|sup_list| {
                        sup_list.into_iter().zip(orbits.iter())
                        .map(|(sup,orb_rep)| orb_rep.apply_paut(PartialAut::new(&orb_rep.support(), &sup)))
                        .collect()
                    }).collect();
   

                return product_rep_list;
            }

    /*
    Given a representative of an orbits and a support, output the list of all elements in the orbit supported by the given supprt.
    Technically we should have a method to compute the algebraic closure of a support. But we omit that for simplicity.
    */

    fn project_to_support(rep : &Self, support : &Vec<A>) -> Vec<Self>
    where
        Self : Sized;

}