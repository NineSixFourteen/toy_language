pub(crate) struct Grabber{}
use super::*;
impl Grabber{
    pub(crate) fn grab_tokens_before(&self, tokens : Vec<Token> , t:Token) -> (Vec<Token>, Vec<Token>) {
        let num = tokens
        .iter()
        .position(|x| x == &t)
        .unwrap_or(tokens.len());
        (tokens[..num].to_vec(),tokens[num..].to_vec())
    }

    pub(crate) fn grab_line(&self, tokens : Vec<Token>) -> (Vec<Token>,Vec<Token> ){
        self.grab_tokens_before(tokens, Token::SemiColan)
    }

    pub(crate) fn grab_brac(&self, tokens : Vec<Token>) -> (Vec<Token>, Vec<Token>) {
        let t = tokens.first().unwrap();
        let mut other_brac = Token::Comma;
        match t {
            Token::LBrac => other_brac = Token::RBrac,
            Token::LSquare => other_brac = Token::RSquare,
            Token::LCur => other_brac = Token::RCur,
            _ => {}
        }
        let mut open = 0 ; 
        let num = tokens
        .iter()
        .position(|x| 
        if x == &other_brac && open == 0 {
            true
        } else if x == t {
            open += 1 ;
            false
        } else if x == &other_brac {
            open -= 1 ;
            false
        } else {
           false
        }
        ).unwrap_or(tokens.len());
        (tokens[1..num].to_vec(), tokens[num + 1..].to_vec())
    }

    pub(crate) fn grab_fn(&self, tokens : Vec<Token>) -> ((Vec<Token> , Vec<Token>) , Vec<Token>){
        let (x, rem) = self.grab_tokens_before(tokens, Token::LCur);
        let (y, rem) = self.grab_brac(rem);
        ((x,y),rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_grab() {
        let mut y = Grabber {};
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
        ], Token::LCur);
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
        ]))
    }
}