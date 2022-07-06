mod eval_bin_op;
mod eval_func;
mod eval_jmp;

use std::collections::HashMap;
use super::*;

pub(crate) struct Evaluator {
    vars : HashMap<String,Value>,
    stack : Vec<Value>,
    point : usize,
    commands : Vec<Command>
}

impl Evaluator {
     
    pub fn new () -> Evaluator{
        Evaluator {
            stack : Vec::new(),
            vars : HashMap::new(),
            point : 0,
            commands : Vec::new()
        }
    }
    
    pub(crate) fn new_e(commands : Vec<Command>) -> Evaluator{
        Evaluator {
            stack : Vec::new(),
            vars : HashMap::new(),
            point : 0,
            commands
        }
    }
    
    fn next_cmd(&mut self) {
        self.point += 1;
    }

    fn get_cur(& mut self) -> Command {
        self.commands.get(self.point).unwrap_or(&Command::OCmd(OtherCmd::ThrowError(StrError::CommandOutOfBounds))).clone()
    }

    fn has_next(& mut self) -> bool {
        self.point < self.commands.len()
    }

    fn jmp_to(& mut self, x: usize) -> Result<(), StrError>{
        if x == 0 {
            self.point = 0 ; 
            return Err(StrError::GOTOZero);
        } else if x >= self.commands.len() {
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
        self.stack.pop().ok_or(StrError::NothingToPop)
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
                    }
                }
            } 
            self.next_cmd();
        }
        Ok(Value::Nothing)
    }

}