use std::collections::HashMap;

use crate::stack_machine::{Command, Evaluator::Evaluator,Function};
use crate::parser::{Program, Function as OtherFunction, Line};
#[allow(dead_code)]
mod cmpl_ln;

pub(crate) struct Compiler {
    pub commands : Vec<Command> 
}

impl Compiler {

    pub(crate) fn compile(prog : Program) -> Evaluator {
        let compiler = Compiler{commands : vec![]};
        match prog {
            Program{ main, methods } => {
                let z : HashMap<String, Function> = methods
                .iter()
                .map(|x| compiler.compile_fn(x.clone()))
                .collect();
                let (_ , main) = compiler.compile_fn(main);
                Evaluator{
                    vars: HashMap::new(),
                    stack: Vec::new(),
                    point: 0,
                    main: main,
                    funcs: z
                }
            }
        }
    }
    
    fn compile_fn(&self, func : OtherFunction ) -> (String,Function) {
        match func {
            OtherFunction{ name, ty: _,  body, params } => {
                let f : Function;
                let mut comp = Compiler{ commands: Vec::new() };
                comp.compile_lines(body);
                f = Function::new(
                  params.iter().map(|(a,_b)| a.clone()).collect(), 
                    comp.commands
                );
                (name , f)
            }
        }
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