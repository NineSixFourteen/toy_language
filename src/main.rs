#[allow(dead_code)]
pub mod stack_machine;
#[allow(dead_code)]
pub mod parser;
pub mod tests;
pub mod compiler;
fn main() {
    let z = "900 500";
    let a = &z[..1];
    let b = &z[1..];
    println!("{} {}",a , b);
    /* 
    let x = "Print 100 + 10 ";
    let mut tk = parser::tokenizer::Tokenizer::new(x);
    tk.tokenize();
    let commands = vec![
        Command::OCmd(OtherCmd::Push(Value::Int(26))),
        Command::OCmd(OtherCmd::Push(Value::Int(25))),
        Command::JCmd(JmpCmd::IFEQ(5)),
        Command::OCmd(OtherCmd::Push(Value::Int(1))),
        Command::OCmd(OtherCmd::Return),
        Command::OCmd(OtherCmd::Push(Value::Int(5))),
        Command::OCmd(OtherCmd::Return)
        ];

    let mut ev = Evaluator::Evaluator::new_e(commands);
    let result = ev.eval();
    match result {
        Ok(Value::Int(x)) => {
            println!("got int {}", x);
        }
        _ => {
            println!("didn't get int")
        }
    }
    */
    
}