use super::parser::AST;

fn var_num() -> f64 {
    return 22.0;
}
fn var_str() -> String {
    return "1488".to_string();
}

fn ariph(bin: AST) -> f64 {
    match bin {
        AST::Num(x) => x,
        AST::AriphExpression { left, op, right } => match op.as_str() {
            "+" => ariph(*left) + ariph(*right),
            "-" => ariph(*left) - ariph(*right),
            "*" => ariph(*left) * ariph(*right),
            "/" => ariph(*left) / ariph(*right),
            _ => panic!(),
        },
        AST::Var(n) => var_num(),
        _ => panic!(),
    }
}
fn string(str: AST) -> String {
    let (mut base, next) = match str {
        AST::Str { base, next } => (
            match *base {
                AST::Chars(n) => n,
                AST::Var(n) => var_str(),
                _ => panic!(),
            },
            next,
        ),
        _ => panic!(),
    };
    if *next == AST::Nothing {
        return base;
    }
    base.push_str(&string(*next));
    return base;
}

pub fn interpreter(ast: AST) {
    println!("{}", string(ast));
    // println!("{}", ariph(ast));
}
