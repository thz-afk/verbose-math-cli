#[derive(Debug, Clone, PartialEq)]
pub enum TType {
    ADD, SUB, MUL, DIV, POW, MOD,
    LPAR, RPAR,
    NUM,
    SQRT, ABS,
}

#[derive(Debug, Clone)]
pub struct Tok {
    pub typ: TType,
    pub val: f64,
}

impl Tok {
    pub fn new(typ: TType, val: f64) -> Tok {
        Tok { typ, val }
    }
}

impl TType {
    pub fn prec(&self) -> i32 {
        match self {
            Self::ADD | Self::SUB => 1,
            Self::MUL | Self::DIV | Self::MOD => 2,
            Self::POW => 3,
            Self::SQRT | Self::ABS => 4,
            _ => 0,
        }
    }
}
