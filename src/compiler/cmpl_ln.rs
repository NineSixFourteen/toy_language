use crate::{parser::{Line, Node, BoolNode}, stack_machine::*};

use super::Compiler;
struct num {
    l : usize
}
impl Compiler{

    pub(crate) fn complile_init_var(&mut self, line : Line)  {
        if let Line::InitVar(ty, name, val) = line {
            self.compile_expr(val);
            self.commands.push(Command::VCmd(VarCmd::SetVar(name)));
        } else {
            unreachable!()
        }
    }

    pub(crate) fn compile_return(&mut self, line: Line) {
        if let Line::Return(val ) = line {
            self.compile_expr(val);
            self.commands.push(Command::OCmd(OtherCmd::Return));
        } else {
            unreachable!()
        }
    }

    pub(crate) fn compile_for(&mut self, line: Line)  {
        if let Line::For(name, startVal, endVal, lines) = line { 
            self.compile_expr(startVal);
            self.commands.push(Command::VCmd(VarCmd::SetVar(name.clone())));
            let pos = self.commands.len() ;
            self.compile_lines(lines);
            self.commands.push(Command::VCmd(VarCmd::IncVar(name.clone(), Value::Int(1))));
            self.commands.push(Command::VCmd(VarCmd::GetVar(name)));
            self.compile_expr(endVal);
            self.commands.push(Command::BOp(BinOp::LT));
            self.commands.push(Command::JCmd(JmpCmd::IFTru(pos )));
       
        }

    }

    pub(crate) fn compile_print(&mut self, line: Line)  {
        if let Line::Print(val) = line {
            self.compile_expr(val);
            self.commands.push(Command::OCmd(OtherCmd::Print));
        } else {
            unreachable!()
        }
    }

    pub(crate) fn compile_overwrite(&mut self, line : Line) {
        if let Line::OverVar(x, y) = line {
            self.compile_expr(y);
            self.commands.push(Command::VCmd(VarCmd::SetVar(x)));
        }   
    }

    fn compile_expr(&mut self, node : Node)  {
        match node.clone() {
            Node::Add(x, y) | 
            Node::Mul(x, y) |
            Node::Sub(x, y) |
            Node::Div(x, y) 
            => {
                self.compile_expr(*x);
                self.compile_expr(*y);
                match node {
                    Node::Add(_, _) => self.commands.push(Command::BOp(BinOp::Add)),
                    Node::Sub(_, _) => self.commands.push(Command::BOp(BinOp::Minus)), 
                    Node::Mul(_, _) => self.commands.push(Command::BOp(BinOp::Mul)),
                    Node::Div(_, _) => self.commands.push(Command::BOp(BinOp::Div)),
                    _ => unreachable!(),
                }
            }
            Node::Leaf(x) => {
                match x.parse::<i64>() {
                    Ok(i) => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Int(i)))),
                    Err(_) => if x.starts_with("\"") && x.ends_with("\"") {
                      self.commands.push(Command::OCmd(OtherCmd::Push(Value::String(x)))); 
                    } else {
                      self.commands.push(Command::VCmd(VarCmd::GetVar(x)));
                    }
                }
            }
            Node::LoadVar(x) => self.commands.push(Command::VCmd(VarCmd::GetVar(x))),
            Node::FCall(_, _) => todo!(),
            Node::Nothing => panic!(),
        }
    }

    pub(crate) fn compile_if(&mut self, line : Line) {
        if let Line::If(node, lines) = line {
            self.compile_jmp(node);
            let pos = self.commands.len();
            self.commands.push(Command::JCmd(JmpCmd::IFFal(0)));
            self.compile_lines(lines);
            self.update_jmp(pos, self.commands.len());
        }
    }

    fn update_jmp(&mut self, pos : usize, loc : usize){
        let y = self.commands.get_mut(pos).unwrap() ;
        match y {
            Command::JCmd(z) => {
                match z {
                    JmpCmd::GOTO( x) => *y = Command::JCmd(JmpCmd::GOTO(loc)),
                    JmpCmd::IFTru(x) => *y = Command::JCmd(JmpCmd::IFTru(loc)),
                    JmpCmd::IFFal(x) => *y = Command::JCmd(JmpCmd::IFFal(loc))
                };
            }
            Command::BOp(_) |
            Command::VCmd(_) |
            Command::OCmd(_) => panic!()
        }
    }

    fn compile_jmp(&mut self, node: BoolNode) {
        match &node {
            BoolNode::LThan(  x, y) |
            BoolNode::GThan(  x, y) |
            BoolNode::GThanEq(x, y) | 
            BoolNode::LThanEq(x, y) |
            BoolNode::Eq(     x, y) |
            BoolNode::NEq(    x, y) => {
                self.compile_expr(x.clone());
                self.compile_expr(y.clone());
                match node {
                    BoolNode::LThan(_, _)   => self.commands.push(Command::BOp(BinOp::LT)),
                    BoolNode::GThan(_, _)   => self.commands.push(Command::BOp(BinOp::GT)),
                    BoolNode::GThanEq(_, _) => self.commands.push(Command::BOp(BinOp::GTEQ)),
                    BoolNode::LThanEq(_, _) => self.commands.push(Command::BOp(BinOp::LTEQ)),
                    BoolNode::Eq(_, _)      => self.commands.push(Command::BOp(BinOp::EQ)),
                    BoolNode::NEq(_, _)     => self.commands.push(Command::BOp(BinOp::NEQ)), 
                    _ => panic!(),
                }
            }
            BoolNode::And(x, y) |
            BoolNode::Or( x, y) => {
                todo!()
            }
            BoolNode::Not(_) => todo!(),
        }
    }

}