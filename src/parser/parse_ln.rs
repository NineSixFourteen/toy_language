use super::{Parser, tokenizer::{Token, TokenTy}, Line, ParseError, grabber::Grabber, Primitive, BoolNode, Node, NodeTy};


impl Parser {

    pub(crate) fn parse_print(tokens: Vec<Token> ) -> Result<(Line,Vec<Token>), ParseError> {
        let (line, rem) = Grabber::grab_line(tokens)?;
        let x = Parser::parse_expr(line[1..].to_vec())?;
        Ok((Line::Print(x),rem))        
     }
    
    pub(crate)fn parse_for(tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let (line, rem)     = Grabber::grab_brac(tokens[1..].to_vec())?;
        let (body, rem)     = Grabber::grab_brac(rem)?;
        let (init      , rem2)   = Parser::parse_line(line)?;
        let (bool, mut line2 )  = Grabber::grab_line(rem2)?; 
        let bool_exp     = Parser::parse_bool_expr(bool)?;
        line2.push(Token::new(TokenTy::SemiColan, 0));
        let (other_line, _ ) = Parser::parse_line(line2)?;
        let (lines, _ ) = Parser::parse_lines(body)?;

        Ok((Line::For(Box::new(init), bool_exp, Box::new(other_line), lines), rem))
    }

