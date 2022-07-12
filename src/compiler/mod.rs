use crate::stack_machine::Command;
use crate::parser::{Program, Function, Line};
mod cmpl_ln;

pub(crate) struct Compiler {
    pub commands : Vec<Command> 
}

impl Compiler {

    fn compile(&self, prog : Program){
        todo!()
    }

    fn compile_fn(&self, func : Function )  {
        todo!()
    }

    pub(crate) fn compile_lines(&mut self, lines: Vec<Line> ) {
        for line in lines {
            self.compile_ln(line);
        }
    }

    fn compile_ln(&mut self, line : Line) {
        match line {
            Line::Print(_) => self.compile_print(line),
            Line::InitVar(_, _, _) => self.complile_init_var(line),
            Line::OverVar(_, _) => self.compile_overwrite(line),
            Line::For(_, _, _, _) => self.compile_for(line),
            Line::If(_, _) => self.compile_if(line),
            Line::Return(_) => self.compile_return(line),
            Line::FCall(_) => todo!(),
        }
    }

}