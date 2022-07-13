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

#[derive(Debug,Clone,PartialEq, Eq)]
pub(crate) enum Value {
    Nothing,
    Int(i64),
    String(String),
    Boolean(bool)
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
    NEQ
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
    Return 
}