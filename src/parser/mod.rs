pub(crate) mod tokenizer;
#[allow(dead_code)]
pub(crate) mod grabber;
use tokenizer::*;
use grabber::*;

mod parse_ln;

pub(crate) struct Program {
    pub main : Function,
    pub methods : Vec<Function>
}
#[derive(PartialEq,Debug,Clone)]
pub(crate) struct Function {
   pub name   : String, 
   pub ty     : Primitive,
   pub body   : Vec<Line>,
   pub params : Vec<(String, Primitive)> 
}
#[derive(Debug,PartialEq,Clone)]
pub(crate) enum Primitive {
    Int, 
    String
}
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Line {
    Print(Node),
    FCall(Node),
    InitVar(Primitive,String,Node),
    OverVar(String, Node),
    For(String, Node, Node, Vec<Line>),
    If(BoolNode, Vec<Line>),
    Return(Node)
}

#[derive(PartialEq,Debug,Clone)]
pub(crate) enum Node {
    Add(Box<Node> , Box<Node>),
    Sub(Box<Node> , Box<Node>),
    Mul(Box<Node> , Box<Node>),
    Div(Box<Node> , Box<Node>),
    Leaf(String),
    LoadVar(String),
    FCall(String, Vec<Node>),
    Nothing
}

#[derive(PartialEq,Debug,Clone)]
pub(crate) enum BoolNode {
    LThan(Node,Node),
    GThan(Node,Node),
    GThanEq(Node,Node),
    LThanEq(Node,Node),
    Eq(Node,Node),
    NEq(Node,Node),
    And(Box<BoolNode>,Box<BoolNode>),
    Or(Box<BoolNode>,Box<BoolNode>),
    Not(Box<BoolNode>)
}
#[derive(Debug)]
pub(crate) enum ParseError {
    ExpectButGot(String, Token),
    NoClosingBracket,
    NotValidParamter,
    CantFindToken(Token),
    NoMainFunction

} 


pub(crate) struct Parser {}

impl Parser {

    pub fn parse(tokens: Vec<Token> ) -> Result<Program,ParseError>{
        let (_, funcs) = Parser::parse_fns(tokens)?;
        let pos = funcs.iter().position(|x| x.name.eq("main".into())).ok_or(ParseError::NoMainFunction)?;
        Ok(Program { 
            main: funcs.get(pos).unwrap().clone(), 
            methods: {
                let mut x= funcs[0..pos].to_vec();
                x.append(&mut funcs[pos+1..].to_vec());
                x
            }
        })
    }

    fn parse_fns(mut tokens: Vec<Token>) -> Result<(Vec<Token> , Vec<Function>),ParseError> {
        let mut funcs = Vec::new();
        while tokens.len() != 0 {
            if tokens.first().unwrap() != &Token::Def {
                return Err(ParseError::ExpectButGot("Def".into(), tokens.first().unwrap().clone()));
            }
            let (func, rem) = Parser::parse_fn(tokens)?;
            tokens = rem;
            funcs.push(func);
        }
        Ok((tokens,funcs))
    }

    fn parse_fn( tokens: Vec<Token> ) -> Result<(Function , Vec<Token>),ParseError> {
        let y = Grabber{};
        let ((start,body), rem ) = y.grab_fn(tokens)?;
        let ty = Parser::extrct_prm(start.get(1).unwrap().clone())?;
        let name = Parser::extrct_str(start.get(2).unwrap().clone())?;
        let parms = y.sep_on_comma(start[4..start.len() - 1].into())?;
        let params = Parser::parse_into_params(parms)?; 
        let (body, _ ) = Parser::parse_lines(body)?;
        let func = Function{
            name,
            ty,
            body,
            params
        };
        Ok((func, rem))
    }

    pub(crate) fn parse_lines(mut tokens: Vec<Token>) -> Result<(Vec<Line>, Vec<Token>),ParseError> {
        let mut x = Vec::new();
        while !tokens.is_empty(){
            let (line, rem ) = Parser::parse_line(tokens)?;
            tokens = rem;
            x.push(line);
        }
        Ok((x,tokens))
    }

    fn parse_line( tokens: Vec<Token> ) -> Result<(Line,Vec<Token>),ParseError> {
        match tokens.first().unwrap() {
            Token::Print => Parser::parse_print(tokens),
            Token::For => Parser::parse_for(tokens),
            Token::If => Parser::parse_if(tokens),
            Token::Else => todo!(),
            Token::Return => Parser::parse_return(tokens),
            Token::Int | Token::String => Parser::parse_init_var(tokens),
            _ => Parser::parse_non_line(tokens)
        }
    }

    fn parse_non_line(tokens : Vec<Token>) -> Result<(Line, Vec<Token>), ParseError> {
        if tokens.len() < 2 {
            panic!("To small")
        }
        match tokens.get(1).unwrap() {
            Token::LBrac => Parser::parse_fcall(tokens),
            Token::Equal => Parser::parse_overwrite(tokens),
            _ => panic!("Unkown line")
        }
    }

    fn parse_into_params(tokens : Vec<Vec<Token>> ) -> Result<Vec<(String, Primitive)>,ParseError> {
        if tokens.len() == 1 && tokens.get(0).unwrap().len() == 0  {
            return Ok(Vec::new());
        }
        let mut vec = Vec::new();
        for pair in tokens {
            if pair.len() != 2{
                return Err(ParseError::NotValidParamter);
            }
            vec.push((Parser::extrct_str(pair.get(1).unwrap().clone())?,Parser::extrct_prm(pair.get(0).unwrap().clone())?));
        }
        Ok(vec)
    }




}

#[cfg(test)]
mod tests {
    use crate::parser::Primitive;

    use super::{Tokenizer,Parser,Line,Function, Node, ParseError};

    #[test]
    fn test_parse_fn() -> Result<(), ParseError> {
        let string = 
        "
            def int main(int x , int y) {
                Print x ; 
                Print y ; 
            }
        ";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let (func , _) = Parser::parse_fn(tokenizer.tokens)?;
        assert_eq!(func,
        Function{ 
            name: "main".into(), 
            ty : Primitive::Int,
            body: vec![
                Line::Print(Node::Leaf("x".into())),
                Line::Print(Node::Leaf("y".into()))
            ], 
            params: vec![
                ("x".into(),Primitive::Int),
                ("y".into(),Primitive::Int)
            ] 
        });
        Ok(())
    }
}
