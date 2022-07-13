
use super::*;
impl Parser {
    
    pub(crate) fn parse_print(&self, tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let y = Grabber{};
        let (line, rem) = y.grab_line(tokens);
        let expr = self.parse_expr(line[1..].to_vec())?;
        Ok((Line::Print(expr), rem))
    }

     pub(crate)fn parse_for(&self, tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let grabber = Grabber{};
        let name = tokens.get(1).unwrap().clone();
        let (line,rem) = grabber.grab_tokens_before(tokens, Token::LCur);
        let (body, rem) = grabber.grab_brac(rem);
        let parts = grabber.sep_on_comma(line[3..].to_vec());
        let mut nodes = Vec::new();
        for part in parts {
            nodes.push(self.parse_expr(part)?);
        }
        let n : String;
        match name {
            Token::Value(x) => n = x,
            _ => panic!()
        }
        let (lines, _) = self.parse_lines(body)?;
        Ok((
            Line::For(
                n, 
                nodes.get(0).unwrap().clone(),
                nodes.get(1).unwrap().clone(),
                lines),
            rem
        ))
    }

     pub(crate)fn parse_if(&self, tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError>{
        let grabber = Grabber{};
        let (line, rem ) = grabber.grab_tokens_before(tokens, Token::LCur);
        let bool = self.parse_bool_expr(line[1..].to_vec())?;
        let (body, rem ) = grabber.grab_brac(rem);
        let (lines , _ ) = self.parse_lines(body)?;
        Ok((Line::If(bool, lines),rem))
    }

    pub(crate) fn parse_else(&self, _tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    pub(crate)fn parse_return(&self, tokens: Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let y = Grabber{};
        let (line, rem) = y.grab_line(tokens);
        let expr = self.parse_expr(line[1..].to_vec())?;
        Ok((Line::Return(expr), rem))
    }

    pub(crate) fn parse_init_var(&self, tokens:Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let y = Grabber{};
        let first = tokens.first().unwrap().clone();
        let n = tokens.get(1).unwrap().clone();
        let (line, rem) = y.grab_line(tokens);
        let expr = self.parse_expr(line[3..].to_vec())?;
        let name: String;
        match n{
            Token::Value(x) => name = x,
            _ => panic!()
        }
        let mut ty  = Primitive::Int;
        match first {
            Token::Int => {}
            Token::String => ty = Primitive::String,
            _ => panic!()
        }
        Ok((Line::InitVar(ty, name , expr), rem))
    }


    fn parse_expr(&self, mut tokens: Vec<Token>) -> Result<Node,ParseError> {
        if tokens.len() == 1 {
            return self.parse_val(tokens.first().unwrap().clone());
        }
        let lhs : Node ; 
        match tokens.get(1).unwrap() {
            Token::LBrac => {
                (lhs, tokens) = self.parse_func(tokens)?;
            }
            _ => {
                lhs = self.parse_val(tokens.first().unwrap().clone())?;
                tokens =  tokens[1..].to_vec();
            }
        }
        if tokens.len() == 0 {
            return Ok(lhs);
        }
        match tokens.first().unwrap() {
            Token::Plus  => Ok(Node::Add(Box::new(lhs), Box::new(self.parse_expr(tokens[1..].to_vec())?))),
            Token::Minus => Ok(Node::Sub(Box::new(lhs), Box::new(self.parse_expr(tokens[1..].to_vec())?))),
            Token::Mul   |
            Token::Div   => self.parse_prec2(lhs ,tokens),
            _ => panic!()
        }

    }

    fn parse_prec2(&self, lh : Node, tokens: Vec<Token>) -> Result<Node,ParseError> {
        let grabber = Grabber{};
        let (prec2, rem) = grabber.grab_prec2(tokens[1..].to_vec());
        let lhs;
        let rhs = self.parse_prec2_helper(prec2)?;
        match tokens.first().unwrap() {
            Token::Mul => {
                lhs = Node::Mul(Box::new(lh), Box::new(rhs));
            }
            Token::Div => {
                lhs = Node::Div(Box::new(lh), Box::new(rhs));

            }
            _ => panic!()
        }
        if rem.len() == 0 {
            return Ok(lhs);
        }
        match rem.first().unwrap(){
            Token::Plus  => Ok(Node::Add(Box::new(lhs), Box::new(self.parse_expr(rem[1..].to_vec())?))),
            Token::Minus => Ok(Node::Sub(Box::new(lhs), Box::new(self.parse_expr(rem[1..].to_vec())?))),
            _ => unreachable!()
        }
    }

    fn parse_prec2_helper(&self, mut tokens : Vec<Token> ) -> Result<Node,ParseError> {
        if tokens.len() == 1 {
            return self.parse_val(tokens.first().unwrap().clone());
        }
        let lhs : Node ; 
        match tokens.get(1).unwrap() {
            Token::LBrac => {
                (lhs, tokens) = self.parse_func(tokens)?;
            }
            _ => {
                lhs = self.parse_val(tokens.first().unwrap().clone())?;
                tokens =  tokens[1..].to_vec();
            }
        }
        if tokens.len() == 0 {
            return Ok(lhs);
        }
        match tokens.first().unwrap() {
            Token::Mul => Ok(Node::Mul(Box::new(lhs), Box::new(self.parse_prec2_helper(tokens[1..].to_vec())?))),
            Token::Div => Ok(Node::Div(Box::new(lhs), Box::new(self.parse_prec2_helper(tokens[1..].to_vec())?))),
            _ => panic!()
        }
    }


    pub(crate) fn parse_func(&self, tokens : Vec<Token> ) -> Result<(Node, Vec<Token>),ParseError> {
        let name = self.extrct_str(tokens.first().unwrap().clone())?;
        let grabber = Grabber{};
        let (line, rem ) = grabber.grab_brac(tokens[1..].to_vec());
        let nodes;
        if line.len() != 0 {
            nodes = self.parse_param(line)?;
        } else {
            nodes = Vec::new()
        }
        Ok((Node::FCall(name, nodes), rem))
    }

    fn parse_param(&self, tokens : Vec<Token> ) -> Result<Vec<Node>,ParseError> {
        let grabber = Grabber{};
        let ndes = grabber.sep_on_comma(tokens);
        let mut nodes = Vec::new();
        for nde in ndes {
            nodes.push(self.parse_expr(nde)?);
        } 
        Ok(nodes)
    }

    pub(crate) fn extrct_str(&self, t : Token) -> Result<String,ParseError> {
        if let Token::Value(x) = t {
            Ok(x)
        } else {
            panic!("{:?}",t)
        }
    }
    
    pub(crate) fn extrct_prm(&self, t: Token) -> Result<Primitive,ParseError> {
        match t {
            Token::Int => Ok(Primitive::Int),
            Token::String => Ok(Primitive::String),
            _ => panic!("{:?}",t)
        }
    }

    fn parse_val(&self, t: Token ) -> Result<Node,ParseError> {
        match t {
            Token::Value(x) => Ok(Node::Leaf(x)),
            _ => panic!("{:?}",t)
        }
    }

   
    fn parse_bool_expr(&self, tokens:Vec<Token>) -> Result<BoolNode,ParseError> {
        let grabber = Grabber{};
        let (bef, after) = grabber.sep_on_bool_op1(tokens);
        let op = after.first().unwrap();
        let lhs = self.parse_expr(bef)?;
        let rhs = self.parse_expr(after[1..].to_vec())?;
        match op {
            Token::LT   =>  Ok(BoolNode::LThan(lhs,rhs)),
            Token::GT   =>  Ok(BoolNode::GThan(lhs,rhs)),
            Token::GTEQ =>  Ok(BoolNode::GThanEq(lhs,rhs)),
            Token::LTEQ =>  Ok(BoolNode::LThanEq(lhs,rhs)),
            Token::EQ   =>  Ok(BoolNode::Eq(lhs,rhs)),
            Token::NEQ  =>  Ok(BoolNode::NEq(lhs,rhs)),
            _ => panic!()
        }
    }

    pub(crate) fn parse_fcall(&self, tokens : Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let grabber = Grabber{};
        let (mut line, rem ) = grabber.grab_line(tokens);
        let name = self.extrct_str(line.pop().unwrap())?;
        let (par, _ ) = grabber.grab_brac(line);
        let params = grabber.sep_on_comma(par);
        let mut nodes = Vec::new();
        for param in params {
            nodes.push(self.parse_expr(param)?);
        }
        Ok((Line::FCall(
            Node::FCall(
                name, 
                nodes
           )
        ),rem))
    }

    pub(crate) fn parse_overwrite(&self, tokens : Vec<Token>) -> Result<(Line, Vec<Token>),ParseError> {
        let grabber = Grabber{};
        let (node, rem ) = grabber.grab_line(tokens);
        let name = self.extrct_str(node.first().unwrap().clone())?;
        let expr = self.parse_expr(node[2..].to_vec())?;
        Ok((Line::OverVar(name, expr),rem))
    }

}

#[cfg(test)] 
mod tests {

    use super::*;
    #[test]
    fn test_parse_val() -> Result<(),ParseError> {
        let parser = Parser{};
        let token = Token::Value("90".into());
        let z = parser.parse_val(token)?;
        assert_eq!(z,Node::Leaf("90".into()));
        Ok(())
    }

    #[test]
    fn test_parse_print() -> Result<(), ParseError>{
        let parser = Parser{};
        let tokens = vec![
            Token::Print,
            Token::Value("9".into()),
            Token::Plus,
            Token::Value("9".into()),
            Token::SemiColan
        ];
        let (p,_) = parser.parse_print(tokens)?;
        assert_eq!(p, 
            Line::Print(
                Node::Add(
                    Box::new(Node::Leaf("9".into())),
                    Box::new(Node::Leaf("9".into()))
                ))
        );
        Ok(())
    }

    #[test] 
    fn test_for() -> Result<(),ParseError>{
        let parser = Parser{};
        let tokens = vec![
            Token::For,
            Token::Value("i".into()),
            Token::Comma,
            Token::Value("0".into()),
            Token::Comma,
            Token::Value("15".into()),
            Token::LCur,
                Token::Print,
                Token::Value("100".into()),
                Token::SemiColan,
            Token::RCur
        ];
        let (line,_) = parser.parse_for(tokens)?;
        assert_eq!(line,
            Line::For(
                "i".into(), 
                Node::Leaf("0".into()), 
                Node::Leaf("15".into()), 
                vec![
                    Line::Print(
                        Node::Leaf("100".into())
                    )
                ]
            )
        );
        Ok(())
    }

    #[test] 
    fn test_parse_expr() -> Result<(),ParseError>{
        let parser = Parser{};
        let tokens = 
        vec![
            Token::Value("9".into()),
            Token::Plus,
            Token::Value("10".into()),
            Token::Mul,
            Token::Value("2".into()),
            Token::Div,
            Token::Value("12".into()),
            Token::Plus,
            Token::Value("43".into())
        ];
        let res = parser.parse_expr(tokens)?;
        assert_eq!(res ,
        Node::Add(
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
        ));
        Ok(())
    }
    #[test]
    fn test_stuff() -> Result<(),ParseError> {
        let mut tokenizer = Tokenizer::new("Print 100 + 10 * 3 / 6 + 10;
        ");
        tokenizer.tokenize();
        let parser = Parser{};
        let (line, _) = parser.parse_line(tokenizer.tokens)?;
        assert_eq!(
            line,
            Line::Print(
                Node::Add(
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
        );
        Ok(())
    }

}
