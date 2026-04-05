use core::panic;
use std::hash::Hash;
use std::clone::Clone;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use super::atoms::*;
use super::helpers::*;
use itertools::Itertools;


#[derive(Debug, Hash, Clone)]
pub struct PartialAut<A> where A : AtomsWithOrd + Clone {

// should be BTreeSets
// actually have domain, range and maps.
// redundancy helps! 

    pub domain : BTreeSet<A>,
    pub range  : BTreeSet<A>
}

impl<A : Hash + AtomsWithOrd + Clone> PartialAut<A> {
    // add a check that the map is a bijection
    pub fn new(domain : &BTreeSet<A>, range : &BTreeSet<A>) -> Self {
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
pub trait ElementsWithAtoms<A : AtomsWithOrd + Clone + Hash> : Eq {

    // Applies a partial automorphism to an element
    fn apply_paut(&self, paut : PartialAut<A>) -> Self
    where
        Self : Sized;

    //Gives the support of an element.
    fn support(&self) -> BTreeSet<A>;

    // Checks if two tuples are in the same orbit
    fn in_same_orbit(first : &Vec<Self>, second : &Vec<Self>) -> bool
    where
        Self: Sized {
            let first_sup = first.into_iter()
                .map(|elem| elem.support())
                .into_iter()
                .flatten()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();

            let second_sup = second.into_iter()
                .map(|elem| elem.support())
                .flatten()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();

            if A::in_same_orbit(&first_sup,&second_sup) {
                let paut = PartialAut::new(&first_sup,&second_sup);
                let shifted_first : Vec<Self> = first
                    .into_iter()
                    .map(|elem| elem.apply_paut(paut.clone())).collect();
                     // use paut without cloning
                shifted_first == *second
            } else {
                false
            }
        }
    
    //Given a list of orbits, output the list of orbits in the product

    // this should be implementable just by using support and project to orbit and using AtomsWithOrd
    fn prod_orbit(orbits : &Vec<Orbit<Self>>) -> 
    Vec<Orbit<A, Vec<Self>>>
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
        Self : Sized {
            let rep_supp = rep.support();
            let sup_len = rep_supp.len();
            let support_set : BTreeSet<A> = support.clone().into_iter().collect();
            let sup_list : Vec<Vec<A>> = support_set
                .into_iter()
                .combinations(sup_len)
                .collect();
            let good_sup_list : Vec<Vec<A>> = sup_list.into_iter().filter(|elem| A::in_same_orbit(&rep_supp, elem)).collect();

            good_sup_list.into_iter().map(|supp| rep.apply_paut(PartialAut::new(&rep_supp, &supp))).collect()
        }
}