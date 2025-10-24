use crate::token::{Tok, TType};
use crate::error::{CalcError, CalcResult};

pub struct Scanner {
    src: Vec<char>,
    toks: Vec<Tok>,
    cur: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            toks: Vec::new(),
            cur: 0,
        }
    }

    pub fn scan(&mut self) -> CalcResult<Vec<Tok>> {
        while !self.is_end() {
            self.scan_token()?;
        }
        Ok(self.toks.clone())
    }

    fn scan_token(&mut self) -> CalcResult<()> {
        let ch = self.advance();

        match ch {
            'p' => self.match_word(&[
                ("plus", TType::ADD),
                ("pow", TType::POW),
            ])?,

            'm' => self.match_word(&[
                ("mult", TType::MUL),
                ("multiply", TType::MUL),
                ("minus", TType::SUB),
                ("mod", TType::MOD),
            ])?,

            'd' => self.match_word(&[
                ("div", TType::DIV),
                ("divide", TType::DIV),
            ])?,

            's' => self.match_word(&[
                ("sqrt", TType::SQRT),
            ])?,

            'a' => self.match_word(&[
                ("abs", TType::ABS),
            ])?,

            '+' => self.push_tok(TType::ADD),
            '-' => {
                if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.scan_number(true)?;
                } else {
                    self.push_tok(TType::SUB);
                }
            }
            '*' => self.push_tok(TType::MUL),
            '/' => self.push_tok(TType::DIV),
            '^' => self.push_tok(TType::POW),
            '%' => self.push_tok(TType::MOD),
            '(' => self.push_tok(TType::LPAR),
            ')' => self.push_tok(TType::RPAR),

            c if c.is_whitespace() => {}

            c if c.is_ascii_digit() || c == '.' => {
                self.cur -= 1;
                self.scan_number(false)?;
            }

            c => return Err(CalcError::UnexpectedChar(c.to_string())),
        }

        Ok(())
    }

    fn scan_number(&mut self, negative: bool) -> CalcResult<()> {
        let mut buf = String::new();
        if negative { buf.push('-'); }

        let mut has_dot = false;

        while let Some(&c) = self.src.get(self.cur) {
            match c {
                d if d.is_ascii_digit() => {
                    buf.push(d);
                    self.cur += 1;
                }
                '.' if !has_dot => {
                    has_dot = true;
                    buf.push('.');
                    self.cur += 1;
                }
                _ => break,
            }
        }

        let value = buf.parse::<f64>()
            .map_err(|_| CalcError::UnexpectedChar(buf.clone()))?;

        self.toks.push(Tok::new(TType::NUM, value));
        Ok(())
    }

    fn match_word(&mut self, words: &[(&str, TType)]) -> CalcResult<()> {
        for (w, ttype) in words {
            if self.try_word(w) {
                self.push_tok(*ttype);
                return Ok(());
            }
        }

        let preview: String = self.src.iter().skip(self.cur).take(5).collect();
        Err(CalcError::UnexpectedChar(preview))
    }

    fn try_word(&mut self, w: &str) -> bool {
        let len = w.len();
        if self.cur - 1 + len > self.src.len() {
            return false;
        }

        let slice: String = self.src[self.cur - 1..self.cur - 1 + len].iter().collect();
        if slice == w {
            self.cur += len - 1;
            true
        } else {
            false
        }
    }

    fn is_end(&self) -> bool {
        self.cur >= self.src.len()
    }

    fn advance(&mut self) -> char {
        let c = *self.src.get(self.cur).unwrap_or(&'\0');
        self.cur += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.cur).copied()
    }

    fn push_tok(&mut self, ttype: TType) {
        self.toks.push(Tok::new(ttype, 0.0));
    }
}
