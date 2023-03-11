use crate::runtime::func::GlkFuncDeclaration;

use super::{storage::GlkStack, types::Type, list::GlkList};

pub unsafe fn var_num(mem_stack: *mut GlkStack, name: &String) -> Result<f64, String> {
    let ptr = (*mem_stack).get_typed(name, &Type::Number, true);
    
    match ptr {
        Ok(value) => return Ok(*(value as *mut f64)),
        Err(err) => {
            return Err(err);
        },
    }
}

pub unsafe fn var_list_dyn(mem_stack: *mut GlkStack, name: &String) -> Result<*mut GlkList, String> {
    let res = (*mem_stack).get_typed(name, &Type::List, true);
    
    match res {
        Ok(value) => 
            return Ok(value as *mut GlkList),
        Err(err) => 
            return Err(err),
    }
}

pub unsafe fn var_fn(mem_stack: *mut GlkStack, name: &String) -> Result<*mut GlkFuncDeclaration, String> {
    let ptr = (*mem_stack).get_typed(name, &Type::Func, true);
    
    match ptr {
        Ok(value) => return Ok(value as *mut GlkFuncDeclaration),
        Err(err) => {
            return Err(err);
        },
    }
}

pub unsafe fn var_str(mem_stack: *mut GlkStack, name: &String) -> Result<String, String> {
    let ptr = (*mem_stack).get_typed(name, &Type::String, true);
    
    match ptr {
        Ok(value) => return Ok((*(value as *mut String)).clone()),
        Err(err) => {
            return Err(err);
        },
    }
}