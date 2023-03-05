use super::token::Token;

pub fn err_unknown_token(t: &Token) -> String {
    let msg = format!("якийсь кучерявий базар \"{}\"", t.value);
    return msg;
}

pub fn err_unexpected_token(t: &Token) -> String {
    let msg = format!("\"{}\" - цього тіпа сюда ніхто не кликав", t.value);
    return msg;
}

pub fn err_unmatched_quote() -> String {
    return "не закрита \"".to_string();
}

pub fn err_inner_compilation_error() -> String {
    return "Якась залупа тут кароче закинь пж сюда https://github.com/siuhh/galyak/issues шо сталось".to_string();
}
