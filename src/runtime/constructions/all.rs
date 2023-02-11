// штріхи? *тип*

use super::object::*;
use crate::pre::token::tokens::dynamic::*;
use crate::pre::token::tokens::stat::*;

use super::construction::Construction;

macro_rules! token {
    ( $token:expr, $opt:expr ) => {
        Expectation {
            opt: $opt,
            exp: &token::TokenExpectation { token: $token },
        }
    };
}
macro_rules! value {
    ( $opt:expr ) => {
        Expectation {
            opt: $opt,
            exp: &value::ValueExpectation {},
        }
    };
}
macro_rules! construction {
    ( $cns:expr, $opt:expr ) => {
        Expectation {
            opt: $opt,
            exp: $cns,
        }
    };
}
macro_rules! many_args {
    ( $cns:expr, $opt:expr ) => {
        Expectation {
            opt: $opt,
            exp: &args::ArgsExpectation { template: $cns },
        }
    };
}
macro_rules! call_stack {
    ( $opt:expr ) => {
        Expectation {
            opt: $opt,
            exp: &callstack::CallStackExpectation {},
        }
    };
}

pub const TYPE_SET: &Construction = &Construction {
    expectations: &[token!(ARRAY, true), token!(NAME, false)],
};
//тіп *TYPE_SET* *name* це *value*
pub const DEC_VAR: &Construction = &Construction {
    expectations: &[
        token!(VAR, false),
        construction!(TYPE_SET, false),
        token!(NAME, false),
        token!(SET, false),
        value!(false),
    ],
};
//. *name*([*arg_name*])
pub const CALL_FUNC: &Construction = &Construction {
    expectations: &[
        token!(CALL, false),
        token!(NAME, false),
        many_args!(
            &Construction {
                expectations: &[value!(true)]
            },
            false
        ),
    ],
};

pub const RETURN_TYPE_SET: &Construction = &Construction {
    expectations: &[token!(RET_RYPE, false), token!(NAME, false)],
};

pub const DEF_FUNC: &Construction = &Construction {
    expectations: &[
        token!(NAME, false),
        many_args!(TYPE_SET, false),
        construction!(RETURN_TYPE_SET, true),
        call_stack!(false),
    ],
};

pub const STAT_ELSE: &Construction = &Construction {
    expectations: &[token!(ELSE, false), call_stack!(false)],
};

pub const STAT_IF: &Construction = &Construction {
    expectations: &[
        token!(IF, false),
        value!(false),
        call_stack!(false),
        construction!(STAT_ELSE, true),
    ],
};

pub const STAT_WHILE: &Construction = &Construction {
    expectations: &[
        token!(WHILE, false),
        value!(false),
        call_stack!(false),
        construction!(STAT_ELSE, true),
    ],
};
