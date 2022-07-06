pub(crate) mod tokenizer;
#[allow(dead_code)]
pub(crate) mod grabber;
use tokenizer::*;
use grabber::*;



mod parse_ex;
mod parse_ln;
mod parse_fn;

struct Program {
    main : Function,
    methods : Vec<Function>
}
pub(crate) struct Function {
    name : String, 
    body : Vec<Line> 
}

enum Primitive {
    Int, 
    String
}

enum Line {
    Print(Node),
    InitVar(Primitive,String,Node),
    OverVar(String, Node),
    For(Box<Line>, Node, Vec<Line>),
    If(Node, Vec<Line>)
}

enum Node {
    Add(Box<Node> , Box<Node>),
    Sub(Box<Node> , Box<Node>),
    Mul(Box<Node> , Box<Node>),
    Div(Box<Node> , Box<Node>),
    Leaf(String),
    LoadVar(String),
    FCall(String, Vec<Node>),
    Nothing
}


struct Parser {}

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

    fn parse_lines(&self, mut tokens: Vec<Token>) -> (Vec<Line>, Vec<Token>) {
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
            Token::Int => self.parse_int(tokens),
            Token::String => self.parse_string(tokens),
            _ => todo!()
        }
    }


}

