pub(crate) mod tokenizer;
#[allow(dead_code)]
pub(crate) mod grabber;
use tokenizer::*;
use grabber::*;

mod parse_ln;

pub(crate) struct Program {
    main : Function,
    methods : Vec<Function>
}
pub(crate) struct Function {
    name : String, 
    body : Vec<Line> 
}
#[derive(Debug,PartialEq)]
pub(crate) enum Primitive {
    Int, 
    String
}
#[derive(PartialEq, Debug)]
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

#[derive(PartialEq,Debug)]
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

    fn parse(&self, _tokens: Vec<Token> ) -> Program{
        todo!()
    }

    fn parse_fns(&self, tokens: Vec<Token>) -> (Vec<Token> , Vec<Function>) {
        //let funcs = Vec::new();
        while tokens.first().unwrap() == &Token::Def  {
            
        }
        //(tokens,funcs)
        todo!()
    }

    fn parse_fn(&self, tokens: Vec<Token> ) -> (Function , Vec<Token>) {
        let y = Grabber{};
        let ((_start,body), rem ) = y.grab_fn(tokens);
        let (lines, _ ) = self.parse_lines(body);
        let func = Function{
            name: "Main".into(),
            body: lines
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
        println!("{:?}", &tokens.first().unwrap());
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
        println!("{:?}",tokens.first().unwrap());
        match tokens.get(1).unwrap() {
            Token::LBrac => self.parse_fcall(tokens),
            Token::Equal => self.parse_overwrite(tokens),
            _ => panic!("Unkown line")
        }
    }



}

