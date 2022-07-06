#[allow(non_snake_case)]
pub(crate) mod Evaluator;


#[derive(Clone,PartialEq, Eq)]
pub struct Function {
    params : Vec<String>,
    body : Vec<Command> // The commands for the function
}

impl Function {
    
    pub(crate)fn new( params : Vec<String> , body : Vec<Command>  ) -> Function {
        Function{
            params,
            body
        }
    }
    
}


#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum Value {
    Nothing,
    Int(i64),
    String(String)
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum StrError {
    NoSuchVar,
    CommandOutOfBounds,
    //ErrorBothSidesNotInt,
    NothingToPop,
    //ErrorOne,
    GOTOZero,
    GOTOOutOfBounds,
    OperandNotSupported
}

#[derive(Clone,PartialEq, Eq)]
pub(crate) enum Command {
    VCmd(VarCmd),
    BOp(BinOp),
    JCmd(JmpCmd),
    OCmd(OtherCmd)
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum VarCmd {
    SetVar(String),
    GetVar(String)
}

#[derive(Clone,PartialEq, Eq)]
pub(crate) enum BinOp {
    Add,
    Minus,
    Mul,
    Div
}

#[derive(Clone,PartialEq, Eq)]
pub(crate) enum JmpCmd {
    GOTO(usize),
    IFLT(usize),
    IFGT(usize),
    IFGEQ(usize),
    IFLEQ(usize),
    IFEQ(usize),
    IFNEQ(usize)
}

#[derive(Clone,PartialEq, Eq)]
pub(crate) enum OtherCmd {
    Pop, 
    ThrowError(StrError),
    Func(Function),
    Push(Value),
    Return 
}