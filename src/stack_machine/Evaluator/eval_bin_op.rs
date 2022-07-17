use super::*;
impl Evaluator {

    pub(crate) fn eval_bin_op(&mut self, cmd:BinOp) -> Result<(),StrError>{
        let rhs = self.pop()?;
        let lhs = self.pop()?;
        match (lhs,rhs) {
            (Value::Int(x),Value::Int(y)) => self.eval_int(x,y,cmd)?,
            (Value::String(x), Value::String(y)) => self.eval_string(x,y,cmd)?,
            (Value::Float(x), Value::Float(y) ) => self.eval_float(x,y,cmd)?, 
            (Value::Boolean(x),Value::Boolean(y)) => self.eval_bool(x,y,cmd)?,
            _ => return Err(StrError::OperandNotSupported)
        };
        Ok(())
    }
    fn eval_float(&mut self, x: f32, y: f32, cmd: BinOp) -> Result<(),StrError>{
        match cmd {
            BinOp::Add   =>   self.stack.push(Value::Float(x + y)),
            BinOp::Minus =>   self.stack.push(Value::Float(x - y)),
            BinOp::Mul   =>   self.stack.push(Value::Float(x * y)),
            BinOp::Div   =>   self.stack.push(Value::Float(x / y)),
            BinOp::LT    =>   self.stack.push(Value::Boolean(x < y)),
            BinOp::GT    =>   self.stack.push(Value::Boolean(x < y)),
            BinOp::LTEQ  =>   self.stack.push(Value::Boolean(x <= y)),
            BinOp::GTEQ  =>   self.stack.push(Value::Boolean(x >= y)),
            BinOp::EQ    =>   self.stack.push(Value::Boolean(x == y)),
            BinOp::NEQ   =>   self.stack.push(Value::Boolean(x != y)),
            _ => return Err(StrError::OperandNotSupported)
        }
        Ok(())
    }

    fn eval_bool(&mut self, x: bool, y: bool, cmd: BinOp) -> Result<(),StrError> {
        match cmd {
            BinOp::BAnd => self.stack.push(Value::Boolean(x && y)),
            BinOp::BOr => self.stack.push(Value::Boolean(x || y)),
            _ => return Err(StrError::OperandNotSupported)
        }
        Ok(())
    }

    pub(crate) fn eval_int(&mut self,x:i64,y:i64, cmd:BinOp) -> Result<(),StrError>{
        match cmd {
            BinOp::Add   => self.stack.push(Value::Int(x + y)),
            BinOp::Minus => self.stack.push(Value::Int(x - y)),
            BinOp::Mul   => self.stack.push(Value::Int(x * y)),
            BinOp::Div   => self.stack.push(Value::Int(x / y)),
            BinOp::LT    => self.stack.push(Value::Boolean(x < y)),
            BinOp::GT    => self.stack.push(Value::Boolean(x > y)),
            BinOp::LTEQ  => self.stack.push(Value::Boolean(x <= y)),
            BinOp::GTEQ  => self.stack.push(Value::Boolean(x >= y)),
            BinOp::EQ    => self.stack.push(Value::Boolean(x == y)),
            BinOp::NEQ   => self.stack.push(Value::Boolean(x != y)),
            BinOp::And   => self.stack.push(Value::Int(x & y)),
            BinOp::Or    => self.stack.push(Value::Int(x | y)),
            _ => return Err(StrError::OperandNotSupported)
        }
        Ok(())
    }

    pub(crate) fn eval_string(&mut self,x:String,y:String,cmd:BinOp) -> Result<(),StrError>{
        match cmd {
            BinOp::Add => self.stack.push(Value::String(x + &y)),
            _ => return Err(StrError::OperandNotSupported)
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests{
use super::*;
#[test]
fn eval_test_plus() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::BOp(BinOp::Add),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(200));
    Ok(())
}

#[test]
fn eval_test_sub() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(100))),
        Command::OCmd(OtherCmd::Push(Value::Int(80))),
        Command::BOp(BinOp::Minus),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(20));
    Ok(())
}

#[test]
fn eval_test_mul() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(2))),
        Command::OCmd(OtherCmd::Push(Value::Int(23))),
        Command::BOp(BinOp::Mul),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(46));
    Ok(())
}

#[test]
fn eval_test_div() -> Result<(), StrError> {
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(88))),
        Command::OCmd(OtherCmd::Push(Value::Int(8))),
        Command::BOp(BinOp::Div),
        Command::OCmd(OtherCmd::Return)
        ];
    let mut ev = Evaluator::new_e(commands);
    let result = ev.eval()?;
    assert_eq!(result, Value::Int(11));
    Ok(())
}
}