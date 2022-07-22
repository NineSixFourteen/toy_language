#[derive(Clone,Debug,PartialEq)]
pub(crate) enum TokenTy {
    // Line Types
    Print, For, If, Else, Return, 
    // Primitives
    Int, String, Boolean, Char, Float, Array, 
    // Seperators
    SemiColan, Comma, Equal, 
    // Seperators - Brackets 
    LBrac, RBrac, LSquare, RSquare, LCur, RCur,
    // Sererators - Single Operartor
    Inc, Dec,
    // Seperators - Binary Operartor 
    Plus, Minus, Mul, Div, And, Or, 
    // Seperators - Single Boolean Operartor
    Not, 
    // Seperators - Binary Boolean Operators
    BAnd, BOr, LT, GT, LTEQ, GTEQ, EQ, NEQ,  
    // Others 
    Value(String) , Def
}

#[derive(Debug,PartialEq,Clone)]
pub(crate) struct Token {
    pub ty : TokenTy,
    pub line_num : usize,
}

impl Token {
    pub(crate) fn new(ty : TokenTy, line_num : usize) -> Token {
        Token {
            ty,
            line_num 
        }
    }
}

pub(crate) struct Tokenizer<'a> {
    code: &'a str, 
    pos: usize,
    line_num : usize,
    pub(crate) tokens : Vec<Token>
}

impl<'a> Tokenizer<'a> {

    pub(crate) fn new(message : &'a str) -> Tokenizer<'a> {
        Tokenizer { 
            code : message,
            pos: 0, 
            line_num : 0,
            tokens: Vec::new()
         }
    }

    pub(crate) fn tokenize(&mut self){
        while !(self.code.is_empty() || self.pos >= self.code.len() ){
            self.step();
        }
        if !self.code.is_empty() {
            self.match_word(self.code.into());
        }
    }

    fn cur(&mut self) -> char {
        match self.code.chars().nth(self.pos) {
            Some(x) => x,
            None => ' ',
        }
    }

    fn match_keyword_and_check(&mut self, c : char , t : TokenTy, t2 : TokenTy ) {
        self.match_keyword();
        if self.cur() == c {
            self.tokens.push(Token::new(t,self.pos));
            self.code = &self.code[1..];
        } else {
            self.tokens.push(Token::new(t2,self.pos));
        }
    }

    fn step(&mut self) {
        match self.code.chars().nth(self.pos) {
            Some(x) => {
                match x {
                    ';' => self.match_keyword_and(TokenTy::SemiColan),
                    ',' => self.match_keyword_and(TokenTy::Comma),
                    '[' => self.match_keyword_and_check(']',TokenTy::Array,TokenTy::LSquare),
                    ']' => self.match_keyword_and(TokenTy::RSquare),
                    '+' => self.match_keyword_and_check('+', TokenTy::Inc, TokenTy::Plus),
                    '-' => self.match_keyword_and_check('-', TokenTy::Dec, TokenTy::Minus),
                    '*' => self.match_keyword_and(TokenTy::Mul),
                    '/' => self.match_keyword_and(TokenTy::Div),
                    '&' => self.match_keyword_and_check('&', TokenTy::BAnd, TokenTy::And),
                    '|' => self.match_keyword_and_check('|', TokenTy::BOr, TokenTy::Or),
                    '<' => self.match_keyword_and_check('=', TokenTy::LTEQ, TokenTy::LT),
                    '>' => self.match_keyword_and_check('=', TokenTy::GTEQ, TokenTy::GT),
                    '=' => self.match_keyword_and_check('=', TokenTy::EQ, TokenTy::Equal),
                    '!' => self.match_keyword_and_check('=', TokenTy::NEQ, TokenTy::Not),
                    '(' => self.match_keyword_and(TokenTy::LBrac),
                    ')' => self.match_keyword_and(TokenTy::RBrac),
                    '{' => self.match_keyword_and(TokenTy::LCur),
                    '}' => self.match_keyword_and(TokenTy::RCur),
                    ' ' => self.match_keyword(),
                    '\n' => {self.line_num += 1;self.pos += 1},
                    _   => self.pos += 1
                }
            }
            None => {}
        }
    }

    fn match_word(&mut self, mut word :&str ){
        word = word.trim();
        if word.ends_with("[]"){
            self.push_token(TokenTy::Array);
            word = &word[..word.len()-2];
        }
        match word {
            "Print"   => self.push_token(TokenTy::Print),
            "int"     => self.push_token(TokenTy::Int),
            "float"   => self.push_token(TokenTy::Float),
            "boolean" => self.push_token(TokenTy::Boolean),
            "char"    => self.push_token(TokenTy::Char), 
            "String"  => self.push_token(TokenTy::String),
            "if"      => self.push_token(TokenTy::If),
            "for"     => self.push_token(TokenTy::For),
            "else"    => self.push_token(TokenTy::Else),
            "return"  => self.push_token(TokenTy::Return),
            "def"     => self.push_token(TokenTy::Def),
            ""        => {}
            _         => self.tokens.push(Token::new(TokenTy::Value(word.trim().into()),self.line_num))
        }
    }

    fn match_keyword(&mut self){
        let word = &self.code[0..self.pos];
        self.code = &self.code[self.pos + 1..];
        self.match_word(word);
        self.pos = 0;
    }

    fn match_keyword_and(&mut self, t : TokenTy ) {
        self.match_keyword();
        self.push_token(t);
    }

    fn push_token(&mut self, ty : TokenTy) {
        self.tokens.push(Token::new(ty,self.line_num))
    }

}


#[cfg(test)] 
mod tests {
    use crate::parser::TokenTy;

