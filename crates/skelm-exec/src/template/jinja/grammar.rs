// auto-generated: "lalrpop 0.22.2"
// sha3: 888ddd10142d15a5a0d03751694de74d817f2171ad7195e26a74f88d2e4a3636
use super::ast::{ArraySubexpr, Expression, Operator};
use super::lexer::StatementToken as Token;
use super::lexer::StatementToken;
use super::parser::Statement;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Statement {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    #[allow(unused_extern_crates)]
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(String),
        Variant1(Token),
        Variant2((String, Expression)),
        Variant3(alloc::vec::Vec<(String, Expression)>),
        Variant4(ArraySubexpr),
        Variant5(bool),
        Variant6(Vec<(String, Expression)>),
        Variant7(Expression),
        Variant8(Option<(String, Expression)>),
        Variant9(Statement),
        Variant10(Option<String>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 28, 3, 0, 5, 29, 30, 0, 0, 0, 0,
        // State 1
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 2
        50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 4
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 5
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 6
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 8
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 9
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 10
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 11
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 12
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 13
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 14
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 15
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 16
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 17
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 18
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 19
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 20
        73, 48, 47, 46, 6, 0, -15, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 21
        45, 48, 47, 78, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 22
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 23
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 24
        73, 48, 47, 46, 6, 0, -17, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 25
        45, 48, 47, 46, 6, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, -22, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 33
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, -24, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 34
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 9,
        // State 35
        0, 0, 0, 0, 0, 0, -33, -33, -33, 10, 11, -33, -33, -33, -33, -33, -33, 0, -33, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 12, 13, 14, 15, -37, -37, 0, -37, -37, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, -37, -37, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -39, -39, 19, 0, 0, 0, 0, 0, 0, 18, 17, 0, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, -19, 0, 0,
        // State 39
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 40
        0, 0, 0, 0, 21, 22, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 53, -20, -20, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 41
        0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 43
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 44
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 45
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 46
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 47
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 52
        74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 54
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 55
        0, 0, 0, 0, 21, 22, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 53, -21, -21, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 56
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, -18, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 58
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, -25, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 9,
        // State 59
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, -26, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 9,
        // State 60
        0, 0, 0, 0, 0, 0, -29, -29, -29, 10, 11, -29, -29, -29, -29, -29, -29, 0, -29, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, -30, -30, -30, 10, 11, -30, -30, -30, -30, -30, -30, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, -31, -31, -31, 10, 11, -31, -31, -31, -31, -31, -31, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -32, -32, -32, 10, 11, -32, -32, -32, -32, -32, -32, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -28, -28, -28, 10, 11, -28, -28, -28, -28, -28, -28, 0, -28, -28, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 12, 13, 14, 15, -35, -35, 0, -35, -35, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, -35, -35, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 12, 13, 14, 15, -34, -34, 0, -34, -34, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, -34, -34, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 12, 13, 14, 15, -36, -36, 0, -36, -36, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, -36, -36, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -38, -38, 19, 0, 0, 0, 0, 0, 0, 18, 17, 0, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 26, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 73
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 76
        0, 0, 0, 86, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 77
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 87, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 78
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 79
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 80
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 81
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 82
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 83
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 84
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 85
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 86
        0, 0, 0, 90, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 87
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 88
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 89
        0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -62,
        // State 27
        -54,
        // State 28
        -52,
        // State 29
        -51,
        // State 30
        -44,
        // State 31
        -53,
        // State 32
        -22,
        // State 33
        -24,
        // State 34
        -27,
        // State 35
        -33,
        // State 36
        -37,
        // State 37
        -39,
        // State 38
        -19,
        // State 39
        -56,
        // State 40
        -20,
        // State 41
        0,
        // State 42
        -13,
        // State 43
        -12,
        // State 44
        -57,
        // State 45
        -42,
        // State 46
        -41,
        // State 47
        -40,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -49,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        -43,
        // State 55
        -21,
        // State 56
        -18,
        // State 57
        -23,
        // State 58
        -25,
        // State 59
        -26,
        // State 60
        -29,
        // State 61
        -30,
        // State 62
        -31,
        // State 63
        -32,
        // State 64
        -28,
        // State 65
        -35,
        // State 66
        -34,
        // State 67
        -36,
        // State 68
        -38,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        -59,
        // State 74
        0,
        // State 75
        0,
        // State 76
        0,
        // State 77
        0,
        // State 78
        -58,
        // State 79
        -50,
        // State 80
        -55,
        // State 81
        0,
        // State 82
        -61,
        // State 83
        0,
        // State 84
        -60,
        // State 85
        0,
        // State 86
        0,
        // State 87
        0,
        // State 88
        0,
        // State 89
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 24,
            3 => 74,
            4 => 48,
            5 => 30,
            6 => 69,
            7 => match state {
                1 => 31,
                3 => 50,
                4 => 51,
                5 => 53,
                21 => 75,
                22 => 79,
                23 => 80,
                25 => 88,
                _ => 70,
            },
            8 => 32,
            9 => match state {
                8 => 57,
                _ => 33,
            },
            10 => match state {
                9 => 58,
                10 => 59,
                _ => 34,
            },
            11 => match state {
                11 => 60,
                12 => 61,
                13 => 62,
                14 => 63,
                15 => 64,
                _ => 35,
            },
            12 => match state {
                16 => 65,
                17 => 66,
                18 => 67,
                _ => 36,
            },
            13 => match state {
                19 => 68,
                _ => 37,
            },
            14 => match state {
                7 => 56,
                _ => 38,
            },
            15 => 39,
            16 => match state {
                24 => 81,
                _ => 71,
            },
            18 => 26,
            19 => match state {
                6 => 55,
                _ => 40,
            },
            _ => 0,
        }
    }
    #[allow(clippy::needless_raw_string_hashes)]
    const __TERMINAL: &[&str] = &[
        r###"identifier"###,
        r###"string"###,
        r###"sstring"###,
        r###"number"###,
        r###""(""###,
        r###""[""###,
        r###"")""###,
        r###""]""###,
        r###""|""###,
        r###""+""###,
        r###""-""###,
        r###""<""###,
        r###""<=""###,
        r###"">""###,
        r###"">=""###,
        r###""==""###,
        r###""!=""###,
        r###"".""###,
        r###"",""###,
        r###""=""###,
        r###""::""###,
        r###""true""###,
        r###""false""###,
        r###""elif""###,
        r###""if""###,
        r###""else""###,
        r###""for""###,
        r###""in""###,
        r###""set""###,
        r###""endfor""###,
        r###""endif""###,
        r###""and""###,
        r###""or""###,
        r###""not""###,
        r###""is""###,
    ];
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    fn __expected_tokens_from_states<
    >(
        __states: &[i8],
        _: core::marker::PhantomData<()>,
    ) -> alloc::vec::Vec<alloc::string::String>
    {
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            if __accepts(None, __states, Some(index), core::marker::PhantomData::<()>) {
                Some(alloc::string::ToString::to_string(terminal))
            } else {
                None
            }
        }).collect()
    }
    struct __StateMachine<>
    where
    {
        __phantom: core::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where
    {
        type Location = ();
        type Error = &'static str;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Statement;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 35 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        fn expected_tokens_from_states(&self, states: &[i8]) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens_from_states(states, core::marker::PhantomData::<()>)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, core::marker::PhantomData::<()>)
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: core::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        #[warn(unused_variables)]
        match __token {
            StatementToken::Ident(_) if true => Some(0),
            StatementToken::String(_) if true => Some(1),
            StatementToken::SString(_) if true => Some(2),
            StatementToken::Number(_) if true => Some(3),
            StatementToken::ParenOpen if true => Some(4),
            StatementToken::BrackOpen if true => Some(5),
            StatementToken::ParenClose if true => Some(6),
            StatementToken::BrackClose if true => Some(7),
            StatementToken::Pipe if true => Some(8),
            StatementToken::Plus if true => Some(9),
            StatementToken::Minus if true => Some(10),
            StatementToken::Less if true => Some(11),
            StatementToken::LessEqual if true => Some(12),
            StatementToken::Greater if true => Some(13),
            StatementToken::GreaterEqual if true => Some(14),
            StatementToken::Equal if true => Some(15),
            StatementToken::NotEqual if true => Some(16),
            StatementToken::Dot if true => Some(17),
            StatementToken::Comma if true => Some(18),
            StatementToken::Assign if true => Some(19),
            StatementToken::DDColon if true => Some(20),
            StatementToken::True if true => Some(21),
            StatementToken::False if true => Some(22),
            StatementToken::Elif if true => Some(23),
            StatementToken::If if true => Some(24),
            StatementToken::Else if true => Some(25),
            StatementToken::For if true => Some(26),
            StatementToken::In if true => Some(27),
            StatementToken::Set if true => Some(28),
            StatementToken::EndFor if true => Some(29),
            StatementToken::EndIf if true => Some(30),
            StatementToken::And if true => Some(31),
            StatementToken::Or if true => Some(32),
            StatementToken::Not if true => Some(33),
            StatementToken::Is if true => Some(34),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        #[allow(clippy::manual_range_patterns)]match __token_index {
            0 | 1 | 2 | 3 => match __token {
                StatementToken::Ident(__tok0) | StatementToken::String(__tok0) | StatementToken::SString(__tok0) | StatementToken::Number(__tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 => __Symbol::Variant1(__token),
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
    >(
        __reduce_index: i8,
        _: core::marker::PhantomData<()>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 3,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 6,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 6,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 7,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 9,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 10,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 11,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 12,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 14,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            40 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            41 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            42 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 15,
                }
            }
            43 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            44 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 16,
                }
            }
            45 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            46 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 17,
                }
            }
            47 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 17,
                }
            }
            48 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            49 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            50 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            51 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            52 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 18,
                }
            }
            53 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 18,
                }
            }
            54 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 18,
                }
            }
            55 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            56 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 19,
                }
            }
            57 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            58 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 19,
                }
            }
            59 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 19,
                }
            }
            60 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 19,
                }
            }
            61 => __state_machine::SimulatedReduce::Accept,
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 21,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct StatementParser {
        _priv: (),
    }

    impl Default for StatementParser { fn default() -> Self { Self::new() } }
    impl StatementParser {
        pub fn new() -> StatementParser {
            StatementParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<Statement, __lalrpop_util::ParseError<(), Token, &'static str>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
    >(
        __error_state: Option<i8>,
        __states: &[i8],
        __opt_integer: Option<usize>,
        _: core::marker::PhantomData<()>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.extend(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), core::marker::PhantomData::<()>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&()>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Statement,__lalrpop_util::ParseError<(), Token, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            61 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {__action}")
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), (String, Expression), ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), ArraySubexpr, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Expression, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Option<(String, Expression)>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Option<String>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Statement, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), String, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Vec<(String, Expression)>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), alloc::vec::Vec<(String, Expression)>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), bool, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(55);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action55::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(53);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action53::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(54);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(58);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action58::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(59);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action59::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(66);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(67);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action67::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(68);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action68::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(69);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action69::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(46);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(47);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action47::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(62);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(63);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action63::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(64);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action64::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(65);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action65::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(9);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action9::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action10::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(11);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(13);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action13::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(16);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(21);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(22);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action22::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(26);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action26::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(28);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action28::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action41::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(42);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action42::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(43);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action43::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(44);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action44::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(45);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action45::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action39::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(40);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(51);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(52);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action52::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(1);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action1::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(2);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action2::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(5);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action5::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action7::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(31);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action31::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action32::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(34);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action34::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(35);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(36);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action36::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(48);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(49);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action49::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 21)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Statement::StatementParser;

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action0((_, __0, _): ((), Statement, ())) -> Statement {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action1((_, _, _): ((), Token, ()), (_, condition, _): ((), Expression, ())) -> Statement {
    Statement::If { condition }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action2(
    (_, _, _): ((), Token, ()),
    (_, bind, _): ((), String, ()),
    (_, _, _): ((), Token, ()),
    (_, expression, _): ((), Expression, ()),
) -> Statement {
    Statement::For {
        bind,
        iter: expression,
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action3((_, __0, _): ((), Token, ())) -> Statement {
    Statement::EndIf
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action4((_, __0, _): ((), Token, ())) -> Statement {
    Statement::EndFor
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action5((_, _, _): ((), Token, ()), (_, condition, _): ((), Expression, ())) -> Statement {
    Statement::Elif { condition }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action6((_, __0, _): ((), Token, ())) -> Statement {
    Statement::Else
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action7(
    (_, _, _): ((), Token, ()),
    (_, expr, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, value, _): ((), Expression, ()),
) -> Statement {
    Statement::Set { expr, value }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action8((_, __0, _): ((), String, ())) -> String {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action9((_, e, _): ((), Expression, ())) -> Expression {
    e
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action10((_, _, _): ((), Token, ()), (_, t, _): ((), Expression, ())) -> Expression {
    Expression::LogicalNot(Box::new(t))
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action11((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action12(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::Is,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action13((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action14(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::Add,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action15(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::Sub,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action16((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action17(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::In,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action18(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::LesserThan,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action19(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::LesserEqual,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action20(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::GreaterThan,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action21(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::GreaterEqual,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action22((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action23(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::Equal,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action24(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    {
        Expression::BinaryOperation {
            lhs: Box::new(lhs),
            op: Operator::NotEqual,
            rhs: Box::new(rhs),
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action25(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    { Expression::Filter(Box::new(lhs), Box::new(rhs)) }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action26((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action27(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    { Expression::LogicalAnd(Box::new(lhs), Box::new(rhs)) }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action28((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action29(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, rhs, _): ((), Expression, ()),
) -> Expression {
    { Expression::LogicalOr(Box::new(lhs), Box::new(rhs)) }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action30((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action31((_, __0, _): ((), Expression, ())) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action32((_, ident, _): ((), String, ())) -> Expression {
    Expression::Identifier(ident)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action33(
    (_, _, _): ((), Token, ()),
    (_, __0, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
) -> Expression {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action34(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, ident, _): ((), String, ()),
) -> Expression {
    Expression::MemberAccess(Box::new(lhs), ident)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action35(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, sub, _): ((), ArraySubexpr, ()),
    (_, _, _): ((), Token, ()),
) -> Expression {
    Expression::ArraySubscript(Box::new(lhs), sub)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action36(
    (_, lhs, _): ((), Expression, ()),
    (_, _, _): ((), Token, ()),
    (_, params, _): ((), Vec<(String, Expression)>, ()),
    (_, _, _): ((), Token, ()),
) -> Expression {
    Expression::Call(Box::new(lhs), params)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action37(
    (_, nstart, _): ((), Option<String>, ()),
    (_, _, _): ((), Token, ()),
    (_, nend, _): ((), Option<String>, ()),
) -> ArraySubexpr {
    ArraySubexpr::Range(nstart, nend)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action38((_, expr, _): ((), Expression, ())) -> ArraySubexpr {
    ArraySubexpr::Value(Box::new(expr))
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action39(
    (_, ident, _): ((), String, ()),
    (_, _, _): ((), Token, ()),
    (_, e, _): ((), Expression, ()),
) -> (String, Expression) {
    (ident, e)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action40((_, e, _): ((), Expression, ())) -> (String, Expression) {
    (String::new(), e)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action41((_, s, _): ((), String, ())) -> Expression {
    Expression::String(s)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action42((_, s, _): ((), String, ())) -> Expression {
    Expression::String(s)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action43((_, n, _): ((), String, ())) -> Expression {
    Expression::Number(n)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action44((_, _, _): ((), Token, ()), (_, n, _): ((), String, ())) -> Expression {
    Expression::Number(n)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action45((_, b, _): ((), bool, ())) -> Expression {
    Expression::Boolean(b)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action46((_, __0, _): ((), Token, ())) -> bool {
    true
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action47((_, __0, _): ((), Token, ())) -> bool {
    false
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action48((_, __0, _): ((), String, ())) -> Option<String> {
    Some(__0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action49(__lookbehind: &(), __lookahead: &()) -> Option<String> {
    None
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action50(
    (_, mut v, _): ((), alloc::vec::Vec<(String, Expression)>, ()),
    (_, e, _): ((), Option<(String, Expression)>, ()),
) -> Vec<(String, Expression)> {
    match e {
        // (2)
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action51((_, __0, _): ((), (String, Expression), ())) -> Option<(String, Expression)> {
    Some(__0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action52(__lookbehind: &(), __lookahead: &()) -> Option<(String, Expression)> {
    None
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action53(__lookbehind: &(), __lookahead: &()) -> alloc::vec::Vec<(String, Expression)> {
    alloc::vec![]
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action54(
    (_, v, _): ((), alloc::vec::Vec<(String, Expression)>, ()),
) -> alloc::vec::Vec<(String, Expression)> {
    v
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action55(
    (_, __0, _): ((), (String, Expression), ()),
    (_, _, _): ((), Token, ()),
) -> (String, Expression) {
    __0
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action56(
    (_, __0, _): ((), (String, Expression), ()),
) -> alloc::vec::Vec<(String, Expression)> {
    alloc::vec![__0]
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action57(
    (_, v, _): ((), alloc::vec::Vec<(String, Expression)>, ()),
    (_, e, _): ((), (String, Expression), ()),
) -> alloc::vec::Vec<(String, Expression)> {
    {
        let mut v = v;
        v.push(e);
        v
    }
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action58(
    __0: ((), (String, Expression), ()),
    __1: ((), Token, ()),
) -> alloc::vec::Vec<(String, Expression)> {
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action55(__0, __1);
    let __temp0 = (__start0, __temp0, __end0);
    __action56(__temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action59(
    __0: ((), alloc::vec::Vec<(String, Expression)>, ()),
    __1: ((), (String, Expression), ()),
    __2: ((), Token, ()),
) -> alloc::vec::Vec<(String, Expression)> {
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action55(__1, __2);
    let __temp0 = (__start0, __temp0, __end0);
    __action57(__0, __temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action60(__0: ((), Option<(String, Expression)>, ())) -> Vec<(String, Expression)> {
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action53(&__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action50(__temp0, __0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action61(
    __0: ((), alloc::vec::Vec<(String, Expression)>, ()),
    __1: ((), Option<(String, Expression)>, ()),
) -> Vec<(String, Expression)> {
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action54(__0);
    let __temp0 = (__start0, __temp0, __end0);
    __action50(__temp0, __1)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action62(__0: ((), (String, Expression), ())) -> Vec<(String, Expression)> {
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action51(__0);
    let __temp0 = (__start0, __temp0, __end0);
    __action60(__temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action63(__lookbehind: &(), __lookahead: &()) -> Vec<(String, Expression)> {
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action52(&__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action60(__temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action64(
    __0: ((), alloc::vec::Vec<(String, Expression)>, ()),
    __1: ((), (String, Expression), ()),
) -> Vec<(String, Expression)> {
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action51(__1);
    let __temp0 = (__start0, __temp0, __end0);
    __action61(__0, __temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action65(__0: ((), alloc::vec::Vec<(String, Expression)>, ())) -> Vec<(String, Expression)> {
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action52(&__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    __action61(__0, __temp0)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action66(__0: ((), String, ()), __1: ((), Token, ()), __2: ((), String, ())) -> ArraySubexpr {
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __start1 = __2.0;
    let __end1 = __2.2;
    let __temp0 = __action48(__0);
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action48(__2);
    let __temp1 = (__start1, __temp1, __end1);
    __action37(__temp0, __1, __temp1)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action67(__0: ((), String, ()), __1: ((), Token, ())) -> ArraySubexpr {
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __start1 = __1.2;
    let __end1 = __1.2;
    let __temp0 = __action48(__0);
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action49(&__start1, &__end1);
    let __temp1 = (__start1, __temp1, __end1);
    __action37(__temp0, __1, __temp1)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action68(__0: ((), Token, ()), __1: ((), String, ())) -> ArraySubexpr {
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __start1 = __1.0;
    let __end1 = __1.2;
    let __temp0 = __action49(&__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action48(__1);
    let __temp1 = (__start1, __temp1, __end1);
    __action37(__temp0, __0, __temp1)
}

#[allow(
    clippy::too_many_arguments,
    clippy::needless_lifetimes,
    clippy::just_underscores_and_digits
)]
fn __action69(__0: ((), Token, ())) -> ArraySubexpr {
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __start1 = __0.2;
    let __end1 = __0.2;
    let __temp0 = __action49(&__start0, &__end0);
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action49(&__start1, &__end1);
    let __temp1 = (__start1, __temp1, __end1);
    __action37(__temp0, __0, __temp1)
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple {
    fn to_triple(
        self,
    ) -> Result<((), Token, ()), __lalrpop_util::ParseError<(), Token, &'static str>>;
}

impl __ToTriple for Token {
    fn to_triple(
        self,
    ) -> Result<((), Token, ()), __lalrpop_util::ParseError<(), Token, &'static str>> {
        Ok(((), self, ()))
    }
}
impl __ToTriple for Result<Token, &'static str> {
    fn to_triple(
        self,
    ) -> Result<((), Token, ()), __lalrpop_util::ParseError<(), Token, &'static str>> {
        match self {
            Ok(v) => Ok(((), v, ())),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
