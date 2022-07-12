#[derive(Clone,Debug,PartialEq)]
pub(crate) enum Token {
    // Line Types
    Print, For, If, Else, Return, 
    // Primitives
    Int, String, 
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

pub(crate) struct Tokenizer<'a> {
    code: &'a str, 
    pos: usize,
    pub(crate) tokens : Vec<Token>
}

impl<'a> Tokenizer<'a> {

    pub(crate) fn new(message : &'a str) -> Tokenizer<'a> {
        Tokenizer { 
            code : message,
            pos: 0, 
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

    fn match_keyword_and_check(&mut self, c : char , t : Token, t2 : Token ) {
        self.match_keyword();
        if self.cur() == c {
            self.tokens.push(t);
            self.code = &self.code[1..];
        } else {
            self.tokens.push(t2);
        }
    }

    fn step(&mut self) {
        match self.code.chars().nth(self.pos) {
            Some(x) => {
                match x {
                    ';' => self.match_keyword_and(Token::SemiColan),
                    ',' => self.match_keyword_and(Token::Comma),
                    '[' => self.match_keyword_and(Token::LSquare),
                    ']' => self.match_keyword_and(Token::RSquare),
                    '+' => self.match_keyword_and_check('+', Token::Inc, Token::Plus),
                    '-' => self.match_keyword_and_check('-', Token::Dec, Token::Minus),
                    '*' => self.match_keyword_and(Token::Mul),
                    '/' => self.match_keyword_and(Token::Div),
                    '&' => self.match_keyword_and_check('&', Token::BAnd, Token::And),
                    '|' => self.match_keyword_and_check('|', Token::BOr, Token::Or),
                    '<' => self.match_keyword_and_check('=', Token::LTEQ, Token::LT),
                    '>' => self.match_keyword_and_check('=', Token::GTEQ, Token::GT),
                    '=' => self.match_keyword_and_check('=', Token::EQ, Token::Equal),
                    '!' => self.match_keyword_and_check('=', Token::NEQ, Token::Not),
                    '(' => self.match_keyword_and(Token::LBrac),
                    ')' => self.match_keyword_and(Token::RBrac),
                    '{' => self.match_keyword_and(Token::LCur),
                    '}' => self.match_keyword_and(Token::RCur),
                    ' ' => self.match_keyword(),
                    _   => self.pos += 1
                }
            }
            None => {}
        }
    }

    fn match_word(&mut self, word :&str ){
        let wo = word.trim();
        match wo {
            "Print"  => self.tokens.push(Token::Print),
            "int"    => self.tokens.push(Token::Int),
            "String" => self.tokens.push(Token::String),
            "if"     => self.tokens.push(Token::If),
            "for"    => self.tokens.push(Token::For),
            "else"   => self.tokens.push(Token::Else),
            "return" => self.tokens.push(Token::Return),
            "def"    => self.tokens.push(Token::Def),
            ""       => {}
            _        => self.tokens.push(Token::Value(word.trim().into()))
        }
    }

    fn match_keyword(&mut self){
        let word = &self.code[0..self.pos];
        self.code = &self.code[self.pos + 1..];
        self.match_word(word);
        self.pos = 0;
    }

    fn match_keyword_and(&mut self, t : Token ) {
        self.match_keyword();
        self.tokens.push(t);
    }

    }

#[cfg(test)] 
mod tests {
    use crate::parser::tokenizer::Token;

    use super::Tokenizer;

    #[test]
    fn token_test_1() {
        let str = "Print x + 10 - 3;";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        assert_eq!(tk.tokens, 
        vec![
            Token::Print,
            Token::Value("x".into()),
            Token::Plus,
            Token::Value("10".into()),
            Token::Minus,
            Token::Value("3".into()),
            Token::SemiColan
        ]
    );
    }
    #[test]
    fn token_test_2() {
        let str = "Print x + 10 - 3";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        assert_eq!(tk.tokens, 
        vec![
            Token::Print,
            Token::Value("x".into()),
            Token::Plus,
            Token::Value("10".into()),
            Token::Minus,
            Token::Value("3".into())
        ]
    );
    }
    #[test]
    fn token_test_3(){
        let str = "Print for int String if else return ";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        assert_eq!(tk.tokens , 
        vec![
            Token::Print,
            Token::For,
            Token::Int,
            Token::String,
            Token::If,
            Token::Else,
            Token::Return
        ])
    }

    #[test]
    fn token_test_4(){
        let str = "+ - * / < > = ++ -- != == >= <= ";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        assert_eq!(tk.tokens , 
        vec![
            Token::Plus,
            Token::Minus,
            Token::Mul,
            Token::Div,
            Token::LT,
            Token::GT,
            Token::Equal,
            Token::Inc,
            Token::Dec,
            Token::NEQ,
            Token::EQ,
            Token::GTEQ,
            Token::LTEQ
        ])
    }

    #[test]
    fn token_test_5(){
        let str = "( ) { } [ ]";
        let mut tk = Tokenizer::new(str);
        tk.tokenize();
        assert_eq!(tk.tokens , 
        vec![
            Token::LBrac,
            Token::RBrac,
            Token::LCur,
            Token::RCur,
            Token::LSquare,
            Token::RSquare
        ])
    }

}