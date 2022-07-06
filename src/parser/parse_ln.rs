use super::*;
impl Parser {

    pub(crate) fn parse_print(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        let y = Grabber{};
        let (line, rem) = y.grab_line(tokens);
        let expr = self.parse_expr(line[1..].to_vec());
        (Line::Print(
            Node::Add(
                Box::new(Node::Leaf("9".into())), 
                Box::new(Node::Leaf("9".into())))
            )
        , rem)
    }

     pub(crate)fn parse_for(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

     pub(crate)fn parse_if(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    pub(crate) fn parse_else(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    pub(crate)fn parse_return(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    pub(crate) fn parse_int(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    pub(crate) fn parse_string(&self, tokens: Vec<Token>) -> (Line, Vec<Token>) {
        todo!()
    }

    fn parse_expr(&self,tokens: Vec<Token>) -> Node {
        match tokens.get(1).unwrap() {
            Token::Plus => Node::Add(
                Box::new(self.parse_val(tokens.first().unwrap().clone())), 
                Box::new(self.parse_expr(tokens[2..].to_vec()))
            ),
            Token::Minus => Node::Sub(
                Box::new(self.parse_val(tokens.first().unwrap().clone())), 
                Box::new(self.parse_expr(tokens[2..].to_vec()))
            ),
            Token::Mul =>  Node::Mul(
                Box::new(self.parse_val(tokens.first().unwrap().clone())), 
                Box::new(self.parse_expr(tokens[2..].to_vec()))
            ),
            Token::Div => todo!(),
            Token::And => todo!(),
            Token::Or => todo!(),
            _ => panic!()
        }
    }

    fn parse_val(&self, t: Token ) -> Node {
        match t {
            Token::Value(x) => Node::Leaf(x),
            _ => panic!()
        }
    }


}