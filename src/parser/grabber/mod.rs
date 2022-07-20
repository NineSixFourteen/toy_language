pub(crate) struct Grabber{}
use super::*;
impl Grabber{
    pub(crate) fn grab_tokens_before( tokens : Vec<Token> , t:TokenTy) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let num = tokens
        .iter()
        .position(|x| x.ty == t)
        .ok_or(ParseError::CantFindToken(Token::new(t,tokens.first().unwrap().line_num)))?;
        Ok((tokens[..num].to_vec(),tokens[num..].to_vec()))
        
    }

    pub(crate) fn grab_line( tokens : Vec<Token>) -> Result<(Vec<Token>,Vec<Token> ), ParseError>{
        let (line, rem ) = Grabber::grab_tokens_before(tokens, TokenTy::SemiColan)?;
        Ok((line , rem[1..].to_vec()))
    }

    pub(crate) fn grab_brac( tokens : Vec<Token>) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let t = tokens.first().unwrap();
        let other_brac;
        match t.ty {
            TokenTy::LBrac => other_brac = TokenTy::RBrac,
            TokenTy::LSquare => other_brac = TokenTy::RSquare,
            TokenTy::LCur => other_brac = TokenTy::RCur,
            _ => return Err(ParseError::ExpectButGot("Bracket".into(), t.clone()))
        }
        let mut open = 0 ; 
        let num = tokens
        .iter()
        .position(
            |x| 
            if x.ty == other_brac && open == 1 {true}
            else if x.ty == t.ty {open += 1;false} 
            else if x.ty == other_brac {open -= 1;false}
            else {false}
        ).ok_or(ParseError::NoClosingBracket)?;
        Ok((tokens[1..num].to_vec(), tokens[num+ 1 ..].to_vec()))
    }

    pub(crate) fn grab_fn( tokens : Vec<Token>) -> Result<((Vec<Token> , Vec<Token>) , Vec<Token>),ParseError>{
        let (x, rem) = Grabber::grab_tokens_before(tokens, TokenTy::LCur)?;
        let (y, rem) = Grabber::grab_brac(rem)?;
        Ok(((x,y),rem))
    }

    pub(crate) fn grab_prec2( tokens : Vec<Token>) -> Result<(Vec<Token>,Vec<Token>),ParseError> {
        let pos = tokens
        .iter()
        .position(
            |x| 
            match x.ty {
                TokenTy::Plus | TokenTy::Minus => true,
                _ => false,
            } 
        ).unwrap_or(tokens.len());
        Ok((tokens[..pos].to_vec() , tokens[pos..].to_vec()))
    }

    pub(crate) fn sep_on_comma( tokens : Vec<Token> ) -> Result<Vec<Vec<Token>>,ParseError> {
        let mut vecs : Vec<Vec<Token>> = Vec::new() ;
        let mut vec  : Vec<Token> = Vec::new();
        for token in tokens {
            if token.ty == TokenTy::Comma {
                vecs.push(vec.clone());
                vec.clear();
            } else {
                vec.push(token);
            }
        }
        vecs.push(vec);
        Ok(vecs)
    }

    pub(crate) fn sep_on_bool_op1( tokens : Vec<Token>) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let pos = tokens
        .iter()
        .position(|x| 
            match x.ty {
                TokenTy::LT | TokenTy::GT | TokenTy::LTEQ | TokenTy::GTEQ | TokenTy::EQ | TokenTy::NEQ
                => true,
                _ => false 
            }
        ).ok_or(ParseError::ExpectButGot("Bool operator".into(), Token::new(TokenTy::Value("()".into()),0)))?;
        Ok((tokens[..pos].to_vec(), tokens[pos..].to_vec()))
    }

    pub(crate) fn grab_and(tokens: Vec<Token>) -> Result<(Vec<Token> , Vec<Token>), ParseError> {
        let pos = tokens
        .iter()
        .position(|x| x.ty == TokenTy::BAnd)
        .ok_or(ParseError::ExpectButGot("And".into(),Token::new(TokenTy::Value("()".into()),0)))?;
        Ok((tokens[..pos].to_vec(), tokens[pos+1..].to_vec()))
    }

    pub(crate) fn grab_or(tokens: Vec<Token>) -> Result<(Vec<Token>, Vec<Token>),ParseError> {
        let pos = tokens
        .iter()
        .position(|x| x.ty == TokenTy::BOr)
        .ok_or(ParseError::ExpectButGot("And".into(),Token::new(TokenTy::Value("()".into()),0)))?;
        Ok((tokens[..pos].to_vec(), tokens[pos+1..].to_vec()))
    }

  
        
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
fn test_grab() -> Result<(),ParseError> {
        let z = Grabber::grab_tokens_before(make_tokens(vec![
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::LCur,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or
        ]), TokenTy::LCur)?;
        assert_eq!(z , ((make_tokens(vec![
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
        ]),make_tokens(vec![
            TokenTy::LCur,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or,
            TokenTy::Or
        ]))));
        Ok(())
    }
    #[test]
    fn test_grab_prec2() -> Result<(),ParseError>{
        let tokens = make_tokens(vec![
            TokenTy::Value("9".into()),
            TokenTy::Mul,
            TokenTy::Value("2".into()),
            TokenTy::Div,
            TokenTy::Value("3".into()),
            TokenTy::Plus,
            TokenTy::Value("9".into())
        ]);
        let (one,two) = Grabber::grab_prec2(tokens)?;
        assert_eq!(one,
            make_tokens(vec![
                TokenTy::Value("9".into()),
                TokenTy::Mul,
                TokenTy::Value("2".into()),
                TokenTy::Div,
                TokenTy::Value("3".into())
            ]
        ));
        assert_eq!(two,
            make_tokens(vec![
                TokenTy::Plus,
                TokenTy::Value("9".into())
            ]
        ));
        Ok(())
    }
    #[test]
    fn test_grab_brac() -> Result<(),ParseError>{
        let tokens = make_tokens(vec![
            TokenTy::LBrac,
            TokenTy::And,
            TokenTy::And,
            TokenTy::And,
            TokenTy::RBrac,
            TokenTy::Or
        ]);
        let (bef,after) = Grabber::grab_brac(tokens)?;
        assert_eq!(bef,make_tokens(vec![
            TokenTy::And,TokenTy::And,TokenTy::And
        ]));
        assert_eq!(after, make_tokens(vec![
            TokenTy::Or
        ]));
        Ok(())
    }

    fn make_tokens(tokens : Vec<TokenTy> ) -> Vec<Token> {
        let mut toks = Vec::new();
        for token in tokens {
            toks.push(Token::new(token, 0));
        }
        toks
    }
}