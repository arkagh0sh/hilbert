use std::hash::Hash;
use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, Ordering::Equal, max};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Mul, Sub, Neg, Div};
use num_traits::{Zero, Pow, ConstOne};
use num_traits::identities::One;

use super::helpers::*;
#[derive(Clone)]
#[derive(Hash)]
pub struct Monomial<X> where X : Ord + Clone {
    expo : BTreeMap<X,u8>
}

impl<X: Hash + Ord + Clone> Monomial<X> {

    fn new(expo_list : Vec<(X,u8)>) -> Monomial<X> {
        let x: Vec<(X, u8)> = expo_list.into_iter().filter(|&(_, v)| v != 0).collect();
        let new_expo: BTreeMap<X, u8> = sum_values_by_key(x);
        Monomial {expo : new_expo}
    } 

    // fn var(x : X)-> Monomial<X> {
    //     Monomial::new(vec![(x,1)])
    // }

    fn domain(&self) -> BTreeSet<X> {
        self.expo.keys().cloned().collect()
    }

    fn exp_of(&self, x : &X) -> u8 {
        let a: Option<&u8> = self.expo.get(&x);
        match a {
            Some(n) => *n,
            None => 0
        }
    }

    pub fn is_divisible_by(&self, other : &Monomial<X>) -> bool {
        let mut answer = true;
        for (x,n) in other.expo.iter() {
            if *n > self.exp_of(x) {
                answer = false;
            }
        }
        answer
    }

    pub fn lcm(&self, other : &Monomial<X>) -> Monomial<X> {
        let mut new_expo : BTreeMap<X,u8> = BTreeMap::new();
        let varset = union(self.domain(), other.domain());
        for x in varset {
            new_expo.insert(x.clone(), max(self.exp_of(&x), other.exp_of(&x)));
        }
        Monomial { expo: new_expo }
    }
}

impl<X : Hash + Ord + Clone> PartialEq for Monomial<X> {
    fn eq(&self, other: &Self) -> bool {
        self.expo == other.expo 
    }
}

impl<X : Hash + Ord + Clone> Eq for Monomial<X> {}

impl<X : Hash + Ord + Clone> PartialOrd for  Monomial<X>{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

// We use the co-lex order (and NOT the revlex order)

impl<X : Hash + Ord + Clone> Ord for Monomial<X> {
    fn cmp(&self, other: &Self) -> Ordering {
        let dif = find_biggest_diff_key(&self.expo, &other.expo);
       match dif {
            None => Equal,
            Some(x) => if self.exp_of(&x) < other.exp_of(&x) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl<X : Clone + Ord + Hash> Mul for Monomial<X> {

    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let new_expo: BTreeMap<X, u8> = merge_sum(self.expo, rhs.expo);
        Monomial {expo : new_expo}
    }
}

impl<X : Clone + Ord + Hash> Div for Monomial<X> {

    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Option<Self> {
        if !(self.is_divisible_by(&rhs)) {
            None
        } else {
            let mut result = BTreeMap::new();
            for (x, v1) in self.expo.iter() {
                let new_val = v1 - rhs.exp_of(x);
                result.insert(x.clone(),new_val);
            }
            Some(Monomial {expo : result})
        }
    }
}

impl<X : Clone + Ord + Hash> One for Monomial<X> {
    fn one() -> Self {
        Monomial::new(vec![])
    }
}

impl<X : Clone + Ord + Hash> Pow<u8> for Monomial<X> {
    type Output = Self;
    fn pow(self, rhs: u8) -> Self::Output {
        let new_expo: BTreeMap<X, u8> = self.expo.iter().map(|(k, v)| (k.clone(), v.clone() * rhs.clone())).collect();
        Monomial {expo : new_expo}
    }
}

impl<X : Ord + Clone + Display> Display for Monomial<X> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut to_view : String = String::new();
        for (x,n) in self.expo.iter() {
            to_view = to_view + &x.to_string() + "^" + &n.to_string() + " ";
        }
        if !(self.expo.is_empty()) {
            to_view = String::from("( ") + &to_view + ")";
        } else {
            to_view = to_view + "1";
        }
        write!(f,"{}",to_view)
    }
}

#[derive(Clone)]
#[derive(Hash)]
pub struct Polynomial<F,X>
where
  X : Hash + Ord + Clone,
{
    coeff : BTreeMap<Monomial<X>,F>
}

impl<F : AddAssign + Default + Eq + Zero + ConstOne + Clone, X : Hash + Ord + Clone> Polynomial<F,X> {
    fn new(coeff_list : Vec<(Monomial<X>, F)>) -> Polynomial<F,X> {
        let new_coeff: BTreeMap<Monomial<X>, F> = sum_values_by_key(coeff_list);
        Polynomial {coeff : new_coeff}
    }

