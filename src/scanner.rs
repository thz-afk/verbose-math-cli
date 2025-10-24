use crate::token::{Tok, TType};
use crate::error::{CalcError, CalcResult};

pub struct Scanner {
    src: String,
    toks: Vec<Tok>,
    cur: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Scanner {
        Scanner {
            src: src.to_string(),
            toks: Vec::new(),
            cur: 0,
        }
    }

    pub fn scan(&mut self) -> CalcResult<Vec<Tok>> {
        while !self.end() {
            self.scan_tok()?;
        }
        Ok(self.toks.clone())
    }

    fn scan_tok(&mut self) -> CalcResult<()> {
        let ch = self.adv();

        match ch {
            'p' => {
                if self.word("plus") {
                    self.toks.push(Tok::new(TType::ADD, 0.0));
                } else if self.word("pow") {
                    self.toks.push(Tok::new(TType::POW, 0.0));
                } else {
                    return Err(CalcError::UnexpectedChar(format!("p{}", self.rest())));
                }
            }
            'm' => {
                if self.word("mult") || self.word("multiply") {
                    self.toks.push(Tok::new(TType::MUL, 0.0));
                } else if self.word("minus") {
                    self.toks.push(Tok::new(TType::SUB, 0.0));
                } else if self.word("mod") {
                    self.toks.push(Tok::new(TType::MOD, 0.0));
                } else {
                    return Err(CalcError::UnexpectedChar(format!("m{}", self.rest())));
                }
            }
            'd' => {
                if self.word("div") || self.word("divide") {
                    self.toks.push(Tok::new(TType::DIV, 0.0));
                } else {
                    return Err(CalcError::UnexpectedChar(format!("d{}", self.rest())));
                }
            }
            's' => {
                if self.word("sqrt") {
                    self.toks.push(Tok::new(TType::SQRT, 0.0));
                } else {
                    return Err(CalcError::UnexpectedChar(format!("s{}", self.rest())));
                }
            }
            'a' => {
                if self.word("abs") {
                    self.toks.push(Tok::new(TType::ABS, 0.0));
                } else {
                    return Err(CalcError::UnexpectedChar(format!("a{}", self.rest())));
                }
            }
            '+' => self.toks.push(Tok::new(TType::ADD, 0.0)),
            '-' => {
                if self.peek().map_or(false, |c| c.is_numeric()) {
                    self.num(true)?;
                } else {
                    self.toks.push(Tok::new(TType::SUB, 0.0));
                }
            }
            '*' => self.toks.push(Tok::new(TType::MUL, 0.0)),
            '/' => self.toks.push(Tok::new(TType::DIV, 0.0)),
            '^' => self.toks.push(Tok::new(TType::POW, 0.0)),
            '%' => self.toks.push(Tok::new(TType::MOD, 0.0)),
            '(' => self.toks.push(Tok::new(TType::LPAR, 0.0)),
            ')' => self.toks.push(Tok::new(TType::RPAR, 0.0)),
            ' ' | '\t' | '\n' | '\r' => {}
            _ => {
                if ch.is_numeric() || ch == '.' {
                    self.cur -= 1;
                    self.num(false)?;
                } else {
                    return Err(CalcError::UnexpectedChar(ch.to_string()));
                }
            }
        }
        Ok(())
    }

    fn num(&mut self, neg: bool) -> CalcResult<()> {
        let mut s = if neg { "-".to_string() } else { String::new() };
        let mut dot = false;
        
        while !self.end() {
            let c = self.peek().unwrap();
            if c.is_numeric() {
                s.push(c);
                self.adv();
            } else if c == '.' && !dot {
                dot = true;
                s.push(c);
                self.adv();
            } else {
                break;
            }
        }
        
        let v = s.parse::<f64>()
            .map_err(|_| CalcError::UnexpectedChar(s.clone()))?;
        self.toks.push(Tok::new(TType::NUM, v));
        Ok(())
    }

    fn end(&self) -> bool {
        self.cur >= self.src.len()
    }

    fn adv(&mut self) -> char {
        let v = self.src.chars().nth(self.cur).unwrap_or('\0');
        self.cur += 1;
        v
    }

    fn peek(&self) -> Option<char> {
        if self.end() {
            None
        } else {
            self.src.chars().nth(self.cur)
        }
    }
    
    fn rest(&self) -> String {
        let e = (self.cur + 5).min(self.src.len());
        self.src[self.cur..e].to_string()
    }

    fn word(&mut self, w: &str) -> bool {
        let sb = self.src.as_bytes();
        let wb = w.as_bytes();
        let st = self.cur - 1;
        
        if st + w.len() > self.src.len() {
            return false;
        }
        
        for i in 0..w.len() {
            if sb[st + i] != wb[i] {
                return false;
            }
        }
        
        self.cur = st + w.len();
        true
    }
}
