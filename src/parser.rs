use crate::token::{Tok, TType};
use crate::error::{CalcError, CalcResult};

pub struct Parser {
    toks: Vec<Tok>,
    cur: usize,
}

impl Parser {
    pub fn new(toks: Vec<Tok>) -> Parser {
        Parser { toks, cur: 0 }
    }

    pub fn parse(&mut self) -> CalcResult<f64> {
        if self.toks.is_empty() {
            return Err(CalcError::EmptyExpr);
        }
        self.expr()
    }

    fn expr(&mut self) -> CalcResult<f64> {
        self.add()
    }

    fn add(&mut self) -> CalcResult<f64> {
        let mut l = self.mul()?;

        while self.m(&[TType::ADD, TType::SUB]) {
            let op = self.prev().typ.clone();
            let r = self.mul()?;
            
            match op {
                TType::ADD => l += r,
                TType::SUB => l -= r,
                _ => {}
            }
        }
        Ok(l)
    }

    fn mul(&mut self) -> CalcResult<f64> {
        let mut l = self.exp()?;

        while self.m(&[TType::MUL, TType::DIV, TType::MOD]) {
            let op = self.prev().typ.clone();
            let r = self.exp()?;
            
            match op {
                TType::MUL => l *= r,
                TType::DIV => {
                    if r == 0.0 {
                        return Err(CalcError::DivByZero);
                    }
                    l /= r;
                }
                TType::MOD => l %= r,
                _ => {}
            }
        }
        Ok(l)
    }

    fn exp(&mut self) -> CalcResult<f64> {
        let mut l = self.un()?;

        while self.m(&[TType::POW]) {
            let r = self.un()?;
            l = l.powf(r);
        }
        Ok(l)
    }

    fn un(&mut self) -> CalcResult<f64> {
        if self.m(&[TType::SUB]) {
            let r = self.un()?;
            return Ok(-r);
        }
        
        if self.m(&[TType::SQRT]) {
            let r = self.un()?;
            return Ok(r.sqrt());
        }
        
        if self.m(&[TType::ABS]) {
            let r = self.un()?;
            return Ok(r.abs());
        }

        self.prim()
    }

    fn prim(&mut self) -> CalcResult<f64> {
        if self.m(&[TType::NUM]) {
            return Ok(self.prev().val);
        }

        if self.m(&[TType::LPAR]) {
            let e = self.expr()?;
            if !self.m(&[TType::RPAR]) {
                return Err(CalcError::MismatchedParen);
            }
            return Ok(e);
        }

        Err(CalcError::InvalidExpr)
    }

    fn m(&mut self, typs: &[TType]) -> bool {
        for t in typs {
            if self.chk(t) {
                self.adv();
                return true;
            }
        }
        false
    }

    fn chk(&self, t: &TType) -> bool {
        if self.end() {
            return false;
        }
        &self.peek().typ == t
    }

    fn adv(&mut self) -> &Tok {
        if !self.end() {
            self.cur += 1;
        }
        self.prev()
    }

    fn end(&self) -> bool {
        self.cur >= self.toks.len()
    }

    fn peek(&self) -> &Tok {
        &self.toks[self.cur]
    }

    fn prev(&self) -> &Tok {
        &self.toks[self.cur - 1]
    }
}