    pub fn var(x : X)-> Polynomial<F,X> {
        Polynomial::new(vec![(Monomial::new(vec![(x,1)]),F::ONE.clone())])
    }

    pub fn constant(c : F) -> Polynomial<F,X> {
        Polynomial::new(vec![(Monomial::one(),c)])
    }

    pub fn square(&self) -> Polynomial<F,X> {
        self.clone() * self.clone()
    }

    fn from_mon(p : Monomial<X>) -> Polynomial<F,X> {
        Polynomial { coeff: BTreeMap::from([(p,F::one())]) }
    }

    pub fn domain(&self) -> BTreeSet<X> {
        let mut answer : BTreeSet<X> = BTreeSet::new();
        for (p,_r) in self.coeff.iter() {
            for x in p.domain() {
                if !(answer.contains(&x)) {
                    answer.insert(x);
                }
            }
        }
        answer
    }

    pub fn leading_mon(&self) -> Monomial<X> {
        let pair   = self.coeff.last_key_value();
        match pair {
            None => Monomial::one(),
            Some((k,_v)) => k.clone()
        }
    }

    pub fn leading_coeff(&self) -> F {
        let pair   = self.coeff.last_key_value();
        match pair {
            None => F::zero(),
            Some((_k,v)) => v.clone()
        }
    }

    pub fn leading_term(&self) -> Polynomial<F,X> {
        let pair   = self.coeff.last_key_value();
        match pair {
            None => Polynomial::zero(),
            Some((k,v)) => Polynomial { coeff: BTreeMap::from([(k.clone(),v.clone())]) }
        }
    }

    pub fn is_reducible_by(&self, other : &Polynomial<F,X>) -> bool {
        if self.is_zero() {
            false
        } else if other.domain().is_subset(&self.domain()) {
            if self.leading_mon().is_divisible_by(&other.leading_mon()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn reduce_by(self, other : &Polynomial<F,X>) -> Polynomial<F,X>
    where
      F : Neg<Output = F>
    {
        if  self.is_reducible_by(other) {
            let quotient = self.leading_mon() / other.leading_mon();

            match quotient {
                None => self,
                Some(p) => self - Polynomial::from_mon(p) * other.clone()
            }
        } else {
            self
        }
    }

    pub fn is_reduced(&self, base : &[Polynomial<F,X>]) -> bool
    where
      F : Display,
      X : Display
    {
        let mut answer = true;
        for g in base {
            if self.is_reducible_by(&g) {
                answer = false;
            }
        }
        answer
    }

    pub fn syzygy(self, other : &Polynomial<F,X>) -> Polynomial<F,X>
    where
      F : Div<Output = F> + Neg<Output = F> + Sub
    {
        let l = self.leading_mon().lcm(&other.leading_mon());
        let a = l.clone() / self.leading_mon();
        let b = l / other.leading_mon();
        match a {
            Some(a1) => {
                let a2 = Polynomial::new(vec![(a1,F::one()/self.leading_coeff())]);
                let f = a2 * self;
                match b {
                    Some(b1) => {
                        let b2 = Polynomial::new(vec![(b1,F::one()/other.leading_coeff())]);
                        let g = b2 * other.clone();
                        f - g
                    }
                    None => panic!("Error in syzygy. Probably problem with lcm")
                }
            }

            None => panic!("Error in syzygy. Probably problem with lcm")
        }
    }

    pub fn remainder(self, base : &[Polynomial<F,X>]) -> Polynomial<F,X>
    where
      F : Neg<Output = F> + Display,
      X : Display
    {
        let mut post = self;
        let mut pre = Polynomial::zero();
        while pre != post {
            pre = post.clone();
            for g in base {
                post = post.reduce_by(g);
                println!("{}",post);
            }
        }
        post
    }
}

impl<F : PartialEq, X : Hash + Ord + Clone> PartialEq for Polynomial<F,X> {
    fn eq(&self, other: &Self) -> bool {
        self.coeff == other.coeff
    }
}

impl<F : PartialEq, X : Hash + Ord + Clone> Eq for Polynomial<F,X> {}

impl<F : AddAssign + Default + Zero, X : Clone + Ord + Hash> Add for Polynomial<F,X> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let new_coeff: BTreeMap<Monomial<X>, F> = merge_sum(self.coeff, rhs.coeff);
        Polynomial { coeff : new_coeff}
    }
}

impl<F : Default + AddAssign + Zero, X : Clone + Ord + Hash> Zero for Polynomial<F,X> {

