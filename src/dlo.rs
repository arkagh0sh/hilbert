use rug::Rational;

pub struct DLO {
    val : Rational
}

impl DLO {
    fn new(q : Rational) -> DLO {
        DLO {val : q}
    }
}