    pub(crate)fn parse_if(tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError>{
        let (line, rem ) = Grabber::grab_tokens_before(tokens, TokenTy::LCur)?;
        let bool = Parser::parse_bool_expr(line[1..].to_vec())?;
        let (body, rem ) = Grabber::grab_brac(rem)?;
        let (lines , _ ) = Parser::parse_lines(body)?;
        let (elses , rem) = Parser::parse_elses(rem)?;
        Ok((Line::If(bool, lines, elses),rem))
    }

    pub(crate) fn parse_elses(mut tokens: Vec<Token>) -> Result<(Vec<Line> , Vec<Token>),ParseError> {
        let mut lines = Vec::new();
        while tokens.len() != 0 && tokens.first().unwrap().ty == TokenTy::Else {
            let (line, rem ) = Grabber::grab_tokens_before(tokens, TokenTy::LCur)?;
            let bool;
            if line.len() == 1 {
                bool = BoolNode::TFVar("true".into());
            }else {
                bool = Parser::parse_bool_expr(line[2..].to_vec())?;
            }
            let (body, rem ) = Grabber::grab_brac(rem)?;
            let (lin , _ ) = Parser::parse_lines(body)?;
            lines.push(Line::If(bool, lin, Vec::new()));
            tokens = rem;
        } 
        Ok((lines, tokens))
    }

    pub(crate)fn parse_return(tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let (line, rem) = Grabber::grab_line(tokens)?;
        let expr = Parser::parse_expr(line[1..].to_vec())?;
        Ok((Line::Return(expr), rem))
    }

    pub(crate) fn parse_array_init(tokens: Vec<Token>) -> Result<(Line, Vec<Token>), ParseError> {
        let ty = tokens.get(0).unwrap().clone();
        let name = tokens.get(2).unwrap().clone();
        let (line, rem) = Grabber::grab_line(tokens)?;
        let (parts , _) = Grabber::grab_brac(line[4..].to_vec())?;
        let elems = Grabber::sep_on_comma(parts)?; 
        let mut elm = Vec::new();
        for el in elems {
            elm.push(Parser::parse_expr(el)?);
        } 
        let name = Parser::extrct_str(name)?;
        let ty = Parser::extrct_prm(ty)?;
        Ok((Line::InitVar(ty, name, NodeTy::Node(Node::Array(elm))),rem))
    }   

    pub(crate) fn parse_init_var( tokens:Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        if tokens.get(1).unwrap().ty == TokenTy::Array {
            Parser::parse_array_init(tokens)
        } else {
            let first = tokens.first().unwrap().clone();
            let n = tokens.get(1).unwrap().clone();
            let (line, rem) = Grabber::grab_line(tokens)?;
            let expr = Parser::parse_expr(line[3..].to_vec())?;
            let name: String;
            match n.ty{
                TokenTy::Value(x) => name = x,
                _ => {return Err(ParseError::ExpectButGot("Value".into(), n));}
            }
            let ty = match first.ty {
                TokenTy::Int => Primitive::Int,
                TokenTy::String => Primitive::String,
                TokenTy::Float =>  Primitive::Float,
                TokenTy::Char => Primitive::Char,
                TokenTy::Boolean => Primitive::Boolean,
                _ => {return Err(ParseError::ExpectButGot("Primitive/type".into(), first));}
            };
            Ok((Line::InitVar(ty, name , expr), rem))
        }
    }

    pub(crate)


    fn contains_bool(tokens : Vec<Token>) -> bool {
        for t in &tokens {
            let x = match t.ty {
                TokenTy::LT   | TokenTy::GT   |
                TokenTy::LTEQ | TokenTy::GTEQ |
                TokenTy::EQ   | TokenTy::NEQ => true,
                _ => false
            };
            if x {return x}
        }
        false
    }

    fn parse_expr(tokens : Vec<Token>) -> Result<NodeTy,ParseError> {
        if Parser::contains_bool(tokens.clone()) {
            let node = Parser::parse_bool_expr(tokens)?;
            Ok(NodeTy::BoolNode(node))
        } else {
            let node = Parser::parse_expr_nb(tokens)?;
            Ok(NodeTy::Node(node))
        }
    }

    //nb = no boolean 
    fn parse_expr_nb( mut tokens: Vec<Token>) -> Result<Node,ParseError> {
        if tokens.len() == 1 {
            return Parser::parse_val(tokens.first().unwrap().clone());
        }
        let lhs : Node ; 
        match tokens.get(1).unwrap().ty {
            TokenTy::LBrac => {
                (lhs, tokens) = Parser::parse_func(tokens)?;
            }
            _ => {
                lhs = Parser::parse_val(tokens.first().unwrap().clone())?;
                tokens =  tokens[1..].to_vec();
            }
        }
        if tokens.len() == 0 {
            return Ok(lhs);
        }
        let not = Token::new(TokenTy::Value("Nothing".into()),0);
        let t = tokens.first().unwrap_or(&not);
        match t.ty {
            TokenTy::Plus  => Ok(Node::Add(Box::new(lhs), Box::new(Parser::parse_expr_nb(tokens[1..].to_vec())?))),
            TokenTy::Minus => Ok(Node::Sub(Box::new(lhs), Box::new(Parser::parse_expr_nb(tokens[1..].to_vec())?))),
            TokenTy::Mul   |
            TokenTy::Div   => Parser::parse_prec2(lhs ,tokens),
        _ => {return Err(ParseError::ExpectButGot("Operator".into(),t.clone()))}
        }

    }

    fn parse_prec2(lh : Node, tokens: Vec<Token>) -> Result<Node,ParseError> {
        let (prec2, rem) = Grabber::grab_prec2(tokens[1..].to_vec())?;
        let lhs;
        let rhs = Parser::parse_prec2_helper(prec2)?;
        let not = Token::new(TokenTy::Value("Nothing".into()),0);
        let t = tokens.first().unwrap_or(&not);
        match t.ty {
            TokenTy::Mul => {
                lhs = Node::Mul(Box::new(lh), Box::new(rhs));
            }
            TokenTy::Div => {
                lhs = Node::Div(Box::new(lh), Box::new(rhs));

            }
            _ => {return Err(ParseError::ExpectButGot("Mul or Div".into(), t.clone()));}
        }
        if rem.len() == 0 {
            return Ok(lhs);
        }
        match rem.first().unwrap().ty {
            TokenTy::Plus  => Ok(Node::Add(Box::new(lhs), Box::new(Parser::parse_expr_nb(rem[1..].to_vec())?))),
            TokenTy::Minus => Ok(Node::Sub(Box::new(lhs), Box::new(Parser::parse_expr_nb(rem[1..].to_vec())?))),
            _ => unreachable!()
        }
    }

    fn parse_prec2_helper( mut tokens : Vec<Token> ) -> Result<Node,ParseError> {
        if tokens.len() == 1 {
            return Parser::parse_val(tokens.first().unwrap().clone());
        }
        let lhs : Node ; 
        match tokens.get(1).unwrap().ty {
            TokenTy::LBrac => {
                (lhs, tokens) = Parser::parse_func(tokens)?;
            }
            _ => {
                lhs = Parser::parse_val(tokens.first().unwrap().clone())?;
                tokens =  tokens[1..].to_vec();
            }
        }
        if tokens.len() == 0 {
            return Ok(lhs);
        }
        match tokens.first().unwrap().ty {
            TokenTy::Mul => Ok(Node::Mul(Box::new(lhs), Box::new(Parser::parse_prec2_helper(tokens[1..].to_vec())?))),
            TokenTy::Div => Ok(Node::Div(Box::new(lhs), Box::new(Parser::parse_prec2_helper(tokens[1..].to_vec())?))),
            _ => unreachable!()
        }
    }

    pub(crate) fn parse_func( tokens : Vec<Token> ) -> Result<(Node, Vec<Token>),ParseError> {
        let name = Parser::extrct_str(tokens.first().unwrap().clone())?;
        let (line, rem ) = Grabber::grab_brac(tokens[1..].to_vec())?;
        let nodes;
        if line.len() != 0 {
            nodes = Parser::parse_param(line)?;
        } else {
            nodes = Vec::new()
        }
        Ok((Node::FCall(name, nodes), rem))
    }

    fn parse_param( tokens : Vec<Token> ) -> Result<Vec<NodeTy>,ParseError> {
        let ndes = Grabber::sep_on_comma(tokens)?;
        let mut nodes = Vec::new();
        for nde in ndes {
            nodes.push(Parser::parse_expr(nde)?);
        } 
        Ok(nodes)
    }

    pub(crate) fn extrct_str( t : Token) -> Result<String,ParseError> {
        if let TokenTy::Value(x) = t.ty {
            Ok(x)
        } else {
            panic!("{:?}",t)
        }
    }
    
    pub(crate) fn extrct_prm( t: Token) -> Result<Primitive,ParseError> {
        match t.ty {
            TokenTy::Int => Ok(Primitive::Int),
            TokenTy::String => Ok(Primitive::String),
            TokenTy::Boolean => Ok(Primitive::Boolean),
            TokenTy::Char => Ok(Primitive::Char),
            TokenTy::Float => Ok(Primitive::Float),
            _ => panic!("{:?}",t)
        }
    }

    fn parse_val( t: Token ) -> Result<Node,ParseError> {
        match t.ty {
            TokenTy::Value(x) => Ok(Node::Leaf(x)),
            _ => panic!("{:?}",t)
        }
    }

    fn parse_bool_expr(tokens : Vec<Token>) -> Result<BoolNode,ParseError> {
        if no_and_or_or(tokens.clone()) {
            Parser::parse_bool_expr_2(tokens.clone())
        } else if only_ands(tokens.clone()) {
            Parser::parse_ands(tokens.clone()) 
        } else {
            let (b_or, a_or) = Grabber::grab_or(tokens)?;
            Ok(BoolNode::Or(Box::new(Parser::parse_ands(b_or)?), Box::new(Parser::parse_bool_expr(a_or)?)))
        }
    }
    pub(crate) fn parse_ands(tokens: Vec<Token>) -> Result<BoolNode, ParseError> {
        if no_and_or_or(tokens.clone()) {
            Parser::parse_bool_expr_2(tokens.clone())
        } else {
            let (b_and, a_and) = Grabber::grab_and(tokens)?;
            Ok(BoolNode::And(Box::new(Parser::parse_bool_expr_2(b_and)?), Box::new(Parser::parse_ands(a_and)?)))
        }
    }

    fn parse_bool_expr_2( tokens:Vec<Token>) -> Result<BoolNode,ParseError> {
        if tokens.len() == 1 {
            let tok = tokens.first().unwrap().clone();
            if let TokenTy::Value(x) = tok.ty {
                return Ok(BoolNode::TFVar(x));
            } else {
                panic!()
            }
        }
        let (bef, after) = Grabber::sep_on_bool_op1(tokens)?;
        let op = after.first().unwrap();
        let lhs = Parser::parse_expr_nb(bef)?;
        let rhs = Parser::parse_expr_nb(after[1..].to_vec())?;
        match op.ty {
            TokenTy::LT   =>  Ok(BoolNode::LThan(lhs,rhs)),
            TokenTy::GT   =>  Ok(BoolNode::GThan(lhs,rhs)),
            TokenTy::GTEQ =>  Ok(BoolNode::GThanEq(lhs,rhs)),
            TokenTy::LTEQ =>  Ok(BoolNode::LThanEq(lhs,rhs)),
            TokenTy::EQ   =>  Ok(BoolNode::Eq(lhs,rhs)),
            TokenTy::NEQ  =>  Ok(BoolNode::NEq(lhs,rhs)),
            _ => {return Err(ParseError::ExpectButGot("Boolean operator".into(), op.clone()));}
        }
    }

    pub(crate) fn parse_fcall( tokens : Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let (mut line, rem ) = Grabber::grab_line(tokens)?;
        let name = Parser::extrct_str(line.pop().unwrap())?;
        let (par, _ ) = Grabber::grab_brac(line)?;
        let params = Grabber::sep_on_comma(par)?;
        let mut nodes = Vec::new();
        for param in params {
            nodes.push(Parser::parse_expr(param)?);
        }
        Ok((Line::FCall(
            Node::FCall(
                name, 
                nodes
           )
        ),rem))
    }

    pub(crate) fn parse_overwrite(tokens : Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let (node, rem ) = Grabber::grab_line(tokens)?;
        let name = Parser::extrct_str(node.first().unwrap().clone())?;
        let expr = Parser::parse_expr(node[2..].to_vec())?;
        Ok((Line::OverVar(name, expr),rem))
    }

}

fn only_ands(tokens: Vec<Token>) -> bool {
    for token in tokens {
        match token.ty {
            TokenTy::BOr => return false,
            _ => {}
        }
    }
    true
}

fn no_and_or_or(tokens: Vec<Token>) -> bool {
    for token in tokens {
        match token.ty {
            TokenTy::BAnd => return false,
            TokenTy::BOr => return false,
            _ => {}
        }
    }
    true
}

#[cfg(test)] 
mod tests {

