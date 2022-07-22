
use crate::{parser::{Line, Node, BoolNode, NodeTy, Primitive}, stack_machine::*};

use super::Compiler;

impl Compiler{

    pub(crate) fn complile_init_var(&mut self, line : Line)  {
        if let Line::InitVar(ty, name, val) = line {
            self.compile_expr(val,ty.clone());
            self.vars.insert(name.clone(), ty);
            self.commands.push(Command::VCmd(VarCmd::SetVar(name)));
        } else {
            unreachable!()
        }
    }


    pub(crate) fn compile_return(&mut self, line: Line) {
        if let Line::Return(val ) = line {
            self.compile_expr(val.clone(),self.clone().infer_type(val));
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
            self.compile_expr(val.clone(),self.clone().infer_type(val.clone())); 
            self.commands.push(Command::OCmd(OtherCmd::Print));
        } else {
            unreachable!()
        }
    }

    pub(crate) fn compile_overwrite(&mut self, line : Line) {
        if let Line::OverVar(x, y) = line {
        let z = self.vars.get(&x).unwrap();
            self.compile_expr(y, z.clone());
            self.commands.push(Command::VCmd(VarCmd::SetVar(x)));
        }   
    }


    fn compile_expr(&mut self, node : NodeTy, ty : Primitive)  {
        match node {
            NodeTy::Node(pol) => {
                match pol.clone() {
                    Node::Add(x, y) | 
                    Node::Mul(x, y) |
                    Node::Sub(x, y) |
                    Node::Div(x, y) 
                    => {
                        self.compile_expr(NodeTy::Node(*x),ty.clone());
                        self.compile_expr(NodeTy::Node(*y),ty);
                        match pol {
                            Node::Add(_, _) => self.commands.push(Command::BOp(BinOp::Add)),
                            Node::Sub(_, _) => self.commands.push(Command::BOp(BinOp::Minus)), 
                            Node::Mul(_, _) => self.commands.push(Command::BOp(BinOp::Mul)),
                            Node::Div(_, _) => self.commands.push(Command::BOp(BinOp::Div)),
                            _ => unreachable!(),
                        }
                    }
                    Node::Leaf(x) => {
                        if self.vars.contains_key(&x) {
                            self.commands.push(Command::VCmd(VarCmd::GetVar(x.clone())));
                            return ;
                        }
                        match ty {
                            Primitive::Int => {
                                let x : i64 = x.parse().unwrap();
                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::Int(x))))
                            }
                            Primitive::String => {
                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::String(x))))
                            },
                            Primitive::Boolean => match x.as_str() {
                                "true" => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(true)))),
                                "false" => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(false)))),
                                _ => panic!()
                            },
                            Primitive::Float => {
                                let x : f32 = x.parse().unwrap();
                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::Float(x))))
                            }
                            Primitive::Char => {
                                self.commands.push(Command::OCmd(OtherCmd::Push(Value::Char(x.chars().nth(1).unwrap()))))
                            }
                            Primitive::Array(x) => todo!(),
                        }
                    }
                    Node::FCall(x, nodes) => {
                        for (k,v) in &self.funcs{
                            print!("{} _ ",k);
                            for s in v {
                                print!("{:?}, ", s);
                            }
                            println!()
                        }
                        let tys  = self.funcs.get(&x).unwrap();
                        let types = &tys[..tys.len()-1].to_vec();
                        if types.len() != nodes.len() {
                            for node in &nodes {
                                println!("{:?}", node)
                            }
                            panic!("{} __ {} __ {}", types.len(), nodes.len(), x)
                        }
                        for num in 0..types.len() {
                            self.compile_expr(nodes.get(num).unwrap().clone(),types.get(num).unwrap().clone());
                        }
                        self.commands.push(Command::OCmd(OtherCmd::Func(x)));
                    }
                    Node::Nothing => panic!(),
                    Node::Array(x) => {
                        let size = x.len();
                        for y in x {
                            self.compile_expr(y.clone(), self.clone().infer_type(y.clone()));//todo stuff
                        }
                        self.commands.push(Command::OCmd(OtherCmd::MakeArray(size)));
                    }
                }
            }
            NodeTy::BoolNode(z) => {
                self.compile_bool(z);
            },
        }
    }

    pub(crate) fn compile_if(&mut self, line : Line) {
        if let Line::If(node, lines,elses) = line {
            if elses.len() == 0 {
                self.compile_bool(node);
                let pos = self.commands.len();
                self.commands.push(Command::JCmd(JmpCmd::IFFal(0)));
                self.compile_lines(lines);
                self.update_jmp(pos, self.commands.len());
            } else {
                let mut x = Vec::new(); // Locations of commands that jumps have to be updated
                x.push(self.compile_if_bool(node));
                for line in &elses {
                    if let Line::If(node,_  ,_ ) = line {
                        x.push(self.compile_if_bool(node.clone()));
                    } else {
                        panic!()
                    }
                }
                let mut y = Vec::new();
                y.push(self.commands.len());
                self.compile_lines(lines);
                let mut z = Vec::new();
                z.push(self.commands.len());
                self.commands.push(Command::JCmd(JmpCmd::GOTO(0)));
                for line in elses {
                    if let Line::If(_,lines  ,_ ) = line {
                        y.push(self.commands.len() );
                        self.compile_lines(lines);
                        z.push(self.commands.len());
                        self.commands.push(Command::JCmd(JmpCmd::GOTO(0)));
                    } else {
                        panic!()
                    }
                }
                self.commands.pop();
                for i in  0..y.len() {
                    self.update_jmp(x.get(i).unwrap().clone(), y.get(i).unwrap().clone())
                }
                for i in z[..z.len()-1].to_vec() {
                    self.update_jmp(i, self.commands.len());
                } 
            }
        }
    }

    fn compile_if_bool(&mut self, node : BoolNode) -> usize {
        self.compile_bool(node);
        self.commands.push(Command::JCmd(JmpCmd::IFTru(0)));
        return self.commands.len() - 1;
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
            Command::BOp(_)   |
            Command::VCmd(_)  |
            Command::OCmd(_) => {
                println!("{} __ {}",pos,loc);
                for com in &self.commands {
                    println!("{:?}", com);
                }
                panic!()}
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
                self.compile_expr(NodeTy::Node(x.clone()),self.clone().infer_type(NodeTy::Node(x.clone())));
                self.compile_expr(NodeTy::Node(y.clone()),self.clone().infer_type(NodeTy::Node(y.clone())));
                match &node {
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
                self.compile_bool(*x.clone());
                self.compile_bool(*y.clone());
                match node {
                    BoolNode::And(_,_) => self.commands.push(Command::BOp(BinOp::BAnd)),
                    BoolNode::Or(_,_) => self.commands.push(Command::BOp(BinOp::BOr)),
                    _ => panic!()
                }
            }
            BoolNode::Not(_) => todo!(),
            BoolNode::TFVar(x) => {
                match x.as_str() {
                    "true"  => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(true)))),
                    "false" => self.commands.push(Command::OCmd(OtherCmd::Push(Value::Boolean(false)))),
                    _ => {
                        if self.vars.contains_key(x) {
                            self.commands.push(Command::VCmd(VarCmd::GetVar(x.clone())))
                        } else {
                            panic!()
                        }
                    }
                    
                }
            }
        }
    }

    fn infer_type(self, node: NodeTy) -> Primitive {
        match node  {
            NodeTy::Node(x) => {
                match x {
                    Node::Add(x,y) |
                    Node::Sub(x, y) |
                    Node::Mul(x, y) |
                    Node::Div(x, y) => self.type_check(*x,*y),
                    Node::Leaf(x) => {
                        match x.parse::<i64>() {
                            Ok(_) => Primitive::Int,
                            Err(_) => {
                                match x.parse::<f32>() {
                                    Ok(_) => Primitive::Float,
                                    Err(_) => {
                                        if x.starts_with("\"") && x.ends_with("\""){
                                            Primitive::String
                                        } else {
                                            self.vars.get(&x).unwrap().clone()
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Node::FCall(x, _) => self.funcs.get(&x).unwrap().last().unwrap().clone(),
                    Node::Nothing => panic!(),
                    Node::Array(x) => Primitive::Array(Box::new(Primitive::Int))//TODO,
                }
            }
            NodeTy::BoolNode(_) => Primitive::Boolean,
        }
    }
    
    fn type_check(self, x : Node, y : Node) -> Primitive {
        let p = self.clone().infer_type(NodeTy::Node(x));
        let p2 = self.infer_type(NodeTy::Node(y));
        match (p,p2) {
            (Primitive::Int, Primitive::Int) => Primitive::Int,
            (Primitive::Int, Primitive::String) => Primitive::String, //TODO just strings in general
            (Primitive::Int, Primitive::Float) => Primitive::Float,
            (Primitive::String, Primitive::String)  | 
            (Primitive::String, Primitive::Boolean) | 
            (Primitive::String, Primitive::Float)   | 
            (Primitive::String, Primitive::Char)    =>  Primitive::String,
            (Primitive::Float, Primitive::Int) => Primitive::Float,
            (Primitive::Float, Primitive::Float) => Primitive::Float,
            (Primitive::Char, Primitive::Char) => Primitive::Char,
            _ => panic!()
        }
    }
    
}



