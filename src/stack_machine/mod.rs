use core::f32;

#[allow(non_snake_case)]
pub(crate) mod Evaluator;


#[derive(Debug, Clone,PartialEq, Eq)]
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

#[derive(Debug,Clone)]
pub(crate) enum Value {
    Nothing,
    Int(i64),
    String(String),
    Boolean(bool),
    Float(f32),
    Char(char),
    Array(Vec<Value>)
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Value {
    fn assert_receiver_is_total_eq(&self) {}
}


#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum StrError {
    NoSuchVar,
    CommandOutOfBounds,
    NothingToPop,
    GOTOZero,
    GOTOOutOfBounds,
    OperandNotSupported
}

#[derive(Clone,PartialEq, Eq,Debug)]
pub(crate) enum Command {
    VCmd(VarCmd),
    BOp(BinOp),
    JCmd(JmpCmd),
    OCmd(OtherCmd)
}

#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum VarCmd {
    SetVar(String),
    GetVar(String),
    IncVar(String,Value),
    DecVar(String,Value)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BinOp {
    Add,
    Minus,
    Mul,
    Div,
    LT,
    GT,
    LTEQ,
    GTEQ,
    EQ,
    NEQ,
    And,
    Or,
    BAnd,
    BOr
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub(crate) enum JmpCmd {
    GOTO(usize),
    IFTru(usize),
    IFFal(usize)
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub(crate) enum OtherCmd {
    Pop, 
    Not,
    Print,
    ThrowError(StrError),
    Func(String),
    Push(Value),
    Return,
    MakeArray(usize),
    GetElem(usize),
    SetElem(usize),
}