    use super::{Tokenizer, Token};

    #[test]
    fn token_test_1() {
        let str = "Print x + 10 - 3;";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        test_helper(tk.tokens, 
        vec![
            TokenTy::Print,
            TokenTy::Value("x".into()),
            TokenTy::Plus,
            TokenTy::Value("10".into()),
            TokenTy::Minus,
            TokenTy::Value("3".into()),
            TokenTy::SemiColan
        ]
    );
    }
    #[test]
    fn token_test_2() {
        let str = "Print x + 10 - 3";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        test_helper(tk.tokens, 
        vec![
            TokenTy::Print,
            TokenTy::Value("x".into()),
            TokenTy::Plus,
            TokenTy::Value("10".into()),
            TokenTy::Minus,
            TokenTy::Value("3".into())
        ]
    );
    }
    #[test]
    fn token_test_3(){
        let str = "Print for int String if else return int[] ";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        test_helper(tk.tokens , 
        vec![
            TokenTy::Print,
            TokenTy::For,
            TokenTy::Int,
            TokenTy::String,
            TokenTy::If,
            TokenTy::Else,
            TokenTy::Return,
            TokenTy::Int,
            TokenTy::Array
        ])
    }

    #[test]
    fn token_test_4(){
        let str = "+ - * / < > = ++ -- != == >= <= ";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        test_helper(tk.tokens , 
        vec![
            TokenTy::Plus,
            TokenTy::Minus,
            TokenTy::Mul,
            TokenTy::Div,
            TokenTy::LT,
            TokenTy::GT,
            TokenTy::Equal,
            TokenTy::Inc,
            TokenTy::Dec,
            TokenTy::NEQ,
            TokenTy::EQ,
            TokenTy::GTEQ,
            TokenTy::LTEQ
        ])
    }

    #[test]
    fn token_test_5(){
        let str = "( ) { } [ ]";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        test_helper(tk.tokens , 
        vec![
            TokenTy::LBrac,
            TokenTy::RBrac,
            TokenTy::LCur,
            TokenTy::RCur,
            TokenTy::LSquare,
            TokenTy::RSquare
        ])
    }
    fn test_helper( tokens : Vec<Token>, expected : Vec<TokenTy>) {
        let mut toks = Vec::new();
        for token in &tokens {
            toks.push(token.ty.clone());
        }
        assert_eq!(expected, toks)
    }
}