    fn node(string : &str) -> NodeTy {
        NodeTy::Node(Node::Leaf(string.into()))
    }

    use crate::parser::{Node, tokenizer::Tokenizer};

    use super::*;
    #[test]
    fn test_parse_val() -> Result<(),ParseError> {
        let token = TokenTy::Value("90".into());
        let z = Parser::parse_val(Token::new(token, 0))?;
        assert_eq!(z,Node::Leaf("90".into()));
        Ok(())
    }

    #[test]
    fn test_parse_print() -> Result<(), ParseError>{
        let tokens = vec![
            TokenTy::Print,
            TokenTy::Value("9".into()),
            TokenTy::Plus,
            TokenTy::Value("9".into()),
            TokenTy::SemiColan
        ];
        let (p,_) = Parser::parse_print(make_tokens(tokens))?;
        assert_eq!(p, 
            Line::Print(
                NodeTy::Node(Node::Add(
                    Box::new(Node::Leaf("9".into())),
                    Box::new(Node::Leaf("9".into()))
                )))
        );
        Ok(())
    }

    #[test] 
    fn test_parse_expr() -> Result<(),ParseError>{
        let tokens = 
        vec![
            TokenTy::Value("9".into()),
            TokenTy::Plus,
            TokenTy::Value("10".into()),
            TokenTy::Mul,
            TokenTy::Value("2".into()),
            TokenTy::Div,
            TokenTy::Value("12".into()),
            TokenTy::Plus,
            TokenTy::Value("43".into())
        ];
        let res = Parser::parse_expr(make_tokens(tokens))?;
        assert_eq!(res ,
        NodeTy::Node(Node::Add(
            Box::new(Node::Leaf("9".into())),
            Box::new(Node::Add(
                Box::new(Node::Mul(
                    Box::new(Node::Leaf("10".into())), 
                    Box::new(Node::Div(
                        Box::new(Node::Leaf("2".into())), 
                        Box::new(Node::Leaf("12".into()))
                    ))
                )),
                Box::new(Node::Leaf("43".into()))))  
        )));
        Ok(())
    }

    #[test]
    fn test_stuff() -> Result<(),ParseError> {
        let mut tokenizer = Tokenizer::new("Print 100 + 10 * 3 / 6 + 10;
        ");
        tokenizer.tokenize();
        let (line, _) = Parser::parse_line(tokenizer.tokens)?;
        assert_eq!(
            line,
            Line::Print(
                NodeTy::Node(Node::Add(
                    Box::new(Node::Leaf("100".into())),
                    Box::new(Node::Add(
                        Box::new(Node::Mul(
                            Box::new(Node::Leaf("10".into())), 
                            Box::new(Node::Div(
                                Box::new(Node::Leaf("3".into())), 
                                Box::new(Node::Leaf("6".into()))
                            )))),
                        Box::new(Node::Leaf("10".into()))
                    ))
                )
            )
        ));
        Ok(())
    }

    fn make_tokens(tokens : Vec<TokenTy> ) -> Vec<Token> {
        let mut  toks = Vec::new();
        for token in tokens {
            toks.push(Token::new(token, 0));
        }
        toks
    }

}
