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


pub(crate) struct Parser {}

impl Parser {

    pub fn parse(&self, tokens: Vec<Token> ) -> Program{
        let (_, funcs) = self.parse_fns(tokens);
        let pos = funcs.iter().position(|x| x.name.eq("main".into())).unwrap();
        Program { 
            main: funcs.get(pos).unwrap().clone(), 
            methods: {
                let mut x= funcs[0..pos].to_vec();
                x.append(&mut funcs[pos+1..].to_vec());
                x
            }
        }
    }

    fn parse_fns(&self, mut tokens: Vec<Token>) -> (Vec<Token> , Vec<Function>) {
        let mut funcs = Vec::new();
        while tokens.len() != 0 && tokens.first().unwrap() == &Token::Def  {
            let (func, rem) = self.parse_fn(tokens);
            tokens = rem;
            funcs.push(func);
        }
        (tokens,funcs)
    }

    fn parse_fn(&self, tokens: Vec<Token> ) -> (Function , Vec<Token>) {
        let y = Grabber{};
        let ((start,body), rem ) = y.grab_fn(tokens);
        let ty = self.extrct_prm(start.get(1).unwrap().clone());
        let name = self.extrct_str(start.get(2).unwrap().clone());
        let parms = y.sep_on_comma(start[4..start.len() - 1].into());
        let params = self.parse_into_params(parms); 
        let (body, _ ) = self.parse_lines(body);
        let func = Function{
            name,
            ty,
            body,
            params
        };
        (func, rem)
    }

    pub(crate) fn parse_lines(&self, mut tokens: Vec<Token>) -> (Vec<Line>, Vec<Token>) {
        let mut x = Vec::new();
        while !tokens.is_empty(){
            let (line, rem ) = self.parse_line(tokens);
            tokens = rem;
            x.push(line);
        }
        (x,tokens)
    }

    fn parse_line(&self, tokens: Vec<Token> ) -> (Line,Vec<Token>) {
        match tokens.first().unwrap() {
            Token::Print => self.parse_print(tokens),
            Token::For => self.parse_for(tokens),
            Token::If => self.parse_if(tokens),
            Token::Else => self.parse_else(tokens),
            Token::Return => self.parse_return(tokens),
            Token::Int | Token::String => self.parse_init_var(tokens),
            _ => self.parse_non_line(tokens)
        }
    }

    fn parse_non_line(&self,tokens : Vec<Token>) -> (Line, Vec<Token>) {
        if tokens.len() < 2 {
            panic!("To small")
        }
        match tokens.get(1).unwrap() {
            Token::LBrac => self.parse_fcall(tokens),
            Token::Equal => self.parse_overwrite(tokens),
            _ => panic!("Unkown line")
        }
    }

    fn parse_into_params(&self,tokens : Vec<Vec<Token>> ) -> Vec<(String, Primitive)> {
        if tokens.len() == 1 && tokens.get(0).unwrap().len() == 0  {
            return Vec::new();
        }
        let mut vec = Vec::new();
        for pair in tokens {
            if pair.len() != 2{
                panic!();
            }
            vec.push((self.extrct_str(pair.get(1).unwrap().clone()),self.extrct_prm(pair.get(0).unwrap().clone())));
        }
        vec
    }




}

#[cfg(test)]
mod tests {

    use crate::parser::Primitive;

    use super::{Tokenizer,Parser,Line,Function, Node};

    #[test]
    fn test_parse_fn() {
        let string = 
        "
            def main(int x , int y) {
                Print x ; 
                Print y ; 
            }
        ";
        let mut tokenizer = Tokenizer::new(string);
        let parser = Parser{};
        tokenizer.tokenize();
        let (func , _) = parser.parse_fn(tokenizer.tokens);
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
        
    }
}
