use std::collections::HashMap;

use crate::stack_machine::{Command, Evaluator::Evaluator,Function};
use crate::parser::{Program, Function as OtherFunction, Line, Primitive};
#[allow(dead_code)]
mod cmpl_ln;

pub(crate) struct Compiler {
    pub commands : Vec<Command>,
    pub vars : HashMap<String,Primitive>,
    pub funcs : HashMap<String, Vec<Primitive>>
}

impl Compiler {
    pub(crate) fn compile(prog : Program) -> Evaluator {
        match prog {
            Program{ main, methods } => {
                let l : HashMap<String,Vec<Primitive>> = get_func_desc(methods.clone()); 
                for (k, _v) in &l {
                    println!("{}",k)
                }
                let compiler = Compiler{commands : vec![], vars :  HashMap::new(), funcs : l};
                let z : HashMap<String, Function> = methods
                .iter()
                .map(|x| compiler.compile_fn(x.clone()))
                .collect();
                let (_ , main) = compiler.compile_fn(main);
                Evaluator{
                    vars: HashMap::new(),
                    stack: Vec::new(),
                    point: 0,
                    main,
                    funcs: z
                }
            }
        }
    }
    
    fn compile_fn(&self, func : OtherFunction ) -> (String,Function) {
        match func {
            OtherFunction{ name, ty: _,  body, params } => {
                let f : Function;
                let mut comp = Compiler{ commands: Vec::new(), vars: params.clone(),  funcs: self.funcs.clone()};
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

    pub(crate) fn compile_ln(&mut self, line : Line) {
        match line {
            Line::Print(_) => self.compile_print(line),
            Line::InitVar(_, _, _) => self.complile_init_var(line),
            Line::OverVar(_, _) => self.compile_overwrite(line),
            Line::For (_,_,_,_) => self.compile_for(line),
            Line::If(_, _) => self.compile_if(line),
            Line::Return(_) => self.compile_return(line),
            Line::FCall(_) => todo!(),
        }
    }

    
}

fn get_func_desc(funcs: Vec<OtherFunction>) -> HashMap<String, Vec<Primitive>> {
    let mut map = HashMap::new();
    for OtherFunction { name, ty, body: _ , params } in funcs {
        let mut val : Vec<Primitive> = params.values().cloned().collect();
        val.push(ty.clone());
        map.insert(name, val);
    }
    map
}