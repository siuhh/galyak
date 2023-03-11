use super::types::{get_type_name, Type};

pub fn err_wrong_type(var_name: &String, expected_type: &Type, passed_type: &Type) -> String {
    return format!(
        "масть \"{}\" штріха \"{}\" не підходить, очікувалось: \"{}\"", 
        get_type_name(passed_type),
        var_name,
        get_type_name(expected_type)
    );
}
pub fn var_not_found(var_name: &String) -> String {
    return format!("штріх \"{}\" не прописаний", var_name);
}
pub fn var_already_exists(var_name: &String) -> String {
    return format!("штріх з ім'ям {} вже існує", var_name);
}