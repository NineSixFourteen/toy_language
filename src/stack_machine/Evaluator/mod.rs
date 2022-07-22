mod eval_bin_op;
mod eval_func;
mod eval_jmp;

use std::collections::HashMap;
use super::*;



pub(crate) struct Evaluator {
    pub vars : HashMap<String,Value>,
    pub stack : Vec<Value>,
    pub point : usize,
    pub main : Function,
    pub funcs : HashMap<String, Function>
}

impl Evaluator {
     
    pub fn new () -> Evaluator{
        Evaluator {
            stack : Vec::new(),
            vars  : HashMap::new(),
            point : 0,
            main  : Function::new(Vec::new(), Vec::new()),
            funcs : HashMap::new()
        }
    }
    
    pub(crate) fn new_e(commands : Vec<Command>) -> Evaluator{
        Evaluator {
            stack : Vec::new(),
            vars  : HashMap::new(),
            point : 0,
            main  : Function { params: vec![], body: commands },
            funcs : HashMap::new()
        }
    }
    
    fn next_cmd(&mut self) {
        self.point += 1;
    }

    fn commands(&self) -> &Vec<Command> {
        &self.main.body
    }

    fn get_cur(& mut self) -> Command {
        self.commands().get(self.point).unwrap_or(&Command::OCmd(OtherCmd::ThrowError(StrError::CommandOutOfBounds))).clone()
    }

    fn has_next(& mut self) -> bool {
        self.point < self.commands().len()
    }

    fn jmp_to(& mut self, x: usize) -> Result<(), StrError>{
        if x == 0 {
            self.point = 0 ; 
            return Err(StrError::GOTOZero);
        } else if x >= self.commands().len() {
            return Err(StrError::GOTOOutOfBounds)
        } 
        self.point = x - 1;
        Ok(())
    }

    fn jmp_to_cond(& mut self, x : usize, con : bool) -> Result<(), StrError>{
        if con {
            self.jmp_to(x)?;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, StrError> {
        let x = self.stack.pop().ok_or(StrError::NothingToPop);
        match &x {
            Ok(_x) => {}
            Err(_e) => {
                for val in &self.stack {
                    print!("<<<>{:?}",val);
                }
                println!("<<<>{:?}",self.commands().get(self.point));
            }   
        }
        x
    }

    pub(crate) fn eval(&mut self) -> Result<Value, StrError> {
        while self.has_next() {
            match self.get_cur() {
                Command::VCmd(x) => {
                    match x {
                        VarCmd::SetVar(name) => {
                            let res = self.pop()?;
                            self.vars.insert(name, res);
                        }
                        VarCmd::GetVar(name) => {
                            let res = self.vars.get(&name).ok_or(StrError::NoSuchVar)?;
                            self.stack.push(res.clone());
                        }
                        VarCmd::IncVar(name,val) => {
                            let res = self.vars.get(&name).ok_or(StrError::NoSuchVar)?;
                            match (res, val) {
                                (Value::Int(x), Value::Int(y)) => 
                                    _ = self.vars.insert(name, Value::Int(x + y)),
                                (Value::String(x), Value::String(y)) => 
                                    _ = self.vars.insert(name, Value::String(x.clone() + &y)),
                                _ => panic!()
                            }
                        },
                        VarCmd::DecVar(_name, _val) => {},
                    }
                }
                Command::BOp(x) => {
                    let _ = self.eval_bin_op(x)?;
                }
                Command::JCmd(x) => {let _ = self.eval_jmp(x)?;}
                Command::OCmd(x) => {
                    match x {
                        OtherCmd::Pop => {
                            let _ = self.pop()?;
                        }
                        OtherCmd::ThrowError(x) => return Err(x),
                        OtherCmd::Func(x) => {
                            let _ = self.eval_func(x)?;
                        }
                        OtherCmd::Push(x) => self.stack.push(x),
                        OtherCmd::Return => {
                            let res = self.pop()?;
                            return Ok(res);
                        }
                        OtherCmd::Print => {
                            let res = self.pop()?;
                            self.printValue(res)?;
                        },
                        OtherCmd::Not => {
                            let res = self.pop()?;
                            match res {
                                Value::Boolean(x) => self.stack.push(Value::Boolean(!x)),
                                _ => panic!()
                            }
                        }
                        OtherCmd::MakeArray(x) => {
                            let mut val = Vec::new();
                            for _ in 0..x {
                                val.push(self.pop()?);
                            }
                            self.stack.push(Value::Array(val))
                        }
                        OtherCmd::GetElem(x) => {
                            let val = self.pop()?;
                            if let Value::Array(z) = val  {
                                self.stack.push(z.get(x).unwrap().clone()); //add errors
                            } else {
                                panic!() //add error
                            }
                        }
                        OtherCmd::SetElem(_x) => {

                        }
                    }
                }
            } 
            self.next_cmd();
        }
        Ok(Value::Nothing)
    }

    fn printValue(&self, val : Value) -> Result<(), StrError>{
        match val {
            Value::Nothing => todo!(),
            Value::Int(x) => println!(">{}",x),
            Value::String(x) => println!(">{}",x),
            Value::Boolean(x) => println!(">{}",x),
            Value::Float(x) => println!(">{}",x),
            Value::Char(x) => println!(">{}",x),
            Value::Array(x) => {
                print!(">[");
                for y in x {
                    match y {
                        Value::Nothing => todo!(),
                        Value::Int(x) => print!("{}, ",x),
                        Value::String(x) => print!("{}, ",x),
                        Value::Boolean(x) => print!("{}, ",x),
                        Value::Float(x) => print!("{}, ",x),
                        Value::Char(x) => print!("{}, ",x),
                        Value::Array(x) => self.printValue(Value::Array(x))?,
                    }
                }
                println!("]")
            }
        }
        Ok(())
    }

   
    

}