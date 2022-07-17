use crate::{parser::{Line, Node, BoolNode, NodeTy}, stack_machine::*};

use super::Compiler;

impl Compiler{

    pub(crate) fn complile_init_var(&mut self, line : Line)  {
        if let Line::InitVar(_ty, name, val) = line {
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
        if let Line::For(line, bool, otherline,lines ) = line { 
            self.compile_ln(*line);
            let pos = self.commands.len();
            self.compile_lines(lines);
            self.compile_ln(*otherline);
            self.compile_bool(bool);
            self.commands.push(Command::JCmd(JmpCmd::IFTru(pos)));
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

    fn compile_expr(&mut self, node : NodeTy)  {
        match node {
            NodeTy::Node(pol) => {
                match pol.clone() {
                    Node::Add(x, y) | 
                    Node::Mul(x, y) |
                    Node::Sub(x, y) |
                    Node::Div(x, y) 
                    => {
                        self.compile_expr(NodeTy::Node(*x));
                        self.compile_expr(NodeTy::Node(*y));
                        match pol {
                            Node::Add(_, _) => self.commands.push(Command::BOp(BinOp::Add)),
                            Node::Sub(_, _) => self.commands.push(Command::BOp(BinOp::Minus)), 
                            Node::Mul(_, _) => self.commands.push(Command::BOp(BinOp::Mul)),
                            Node::Div(_, _) => self.commands.push(Command::BOp(BinOp::Div)),
                            _ => unreachable!(),
                        }
                    }
                    Node::Leaf(x) => {
                        match x.as_str() {
                            "true"=> self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(true)))),
                            "false" => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(false)))),
                            _ => {
                                match x.parse::<i64>() {
                                    Ok(i) => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Int(i)))),
                                    Err(_) => match x.parse::<f32> () {
                                        Ok(i) => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Float(i)))),
                                        Err(_) => {
                                            if x.starts_with("\"") && x.ends_with("\"") {
                                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::String(x)))); 
                                            } else if x.starts_with("\'") && x.ends_with("\'") && x.len() == 3 {
                                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::Char(x.chars().nth(1).unwrap()))));
                                            } else {
                                                self.commands.push(Command::VCmd(VarCmd::GetVar(x)));
                                            } 
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Node::LoadVar(x) => self.commands.push(Command::VCmd(VarCmd::GetVar(x))),
                    Node::FCall(x, nodes) => {
                        for node in nodes {
                            self.compile_expr(node);
                        }
                        self.commands.push(Command::OCmd(OtherCmd::Func(x)));
                    }
                    Node::Nothing => panic!(),
                }
            }
            NodeTy::BoolNode(z) => match &z {
                BoolNode::LThan(x, y) |
                BoolNode::GThan(x, y) |
                BoolNode::GThanEq(x, y) |
                BoolNode::LThanEq(x, y) |
                BoolNode::Eq(x, y) |
                BoolNode::NEq(x, y) => {
                    self.compile_expr(NodeTy::Node(x.clone()));
                    self.compile_expr(NodeTy::Node(y.clone()));
                    let s = match z {
                        BoolNode::LThan(_, _)   => Command::BOp(BinOp::LT),
                        BoolNode::GThan(_, _)   => Command::BOp(BinOp::GT),
                        BoolNode::GThanEq(_, _) => Command::BOp(BinOp::GTEQ),
                        BoolNode::LThanEq(_, _) => Command::BOp(BinOp::LTEQ),
                        BoolNode::Eq(_, _)      => Command::BOp(BinOp::EQ),
                        BoolNode::NEq(_, _)     => Command::BOp(BinOp::NEQ),
                        _ => unreachable!(),
                    };
                    self.commands.push(s);
                }
                BoolNode::And(_x, _y) |
                BoolNode::Or(_x, _y) => {

                }
                BoolNode::Not(_) => todo!(),
            },
        }
    }

    pub(crate) fn compile_if(&mut self, line : Line) {
        if let Line::If(node, lines) = line {
            self.compile_bool(node);
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
                    JmpCmd::GOTO( _) => *y = Command::JCmd(JmpCmd::GOTO(loc)),
                    JmpCmd::IFTru(_) => *y = Command::JCmd(JmpCmd::IFTru(loc)),
                    JmpCmd::IFFal(_) => *y = Command::JCmd(JmpCmd::IFFal(loc))
                };
            }
            Command::BOp(_) |
            Command::VCmd(_) |
            Command::OCmd(_) => panic!()
        }
    }

    fn compile_bool(&mut self, node: BoolNode) {
        match &node {
            BoolNode::LThan(  x, y) |
            BoolNode::GThan(  x, y) |
            BoolNode::GThanEq(x, y) | 
            BoolNode::LThanEq(x, y) |
            BoolNode::Eq(     x, y) |
            BoolNode::NEq(    x, y) => {
                self.compile_expr(NodeTy::Node(x.clone()));
                self.compile_expr(NodeTy::Node(y.clone()));
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
            BoolNode::And(_x,_y) |
            BoolNode::Or( _x, _y) => {
                todo!()
            }
            BoolNode::Not(_) => todo!(),
        }
    }

}