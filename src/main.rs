mod mon;
mod helpers;

use num_traits::{Zero, One, Pow};

use mon::*;

fn main() {
    let x: Polynomial<i8, String> = Polynomial::var(String::from("x"));
    let y: Polynomial<i8, String> = Polynomial::var(String::from("y"));
    let z: Polynomial<i8, String> = Polynomial::zero();
    let i: Polynomial<i8, String> = Polynomial::constant(1);
    let p: Polynomial<i8, String> = (x.clone() * x.clone()) + x.clone() + x.clone() + i.clone().pow(2) + z;
    let q: Polynomial<i8, String> = (x.clone() + Polynomial::one()).pow(2);
    println!("{}", p == q);
    println!("{}",p);
    println!("{}",p.leading_mon());
    println!("{}",p.leading_coeff());
    println!("{}",p.leading_term());
    println!("{}",p.leading_mon().is_divisible_by(&p.leading_mon()));
    println!("{}",p.clone().reduce_by(&i));
    println!("{}",y.clone().is_reduced(&[x.clone()]));
    println!("{}",(x.clone() + y.clone()).leading_mon());
    println!("{}",p.clone().syzygy(&(y + x.clone())));
    println!("{}",p.remainder(&[i]));   
}