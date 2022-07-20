#[allow(dead_code,unused_imports)]
mod tests{
        use std::collections::HashMap;
        use crate::{parser::{NodeTy, tokenizer::Tokenizer, Parser, Line, Node, BoolNode, Primitive}, stack_machine::{StrError, Value, Evaluator::Evaluator}, compiler::Compiler};
    
        #[test]
    fn test_for() {
        let string = "for(int i = 0; i < 15;i = i + 1) {Print 100;}";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let y = Parser::parse_for(tokens);
        let mut x = Line::Print(NodeTy::Node(Node::Nothing)) ; 
        match y {
            Ok((z,_)) => x = z,
            Err(_) => {},
        }
        assert_eq!(x,
            Line::For(
            Box::new(Line::InitVar(Primitive::Int, "i".into(),NodeTy::Node(Node::Leaf("0".into())))),
            BoolNode::LThan(Node::Leaf("i".into()), Node::Leaf("15".into())),
            Box::new(Line::OverVar("i".into(), NodeTy::Node(Node::Add(Box::new(Node::Leaf("i".into())), Box::new(Node::Leaf("1".into())))))),
            vec![
                Line::Print(NodeTy::Node(Node::Leaf("100".into())))
            ]
            )
        );
    }
    

    #[test]
    fn test_if() {
        let string = "if i < 10 {Print 100;}";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let y = Parser::parse_if(tokens);
        let mut x = Line::Print(NodeTy::Node(Node::Nothing)); 
        match y {
            Ok((z,_)) => x = z,
            Err(_) => {},
        }
        assert_eq!(x,
            Line::If(
                BoolNode::LThan(
                    Node::Leaf("i".into()),
                    Node::Leaf("10".into())
                ), 
                vec![
                  Line::Print(NodeTy::Node(Node::Leaf("100".into())))  
                ],Vec::new())
        );
    }

    #[test]
    fn test_full_ting() -> Result<(), StrError> {
        let string = 
        "
            int i = 100 + 10;
            return i;
        ";
        test_code(string, Value::Int(110))
    }

    #[test]
    fn test_full_ting2() -> Result<(), StrError> {
        let string = 
        "
            int i = 100 + 10;
            Print i ;  
            return i;
        ";
        test_code(string, Value::Int(110))
    }
    
    #[test]
    fn test_full_ting_for() -> Result<(),StrError> {
        let string = 
        "
            int i = 100 + 10;
            Print i ;  
            for(int f = 30 ; f > 15; f = f - 1){
                Print f ; 
            }
            return i;
        ";
        test_code(string, Value::Int(110))
    }

    #[test]
    fn test_tingy_wingy() -> Result<(), StrError> {
        let string = 
        "
            int i = 2; 
            for(int f = 0; f < 8; f = f + 1) {
                Print i;
                i = i * 2 ;
            }
            return i ;
        ";
        test_code(string, Value::Int(512))
    }

    #[test]
    fn test_if_wiffy() -> Result<(),StrError> {
        let string = 
        "
            int i = 100 * 10 - 5 / 5;
            if 9 < 10 {
                return 199;
            }

            return i ; 
        ";
        test_code(string, Value::Int(199))
    }

    #[test]
    fn test_whole() -> Result<(),StrError> {
        let message = 
        "
            def int main() {
                Print bob(lol(lol(lol(lol(lol(lol(10)))))) , 22); 
                Print cons(false);
                Print 111; 
                return 100;
            }
            
            def int bob(int x , int y) {
                return 10 + x + y;
            }

            def int cons(boolean b) {
                if b { 
                    return 1;
                }
                return 9; 
            }

            def int lol(int x) {
                return x;
            }
        ";
        test_prog(message, Value::Int(100))
    }

    #[test]
    fn test_ty() -> Result<(),StrError> {
        let message = 
        "
            def int main(){
                boolean b = 9 > 10; 
                return b;
            }
        ";
        test_prog(message, Value::Boolean(false))
    }

    #[test] 
    fn test_nutty_if() -> Result<(),StrError> {
        let message = 
        "
            def int main(){
                if false  {
                    if false {
                        Print 100;
                    }
                    return 10 ;
                } else if false {
                    if false {
                        Print 100;
                    } else if true {
                        Print 10; 
                    }
                    return 2 ;
                } else if false {
                    for(int i = 0; i < 100; i = i + 1){
                        if true {
                            Print 100;
                        }
                    }
                    return 3 ; 
                } else if true {
                    return 100;
                } else {
                    Print 100;
                }
                Print 100;
                return 10;
            }
        ";
        test_prog(message, Value::Int(100))
    }

    fn test_prog(message : &str, val : Value) -> Result<(),StrError> {
        let mut tokenizer = Tokenizer::new(message);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let y = Parser::parse(tokens);
        let x ; 
        match y {
            Ok(z) => x = z,
            Err(x) => panic!("ParseError : {:?}" , x),
        }
        let mut eval = Compiler::compile(x);
        let res = eval.eval()?;
        assert_eq!(res, val);
        Ok(())
    }

    fn test_code(string : &str, val : Value) -> Result<(),StrError> {
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let y = Parser::parse_lines(tokens);
        let x ; 
        match y {
            Ok((z,_)) => x = z,
            Err(x) => panic!("ParseError : {:?}",x),
        }
        let mut compiler = Compiler{
            commands : Vec::new(),
            vars : HashMap::new(),
            funcs : HashMap::new()
        };
        compiler.compile_lines(x);
        let commands = compiler.commands;
        for command in &commands{
            println!("{:?}",command);
        }
        let mut evaluator = Evaluator::new_e(commands);
        let res = evaluator.eval()?;
        assert_eq!(res, val);
        Ok(())
    }
}