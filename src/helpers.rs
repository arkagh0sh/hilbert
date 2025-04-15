use std::hash::Hash;
use std::cmp::Ord;
use std::ops::AddAssign;
use std::collections::{BTreeMap, BTreeSet};
use std::clone::Clone;
use num_traits::identities::Zero;

//Code from chatgpt.

pub fn sum_values_by_key<K, V>(pairs: Vec<(K, V)>) -> BTreeMap<K, V>
where
    K: Hash + Ord + Clone,
    V: AddAssign + Default + Eq + Zero,
{
    let mut map : BTreeMap<K, V> = BTreeMap::new();
    for (key, value) in pairs {
        map.entry(key).or_default().add_assign(value);
    }

    // Now filter out keys with zero values
    map.into_iter()
        .filter(|(_, v)| !((*v).is_zero()))
        .collect()
}

pub fn find_biggest_diff_key<K, V>(
    map1: &BTreeMap<K, V>,
    map2: &BTreeMap<K, V>,
) -> Option<K>
where
    K: Ord + Clone,
    V: PartialEq,
{
    // Create a union of keys from both maps
    let union: BTreeSet<K> = map1.keys().chain(map2.keys()).cloned().collect();

    // Iterate in reverse order (largest to smallest) checking for differences
    for key in union.iter().rev() {
        match (map1.get(key), map2.get(key)) {
            (Some(v1), Some(v2)) if v1 == v2 => continue, // values are equal, so continue
            _ => return Some(key.clone()),
        }
    }
    None
}

pub fn merge_sum<K, V>(map1: BTreeMap<K, V>, map2: BTreeMap<K, V>) -> BTreeMap<K, V>
where
    K: Ord,
    V: AddAssign + Default + Zero,
{
    let mut result : BTreeMap<K,V> = map1;
    for (key, value) in map2 {
        *result.entry(key).or_insert_with(Default::default) += value;
    }
    result.into_iter().filter(|(_, v)| !((*v).is_zero())).collect()
}

// pub fn sum_values_by_key_rat<K>(pairs: Vec<(K, Rational)>) -> BTreeMap<K, Rational>
// where
//     K: Hash + Ord + Clone,
//     //V: AddAssign + Default + Eq + Zero,
// {
//     let mut map : BTreeMap<K, Rational> = BTreeMap::new();
//     for (key, value) in pairs {
//         map.entry(key).or_default().add_assign(value);
//     }

//     // Now filter out keys with zero values
//     map.into_iter().filter(|(_, v)| !((*v).is_zero())).collect()
// }

pub fn union<X : Ord> (first : BTreeSet<X>, second : BTreeSet<X>) -> BTreeSet<X> {
    let mut answer : BTreeSet<X> = first;
    for x in second {
        if !(answer.contains(&x)) {
            answer.insert(x);
        }
    }
    answer
}