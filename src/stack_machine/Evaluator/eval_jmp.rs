
use super::*;

impl Evaluator {
    pub (crate) fn eval_jmp(&mut self, cmd:JmpCmd) -> Result<(),StrError>{
        match cmd {
            JmpCmd::GOTO(x) => {let _ = self.jmp_to(x)?;}
            JmpCmd::IFFal(x) |
            JmpCmd::IFTru(x) => {
                let res = self.pop()?;
                match (cmd,res) {
                    (JmpCmd::IFFal(x), Value::Boolean(false)) |
                    (JmpCmd::IFTru(x), Value::Boolean(true)) => {let _ = self.jmp_to(x);}
                    _ => ()
                }
            }
            _ => {
                self.eval_bin_jmp(cmd)?;
            }
        }
        Ok(())
    }

    fn eval_bin_jmp(&mut self, cmd:JmpCmd ) -> Result<(), StrError> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;
        match (lhs,rhs) {
            (Value::Int(x),Value::Int(y)) => {
                self.eval_int_jmp(x,y,cmd)?;
            }
            _ => return Err(StrError::OperandNotSupported)
        }
        Ok(())
    }
    fn eval_int_jmp(&mut self, x: i64, y: i64, cmd: JmpCmd) -> Result<(), StrError> {
        match cmd {
            JmpCmd::GOTO(_) => {},
            JmpCmd::IFTru(_) => todo!(),
            JmpCmd::IFFal(_) => todo!(),
        }
        Ok(())
    }
}
#[cfg(test)]

mod tests {
    
use super::*;

#[test]
fn eval_test_goto() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(22))), 
        Command::JCmd(JmpCmd::GOTO(3)),
        Command::BOp(BinOp::Div),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(22));
    Ok(())
}
#[test]
fn eval_test_lt_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(21))),
        Command::OCmd(OtherCmd::Push(Value::Int(22))),
        Command::BOp(BinOp::LT),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_lt_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::LT),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}
#[test]
fn eval_test_gt_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::GT),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_gt_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(22))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::GT),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}
#[test]
fn eval_test_lteq_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(22))),
        Command::OCmd(OtherCmd::Push(Value::Int(22))),
        Command::BOp(BinOp::LTEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_lteq_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::LTEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}
#[test]
fn eval_test_gteq_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::GTEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_gteq_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(28))),
        Command::BOp(BinOp::GTEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}

#[test]
fn eval_test_eq_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::EQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_eq_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::EQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];

    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}
#[test]
fn eval_test_neq_true() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::NEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(5));
    Ok(())
}

#[test]
fn eval_test_neq_false() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::BOp(BinOp::NEQ),
        Command::JCmd(JmpCmd::IFTru(6)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(1));
    Ok(())
}

}