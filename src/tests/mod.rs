#[allow(dead_code,unused_imports)]
mod tests{

        use crate::{parser::{NodeTy, tokenizer::Tokenizer, Parser, Line, Node, BoolNode}, stack_machine::{StrError, Value, Evaluator::Evaluator}, compiler::Compiler};

    /* 
    #[test]
    fn test_for() {
        let string = "for i, 0, 15 {Print 100;}";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let y = Parser::parse_for(tokens);
        let mut x = Line::Print(Node::Nothing) ; 
        match y {
            Ok((z,_)) => x = z,
            Err(_) => {},
        }
        assert_eq!(x,
        Line::For(
            "i".into(),
            Node::Leaf("0".into()),
            Node::Leaf("15".into()),
            vec![
                Line::Print(
                    Node::Leaf("100".into())
                )
            ]
        ));
    }
    */

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
                ])   
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
                Print 111; 
                return 100;
            }
            
            def int bob(int x , int y) {
                return 10 + x + y;
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
            commands : Vec::new()
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