pub(crate) struct Grabber{}
use super::*;
impl Grabber{
    pub(crate) fn grab_tokens_before(&self, tokens : Vec<Token> , t:Token) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let num = tokens
        .iter()
        .position(|x| x == &t)
        .ok_or(ParseError::CantFindToken(t))?;
        Ok((tokens[..num].to_vec(),tokens[num..].to_vec()))
        
    }

    pub(crate) fn grab_line(&self, tokens : Vec<Token>) -> Result<(Vec<Token>,Vec<Token> ), ParseError>{
        let (line, rem ) = self.grab_tokens_before(tokens, Token::SemiColan)?;
        Ok((line , rem[1..].to_vec()))
    }

    pub(crate) fn grab_brac(&self, tokens : Vec<Token>) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let t = tokens.first().unwrap();
        let other_brac;
        match t {
            Token::LBrac => other_brac = Token::RBrac,
            Token::LSquare => other_brac = Token::RSquare,
            Token::LCur => other_brac = Token::RCur,
            _ => return Err(ParseError::ExpectButGot("Bracket".into(), t.clone()))
        }
        let mut open = 0 ; 
        let num = tokens
        .iter()
        .position(
            |x| 
            if x == &other_brac && open == 1 {true}
            else if x == t {open += 1;false} 
            else if x == &other_brac {open -= 1;false}
            else {false}
        ).ok_or(ParseError::NoClosingBracket)?;
        Ok((tokens[1..num].to_vec(), tokens[num+ 1 ..].to_vec()))
    }

    pub(crate) fn grab_fn(&self, tokens : Vec<Token>) -> Result<((Vec<Token> , Vec<Token>) , Vec<Token>),ParseError>{
        let (x, rem) = self.grab_tokens_before(tokens, Token::LCur)?;
        let (y, rem) = self.grab_brac(rem)?;
        Ok(((x,y),rem))
    }

    pub(crate) fn grab_prec2(&self, tokens : Vec<Token>) -> Result<(Vec<Token>,Vec<Token>),ParseError> {
        let pos = tokens
        .iter()
        .position(
            |x| 
            match x {
                Token::Plus | Token::Minus => true,
                _ => false,
            } 
        ).unwrap_or(tokens.len());
        Ok((tokens[..pos].to_vec() , tokens[pos..].to_vec()))
    }

    pub(crate) fn sep_on_comma(&self, tokens : Vec<Token> ) -> Result<Vec<Vec<Token>>,ParseError> {
        let mut vecs : Vec<Vec<Token>> = Vec::new() ;
        let mut vec  : Vec<Token> = Vec::new();
        for token in tokens {
            if token == Token::Comma {
                vecs.push(vec.clone());
                vec.clear();
            } else {
                vec.push(token);
            }
        }
        vecs.push(vec);
        Ok(vecs)
    }

    pub(crate) fn sep_on_bool_op1(&self, tokens : Vec<Token>) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let pos = tokens
        .iter()
        .position(|x| 
            match x {
                Token::LT | Token::GT | Token::LTEQ | Token::GTEQ | Token::EQ | Token::NEQ
                => true,
                _ => false 
            }
        ).ok_or(ParseError::ExpectButGot("Bool operator".into(), Token::Value("()".into())))?;
        Ok((tokens[..pos].to_vec(), tokens[pos..].to_vec()))
    }
        
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grab() -> Result<(),ParseError> {
        let y = Grabber {};
        let z = y.grab_tokens_before(vec![
            Token::And,
            Token::And,
            Token::And,
            Token::And,
            Token::And,
            Token::LCur,
            Token::Or,
            Token::Or,
            Token::Or,
            Token::Or,
            Token::Or
        ], Token::LCur)?;
        assert_eq!(z, (vec![
            Token::And,
            Token::And,
            Token::And,
            Token::And,
            Token::And,
        ],vec![
            Token::LCur,
            Token::Or,
            Token::Or,
            Token::Or,
            Token::Or,
            Token::Or
        ]));
        Ok(())
    }
    #[test]
    fn test_grab_prec2() -> Result<(),ParseError>{
        let grabber = Grabber{};
        let tokens = vec![
            Token::Value("9".into()),
            Token::Mul,
            Token::Value("2".into()),
            Token::Div,
            Token::Value("3".into()),
            Token::Plus,
            Token::Value("9".into())
        ];
        let (one,two) = grabber.grab_prec2(tokens)?;
        assert_eq!(one,
            vec![
                Token::Value("9".into()),
                Token::Mul,
                Token::Value("2".into()),
                Token::Div,
                Token::Value("3".into())
            ]
        );
        assert_eq!(two,
            vec![
                Token::Plus,
                Token::Value("9".into())
            ]
        );
        Ok(())
    }
    #[test]
    fn test_grab_brac() -> Result<(),ParseError>{
        let tokens = vec![
            Token::LBrac,
            Token::And,
            Token::And,
            Token::And,
            Token::RBrac,
            Token::Or
        ];
        let grabber = Grabber{};
        let (bef,after) = grabber.grab_brac(tokens)?;
        assert_eq!(bef,vec![
            Token::And,Token::And,Token::And
        ]);
        assert_eq!(after,vec![
            Token::Or
        ]);
        Ok(())
    }
}