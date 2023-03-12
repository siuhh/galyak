pub trait Error {
    
}
pub mod runtime {    
    use crate::runtime::memory::types::{get_type_name, Type};

    pub fn wrong_type(var_name: &String, expected_type: &Type, passed_type: &Type) -> String {
        return format!(
            "масть \"{}\" штріха \"{}\" не підходить, очікувалось: \"{}\"",
            get_type_name(passed_type),
            var_name,
            get_type_name(expected_type)
        );
    }
    
    pub fn wrong_return_type(var_name: &String, expected_type: &Type) -> String {
        return format!(
            "тємка \"{}\" рішає \"{}\"",
            var_name,
            get_type_name(expected_type)
        );
    }
    
    pub fn func_has_no_return(var_name: &String) -> String {
        return format!(
            "тємка \"{}\" нічого не рішає",
            var_name,
        );
    }
    
    pub fn unallowed_operation(op: String, vtype: &Type) -> String {
        return format!("не допустима тєма {} для масті {}", op,  get_type_name(vtype));
    }

    pub fn type_expected(vtype: &Type) -> String {
        return format!("тут має бути \"{}\"", get_type_name(vtype));
    }

    pub fn var_not_found(var_name: &String) -> String {
        return format!("штріх \"{}\" не прописаний", var_name);
    }

    pub fn var_already_exists(var_name: &String) -> String {
        return format!("штріх з ім'ям {} вже існує", var_name);
    }

    pub fn wrong_arguments_count(name: &String, expected: usize, passed: usize) -> String {
        return format!(
            "тємці \"{}\" {} {} {}, а тут {}",
            name, 
            if expected == 1 {
                "потрібен"
            }
            else {
                "потрібно"
            },
            expected, 
            if expected == 0 || expected >= 5 {
                "штріхів"
            }
            else if expected == 1 {
                "штріх"
            }
            else {
                "штріха"
            }, 
            passed
        );
    }
}
pub mod compilation {
    use crate::compiler::token::Token;

    pub fn unknown_token(t: &Token) -> String {
        let msg = format!("якийсь кучерявий базар \"{}\"", t.value);
        return msg;
    }

    pub fn unexpected_token(t: &Token) -> String {
        let msg = format!("\"{}\" - цього тіпа сюда ніхто не кликав", t.value);
        return msg;
    }

    pub fn unmatched_quote() -> String {
        return "не закрита \"".to_string();
    }

    pub fn inner_compilation_error() -> String {
        return "Якась залупа тут кароче закинь пж сюда https://github.com/siuhh/galyak/issues шо сталось".to_string();
    }
}
