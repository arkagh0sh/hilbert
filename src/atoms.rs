pub trait Hom: Eq<Self> {
}

pub trait HomWithOrd: Hom + Ord<Self> {
}