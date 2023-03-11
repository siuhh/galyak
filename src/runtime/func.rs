use std::collections::LinkedList;

use crate::{compiler::ast::{Ast, deref_ast}};

use super::{memory::{storage::{VarInfo}, types::{get_type, Type}}};

unsafe fn init_stack_res(call_stack: &LinkedList<Box<Ast>>) -> LinkedList<VarInfo> {
    let mut reserve = LinkedList::<VarInfo>::new();
    
    for cs in call_stack {
        let mut ast = deref_ast(cs);
        
        if let Ast::Statement { line: _, statement } = ast {
            ast = deref_ast(&statement);
        }
        
        match ast {
            Ast::DeclareVariable { array: _, name, vtype, value: _ } => {
                reserve.push_back(VarInfo { vtype: get_type(&vtype), name })
            }
            Ast::Function { name, args: _, return_type: _, compound_statement: _ } => {
                reserve.push_back(VarInfo { vtype: Type::Func, name })
            }
            _ => (),
        } 
    }
    
    return reserve;
}

pub struct GlkFuncDeclaration {
    pub call_stack: LinkedList<Box<Ast>>,
    pub stack_reservation: LinkedList<VarInfo>,
    pub args: LinkedList<VarInfo>,
    pub return_type: Type
}

impl GlkFuncDeclaration {
    pub unsafe fn new(call_stack: LinkedList<Box<Ast>>, args: LinkedList<VarInfo>, return_type: Type) -> Self {
        let mut stack_reservation = init_stack_res(&call_stack);
        
        //добавити на початок стеку аргументи
        for arg in &args {
            stack_reservation.push_back(arg.clone());
        }
        
        if return_type != Type::Null {
            //тут зберігається ретурн функції
            let result_value = VarInfo { name: "#".to_string(), vtype: return_type };
            stack_reservation.push_back(result_value);
        }
        
        return GlkFuncDeclaration { call_stack, stack_reservation, args, return_type };
    }
}