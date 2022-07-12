// Test completeness of program
#[cfg(test)]
mod tests {
    use std::collections::btree_map::Values;

    use crate::{parser::{*, tokenizer::Tokenizer, self}, compiler::{self, Compiler}, stack_machine::{Evaluator, StrError,Value}};

    #[test]
    fn test_for() {
        let parser = Parser{};
        let string = "for i, 0, 15 {Print 100;}";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let (x,_) = parser.parse_for(tokens);
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
    #[test]
    fn test_if() {
        let parser = Parser{};
        let string = "if i < 10 {Print 100;}";
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let (x,_) = parser.parse_if(tokens);
        assert_eq!(x,
            Line::If(
                BoolNode::LThan(
                    Node::Leaf("i".into()),
                    Node::Leaf("10".into())
                ), 
                vec![
                  Line::Print(Node::Leaf("100".into()))  
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
            for f, 0, 15 {
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
            for f, 0 , 8 {
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

    fn test_code(string : &str, val : Value) -> Result<(),StrError> {
        let parser = Parser{};
        let mut tokenizer = Tokenizer::new(string);
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let (x, _) = parser.parse_lines(tokens);
        let mut compiler = Compiler{
            commands : Vec::new()
        };
        compiler.compile_lines(x);
        let commands = compiler.commands;
        for command in &commands{
            println!("{:?}",command);
        }
        let mut evaluator = Evaluator::Evaluator::new_e(commands);
        let res = evaluator.eval()?;
        assert_eq!(res, val);
        Ok(())
    }
}