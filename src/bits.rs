use std::{ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign, Div}};
use std::fmt::Display;
use num_traits::{ConstOne, One, Zero};
use sorted_iter::sorted_pair_iterator::OuterJoin;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct F2(pub bool);

impl F2 {
    pub const ZERO: Self = F2(false);
    pub const ONE: Self = F2(true);

    /// Inversion is only defined for 1 (which is its own inverse).
    pub fn invert(self) -> Option<Self> {
        if self.0 { Some(self) } else { None }
    }
}

// Addition is XOR
impl Add for F2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        F2(self.0 ^ rhs.0)
    }
}

// Subtraction is ALSO XOR (since -1 = 1 mod 2)
impl Sub for F2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        F2(self.0 ^ rhs.0)
    }
}

// Multiplication is AND
impl Mul for F2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        F2(self.0 & rhs.0)
    }
}

// Negation is a no-op (1 = -1)
impl Neg for F2 {
    type Output = Self;
    fn neg(self) -> Self { self }
}

// Boilerplate for assignment operators
impl AddAssign for F2 { fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl MulAssign for F2 { fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; } }

impl Zero for F2 {
    fn zero() -> Self {
        F2::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == F2::ZERO
    }
}

impl One for F2 {
    fn one() -> Self {
        F2::ONE
    }
}

impl ConstOne for F2 {
    const ONE: Self = F2::ONE;
}

impl Display for F2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let answer = if *self == F2::ZERO { "0" } else {"1"};
        write!(f, "{}", answer)
    }
}

impl Div for F2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if !rhs.0 {
            panic!("Division by zero in F2!");
        }
        self
    }
}