    fn zero() -> Self {
        Polynomial { coeff : BTreeMap::new()}
    }

    fn is_zero(&self) -> bool {
        self.coeff.len() == 0
    }
}

impl<F : AddAssign + Default + Mul<Output = F> + Clone + Zero, X : Clone + Ord + Hash> Mul for Polynomial<F,X>
{

    type Output = Self;

    fn mul(self : Self, other : Self) -> Self {
        let mut result: Polynomial<F,X> = Polynomial::zero();
        for (p1,r1) in self.coeff.iter() {
            for (p2, r2) in other.coeff.iter() {
                result.coeff.entry(p1.clone()*p2.clone()).or_default().add_assign(r1.clone() * r2.clone());
            }
        }
        result
    }
}

impl<F : One + AddAssign + Default + Clone + ConstOne + Zero, X : Hash + Clone + Ord> One for Polynomial<F,X>{

    fn one() -> Self {
        let mut new_coeff : BTreeMap<Monomial<X>,F> = BTreeMap::new();
        new_coeff.insert(Monomial::one(), F::one());
        Polynomial { coeff: new_coeff }
    }
}

impl<F : Clone + AddAssign + Default + One + Eq + Zero + ConstOne,X : Clone + Ord + Hash> Pow<u8> for Polynomial<F,X> {
    type Output = Self;

    fn pow(self, rhs: u8) -> Self::Output {
        if rhs == 0 {
            Polynomial::one()
        } else if rhs == 1 {
            self
        } else {
            let maybe_answer = self.clone().pow(rhs/2);
            if rhs % 2 == 0 {
                maybe_answer.square()
            } else {
                maybe_answer.square() * self
            }          
        }
    }
}

impl<F : Display, X : Hash + Ord + Clone + Display> Display for Polynomial<F,X> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut to_view : String = String::new();
        for (p,c) in self.coeff.iter() {
            if *p == Monomial::one() {
                to_view = to_view + "+ " + &c.to_string() + " ";
            } else {
                to_view = to_view + "+ " + &c.to_string() + "." + &p.to_string() + " ";
            }
        }
        
        if self.coeff.is_empty() {
            to_view = String::from("0");
        }

        write!(f,"{}",to_view)
    }
}

impl<F : Neg<Output = F> + Clone, X : Hash + Ord + Clone> Neg for Polynomial<F,X> {

    type Output = Self;

    fn neg(self) -> Self {
    let new_coeff = self.coeff.into_iter()
        .map(|(k, v)| (k, v.neg()))
        .collect();
    Polynomial { coeff: new_coeff }
    }
}

impl<F : Neg<Output = F> + Clone + AddAssign + Default + Zero, X : Hash + Ord + Clone> Sub for Polynomial<F,X> {
    type Output = Self;

    fn sub(self,other: Self) -> Self {
        self + other.neg()
    }
}