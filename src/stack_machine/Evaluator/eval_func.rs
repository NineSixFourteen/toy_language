pub(crate) use crate::stack_machine::{Function, StrError};

use super::Evaluator;

impl Evaluator {
    pub(crate) fn eval_func(&mut self, func: Function ) -> Result<(), StrError> {
        match func {
            Function { params, body } => {
                let mut ev = Evaluator::new_e(body);
                for str in params {
                    let val = self.pop()?;
                    ev.vars.insert(str, val);
                }
                let res = ev.eval()?;
                self.stack.push(res);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
use crate::stack_machine::*;
use super::Evaluator;

#[test]
fn eval_test() -> Result<(),StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::VCmd(VarCmd::SetVar("a".into())),
        Command::VCmd(VarCmd::GetVar("a".into())),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(100));
    Ok(())
}

#[test]
fn eval_test_func() -> Result<(),StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::OCmd(
            OtherCmd::Func(
                Function::new(
                 vec!["a".into(),"b".into()] ,
                 vec![Command::VCmd(VarCmd::GetVar("a".into())),
                   Command::VCmd(VarCmd::GetVar("b".into())),
                   Command::BOp(BinOp::Add),
                   Command::OCmd(OtherCmd::Return)
            ]
        ))),
        Command::OCmd(OtherCmd::Return)
    ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(Value::Int(200), result);
    Ok(())
}
}