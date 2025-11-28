// auto-generated: "lalrpop 0.22.2"
// sha3: 946ee91f9c9b63d18a360292ae83b1f0796aff9baf507e5432d856796299ccdd
use super::parser::Statement;
use super::ast::{Expression, Operator, ArraySubexpr};
use super::lexer::StatementToken as Token;
use super::lexer::StatementToken;
use super::position::Position;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
#[allow(unused_extern_crates)]
extern crate alloc;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        59, 38, 37, 36, 2, 0, -15, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        35, 38, 37, 64, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        59, 38, 37, 36, 2, 0, -17, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        35, 38, 37, 36, 2, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 23
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 24
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 5,
        // State 25
        0, 0, 0, 0, 0, 0, -33, -33, -33, 6, 7, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 8, 9, 10, 11, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, -37, -37, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, -39, -39, 15, 0, 0, 0, 0, 0, 0, 14, 13, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, -19, 0, 0,
        // State 29
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 30
        0, 0, 0, 0, 17, 18, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 39, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 31
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 33
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 34
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 35
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 36
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 37
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 38
        60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 17, 18, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 39, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, -18, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 44
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 5,
        // State 45
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 5,
        // State 46
        0, 0, 0, 0, 0, 0, -29, -29, -29, 6, 7, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -30, -30, -30, 6, 7, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, -31, -31, -31, 6, 7, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -32, -32, -32, 6, 7, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, -28, -28, -28, 6, 7, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 8, 9, 10, 11, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, -35, -35, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 8, 9, 10, 11, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, -34, -34, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 8, 9, 10, 11, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, -36, -36, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, -38, -38, 15, 0, 0, 0, 0, 0, 0, 14, 13, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 59
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        // State 62
        0, 0, 0, 70, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 71, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 64
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 65
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 67
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 68
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 69
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        // State 73
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
        -44,
        // State 21
        -62,
        // State 22
        -22,
        // State 23
        -24,
        // State 24
        -27,
        // State 25
        -33,
        // State 26
        -37,
        // State 27
        -39,
        // State 28
        -19,
        // State 29
        -56,
        // State 30
        -20,
        // State 31
        0,
        // State 32
        -13,
        // State 33
        -12,
        // State 34
        -57,
        // State 35
        -42,
        // State 36
        -41,
        // State 37
        -40,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -18,
        // State 43
        -23,
        // State 44
        -25,
        // State 45
        -26,
        // State 46
        -29,
        // State 47
        -30,
        // State 48
        -31,
        // State 49
        -32,
        // State 50
        -28,
        // State 51
        -35,
        // State 52
        -34,
        // State 53
        -36,
        // State 54
        -38,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        -59,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -58,
        // State 65
        0,
        // State 66
        -61,
        // State 67
        0,
        // State 68
        -60,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 18,
            3 => 60,
            5 => 20,
            6 => 55,
            7 => match state {
                0 => 21,
                1 => 39,
                17 => 61,
                19 => 72,
                _ => 56,
            },
            8 => 22,
            9 => match state {
                4 => 43,
                _ => 23,
            },
            10 => match state {
                5 => 44,
                6 => 45,
                _ => 24,
            },
            11 => match state {
                7 => 46,
                8 => 47,
                9 => 48,
                10 => 49,
                11 => 50,
                _ => 25,
            },
            12 => match state {
                12 => 51,
                13 => 52,
                14 => 53,
                _ => 26,
            },
            13 => match state {
                15 => 54,
                _ => 27,
            },
            14 => match state {
                3 => 42,
                _ => 28,
            },
            15 => 29,
            16 => match state {
                18 => 65,
                _ => 57,
            },
            19 => match state {
                2 => 41,
                _ => 30,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct ExpressionParser {
        _priv: (),
    }

    impl Default for ExpressionParser { fn default() -> Self { Self::new() } }
    impl ExpressionParser {
        pub fn new() -> ExpressionParser {
            ExpressionParser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                // __Expression = Expression => ActionFn(8);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action8::<>(__sym0);
                return Some(Ok(__nt));
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression::ExpressionParser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression1 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 2
        45, 31, 30, 29, 2, 0, -15, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 3
        28, 31, 30, 50, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 4
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        45, 31, 30, 29, 2, 0, -17, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 6
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 7
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 8
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 9
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 10
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 11
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 12
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 13
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 14
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 15
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 16
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 17
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 18
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 19
        28, 31, 30, 29, 2, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 23
        0, 0, 0, 0, 3, 4, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 32, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 24
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 26
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 27
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 28
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 29
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 30
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 31
        46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 34
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 35
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 8,
        // State 36
        0, 0, 0, 0, 0, 0, -33, -33, -33, 9, 10, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 11, 12, 13, 14, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -37, -37, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 45
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 48
        0, 0, 0, 57, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 58, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 50
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 51
        0, 0, 0, 0, 3, 4, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 32, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 52
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 54
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 55
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 56
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 60
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 8,
        // State 61
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 8,
        // State 62
        0, 0, 0, 0, 0, 0, -29, -29, -29, 9, 10, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -30, -30, -30, 9, 10, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -31, -31, -31, 9, 10, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, -32, -32, -32, 9, 10, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -28, -28, -28, 9, 10, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 11, 12, 13, 14, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -35, -35, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 11, 12, 13, 14, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -34, -34, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 11, 12, 13, 14, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -36, -36, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 73
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
        -44,
        // State 21
        -63,
        // State 22
        -56,
        // State 23
        -20,
        // State 24
        0,
        // State 25
        -13,
        // State 26
        -12,
        // State 27
        -57,
        // State 28
        -42,
        // State 29
        -41,
        // State 30
        -40,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -59,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -58,
        // State 51
        0,
        // State 52
        0,
        // State 53
        -61,
        // State 54
        0,
        // State 55
        -60,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 5,
            3 => 46,
            5 => 20,
            6 => 41,
            7 => match state {
                1 => 32,
                3 => 47,
                19 => 72,
                _ => 42,
            },
            8 => match state {
                0 => 21,
                _ => 33,
            },
            9 => match state {
                7 => 59,
                _ => 34,
            },
            10 => match state {
                8 => 60,
                9 => 61,
                _ => 35,
            },
            11 => match state {
                10 => 62,
                11 => 63,
                12 => 64,
                13 => 65,
                14 => 66,
                _ => 36,
            },
            12 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                _ => 37,
            },
            13 => match state {
                18 => 70,
                _ => 38,
            },
            14 => match state {
                6 => 58,
                _ => 39,
            },
            15 => 22,
            16 => match state {
                5 => 52,
                _ => 43,
            },
            19 => match state {
                4 => 51,
                _ => 23,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => __state_machine::SimulatedReduce::Accept,
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression1Parser {
        _priv: (),
    }

    impl Default for Expression1Parser { fn default() -> Self { Self::new() } }
    impl Expression1Parser {
        pub fn new() -> Expression1Parser {
            Expression1Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                // __Expression1 = Expression1 => ActionFn(1);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression1::Expression1Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression2 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        46, 32, 31, 30, 2, 0, -15, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        29, 32, 31, 51, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        46, 32, 31, 30, 2, 0, -17, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        29, 32, 31, 30, 2, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 24
        0, 0, 0, 0, 4, 5, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 33, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 25
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 27
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 28
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 29
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 30
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 31
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 32
        47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 35
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 8,
        // State 36
        0, 0, 0, 0, 0, 0, -33, -33, -33, 9, 10, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 11, 12, 13, 14, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -37, -37, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 4, 5, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 33, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 46
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 49
        0, 0, 0, 57, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 58, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 51
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 52
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 54
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 55
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 56
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 60
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 8,
        // State 61
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 8,
        // State 62
        0, 0, 0, 0, 0, 0, -29, -29, -29, 9, 10, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -30, -30, -30, 9, 10, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -31, -31, -31, 9, 10, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, -32, -32, -32, 9, 10, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -28, -28, -28, 9, 10, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 11, 12, 13, 14, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -35, -35, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 11, 12, 13, 14, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -34, -34, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 11, 12, 13, 14, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -36, -36, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -64,
        // State 23
        -56,
        // State 24
        -20,
        // State 25
        0,
        // State 26
        -13,
        // State 27
        -12,
        // State 28
        -57,
        // State 29
        -42,
        // State 30
        -41,
        // State 31
        -40,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -59,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        -58,
        // State 52
        0,
        // State 53
        -61,
        // State 54
        0,
        // State 55
        -60,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 5,
            3 => 47,
            5 => 20,
            6 => 42,
            7 => match state {
                1 => 33,
                4 => 48,
                19 => 72,
                _ => 43,
            },
            8 => 21,
            9 => match state {
                0 => 22,
                7 => 59,
                _ => 34,
            },
            10 => match state {
                8 => 60,
                9 => 61,
                _ => 35,
            },
            11 => match state {
                10 => 62,
                11 => 63,
                12 => 64,
                13 => 65,
                14 => 66,
                _ => 36,
            },
            12 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                _ => 37,
            },
            13 => match state {
                18 => 70,
                _ => 38,
            },
            14 => match state {
                6 => 58,
                _ => 39,
            },
            15 => 23,
            16 => match state {
                5 => 52,
                _ => 44,
            },
            19 => match state {
                2 => 41,
                _ => 24,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => __state_machine::SimulatedReduce::Accept,
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression2Parser {
        _priv: (),
    }

    impl Default for Expression2Parser { fn default() -> Self { Self::new() } }
    impl Expression2Parser {
        pub fn new() -> Expression2Parser {
            Expression2Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                // __Expression2 = Expression2 => ActionFn(2);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression2::Expression2Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression4 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        47, 33, 32, 31, 2, 0, -15, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        30, 33, 32, 52, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        47, 33, 32, 31, 2, 0, -17, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        30, 33, 32, 31, 2, 0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        // State 24
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 25
        0, 0, 0, 0, 5, 6, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 34, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 26
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 28
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 29
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 30
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 31
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 32
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 33
        48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 4,
        // State 36
        0, 0, 0, 0, 0, 0, -33, -33, -33, 9, 10, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 11, 12, 13, 14, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -37, -37, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 5, 6, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 34, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 43
        0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 47
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 50
        0, 0, 0, 58, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 59, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 52
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 53
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 55
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 56
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 57
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 4,
        // State 61
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 4,
        // State 62
        0, 0, 0, 0, 0, 0, -29, -29, -29, 9, 10, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -30, -30, -30, 9, 10, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -31, -31, -31, 9, 10, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, -32, -32, -32, 9, 10, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -28, -28, -28, 9, 10, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 11, 12, 13, 14, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -35, -35, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 11, 12, 13, 14, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -34, -34, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 11, 12, 13, 14, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -36, -36, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -24,
        // State 23
        -65,
        // State 24
        -56,
        // State 25
        -20,
        // State 26
        0,
        // State 27
        -13,
        // State 28
        -12,
        // State 29
        -57,
        // State 30
        -42,
        // State 31
        -41,
        // State 32
        -40,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -23,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -59,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        -58,
        // State 53
        0,
        // State 54
        -61,
        // State 55
        0,
        // State 56
        -60,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 6,
            3 => 48,
            5 => 20,
            6 => 43,
            7 => match state {
                1 => 34,
                5 => 49,
                19 => 72,
                _ => 44,
            },
            8 => 21,
            9 => match state {
                3 => 42,
                _ => 22,
            },
            10 => match state {
                0 => 23,
                8 => 60,
                9 => 61,
                _ => 35,
            },
            11 => match state {
                10 => 62,
                11 => 63,
                12 => 64,
                13 => 65,
                14 => 66,
                _ => 36,
            },
            12 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                _ => 37,
            },
            13 => match state {
                18 => 70,
                _ => 38,
            },
            14 => match state {
                7 => 59,
                _ => 39,
            },
            15 => 24,
            16 => match state {
                6 => 53,
                _ => 45,
            },
            19 => match state {
                2 => 41,
                _ => 25,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => __state_machine::SimulatedReduce::Accept,
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression4Parser {
        _priv: (),
    }

    impl Default for Expression4Parser { fn default() -> Self { Self::new() } }
    impl Expression4Parser {
        pub fn new() -> Expression4Parser {
            Expression4Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                // __Expression4 = Expression4 => ActionFn(3);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression4::Expression4Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression6 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        49, 34, 33, 32, 2, 0, -15, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        31, 34, 33, 54, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        49, 34, 33, 32, 2, 0, -17, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        31, 34, 33, 32, 2, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 4,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 26
        0, 0, 0, 0, 7, 8, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 35, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 27
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 29
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 30
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 31
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 32
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 33
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 34
        50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, -33, -33, -33, 5, 6, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 11, 12, 13, 14, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -37, -37, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 7, 8, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 35, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 43
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 4,
        // State 44
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 4,
        // State 45
        0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 49
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0,
        // State 52
        0, 0, 0, 60, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 61, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 54
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 55
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 57
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 58
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 59
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, -29, -29, -29, 5, 6, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, -30, -30, -30, 5, 6, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, -31, -31, -31, 5, 6, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, -32, -32, -32, 5, 6, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -28, -28, -28, 5, 6, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 11, 12, 13, 14, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -35, -35, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 11, 12, 13, 14, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -34, -34, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 11, 12, 13, 14, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, -36, -36, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -24,
        // State 23
        -27,
        // State 24
        -66,
        // State 25
        -56,
        // State 26
        -20,
        // State 27
        0,
        // State 28
        -13,
        // State 29
        -12,
        // State 30
        -57,
        // State 31
        -42,
        // State 32
        -41,
        // State 33
        -40,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -23,
        // State 43
        -25,
        // State 44
        -26,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        -59,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        -58,
        // State 55
        0,
        // State 56
        -61,
        // State 57
        0,
        // State 58
        -60,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 8,
            3 => 50,
            5 => 20,
            6 => 45,
            7 => match state {
                1 => 35,
                7 => 51,
                19 => 72,
                _ => 46,
            },
            8 => 21,
            9 => match state {
                3 => 42,
                _ => 22,
            },
            10 => match state {
                4 => 43,
                5 => 44,
                _ => 23,
            },
            11 => match state {
                0 => 24,
                10 => 62,
                11 => 63,
                12 => 64,
                13 => 65,
                14 => 66,
                _ => 36,
            },
            12 => match state {
                15 => 67,
                16 => 68,
                17 => 69,
                _ => 37,
            },
            13 => match state {
                18 => 70,
                _ => 38,
            },
            14 => match state {
                9 => 61,
                _ => 39,
            },
            15 => 25,
            16 => match state {
                8 => 55,
                _ => 47,
            },
            19 => match state {
                2 => 41,
                _ => 26,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => __state_machine::SimulatedReduce::Accept,
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression6Parser {
        _priv: (),
    }

    impl Default for Expression6Parser { fn default() -> Self { Self::new() } }
    impl Expression6Parser {
        pub fn new() -> Expression6Parser {
            Expression6Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                // __Expression6 = Expression6 => ActionFn(4);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression6::Expression6Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression7 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        54, 35, 34, 33, 2, 0, -15, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        32, 35, 34, 59, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        54, 35, 34, 33, 2, 0, -17, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        32, 35, 34, 33, 2, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 4,
        // State 24
        0, 0, 0, 0, 0, 0, -33, -33, -33, 5, 6, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 27
        0, 0, 0, 0, 12, 13, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 36, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 28
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 30
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 31
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 32
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 33
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 34
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 35
        55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 7, 8, 9, 10, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -37, -37, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 12, 13, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 36, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 43
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 4,
        // State 44
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 4,
        // State 45
        0, 0, 0, 0, 0, 0, -29, -29, -29, 5, 6, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, -30, -30, -30, 5, 6, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -31, -31, -31, 5, 6, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, -32, -32, -32, 5, 6, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -28, -28, -28, 5, 6, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 54
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
        // State 57
        0, 0, 0, 65, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 66, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 59
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 60
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 62
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 63
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 64
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 7, 8, 9, 10, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -35, -35, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 7, 8, 9, 10, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -34, -34, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 7, 8, 9, 10, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -36, -36, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 18, 0, 0, 0, 0, 0, 0, 17, 16, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -24,
        // State 23
        -27,
        // State 24
        -33,
        // State 25
        -67,
        // State 26
        -56,
        // State 27
        -20,
        // State 28
        0,
        // State 29
        -13,
        // State 30
        -12,
        // State 31
        -57,
        // State 32
        -42,
        // State 33
        -41,
        // State 34
        -40,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -23,
        // State 43
        -25,
        // State 44
        -26,
        // State 45
        -29,
        // State 46
        -30,
        // State 47
        -31,
        // State 48
        -32,
        // State 49
        -28,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        -59,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        -58,
        // State 60
        0,
        // State 61
        -61,
        // State 62
        0,
        // State 63
        -60,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 13,
            3 => 55,
            5 => 20,
            6 => 50,
            7 => match state {
                1 => 36,
                12 => 56,
                19 => 72,
                _ => 51,
            },
            8 => 21,
            9 => match state {
                3 => 42,
                _ => 22,
            },
            10 => match state {
                4 => 43,
                5 => 44,
                _ => 23,
            },
            11 => match state {
                6 => 45,
                7 => 46,
                8 => 47,
                9 => 48,
                10 => 49,
                _ => 24,
            },
            12 => match state {
                0 => 25,
                15 => 67,
                16 => 68,
                17 => 69,
                _ => 37,
            },
            13 => match state {
                18 => 70,
                _ => 38,
            },
            14 => match state {
                14 => 66,
                _ => 39,
            },
            15 => 26,
            16 => match state {
                13 => 60,
                _ => 52,
            },
            19 => match state {
                2 => 41,
                _ => 27,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => __state_machine::SimulatedReduce::Accept,
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression7Parser {
        _priv: (),
    }

    impl Default for Expression7Parser { fn default() -> Self { Self::new() } }
    impl Expression7Parser {
        pub fn new() -> Expression7Parser {
            Expression7Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                // __Expression7 = Expression7 => ActionFn(5);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression7::Expression7Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression8 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        57, 36, 35, 34, 2, 0, -15, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        33, 36, 35, 62, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        57, 36, 35, 34, 2, 0, -17, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        33, 36, 35, 34, 2, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 4,
        // State 24
        0, 0, 0, 0, 0, 0, -33, -33, -33, 5, 6, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 7, 8, 9, 10, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -37, -37, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 13, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 28
        0, 0, 0, 0, 15, 16, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 37, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 29
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 31
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 32
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 33
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 34
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 35
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 36
        58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, -39, -39, 14, 0, 0, 0, 0, 0, 0, 13, 12, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 15, 16, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 37, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 43
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 4,
        // State 44
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 4,
        // State 45
        0, 0, 0, 0, 0, 0, -29, -29, -29, 5, 6, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, -30, -30, -30, 5, 6, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -31, -31, -31, 5, 6, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, -32, -32, -32, 5, 6, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -28, -28, -28, 5, 6, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 7, 8, 9, 10, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -35, -35, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 7, 8, 9, 10, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -34, -34, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 7, 8, 9, 10, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -36, -36, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 57
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 60
        0, 0, 0, 68, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 69, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 62
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 63
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 65
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 66
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 67
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, -18, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -38, -38, 14, 0, 0, 0, 0, 0, 0, 13, 12, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -24,
        // State 23
        -27,
        // State 24
        -33,
        // State 25
        -37,
        // State 26
        -68,
        // State 27
        -56,
        // State 28
        -20,
        // State 29
        0,
        // State 30
        -13,
        // State 31
        -12,
        // State 32
        -57,
        // State 33
        -42,
        // State 34
        -41,
        // State 35
        -40,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -23,
        // State 43
        -25,
        // State 44
        -26,
        // State 45
        -29,
        // State 46
        -30,
        // State 47
        -31,
        // State 48
        -32,
        // State 49
        -28,
        // State 50
        -35,
        // State 51
        -34,
        // State 52
        -36,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        -59,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        -58,
        // State 63
        0,
        // State 64
        -61,
        // State 65
        0,
        // State 66
        -60,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 16,
            3 => 58,
            5 => 20,
            6 => 53,
            7 => match state {
                1 => 37,
                15 => 59,
                19 => 72,
                _ => 54,
            },
            8 => 21,
            9 => match state {
                3 => 42,
                _ => 22,
            },
            10 => match state {
                4 => 43,
                5 => 44,
                _ => 23,
            },
            11 => match state {
                6 => 45,
                7 => 46,
                8 => 47,
                9 => 48,
                10 => 49,
                _ => 24,
            },
            12 => match state {
                11 => 50,
                12 => 51,
                13 => 52,
                _ => 25,
            },
            13 => match state {
                0 => 26,
                18 => 70,
                _ => 38,
            },
            14 => match state {
                17 => 69,
                _ => 39,
            },
            15 => 27,
            16 => match state {
                16 => 63,
                _ => 55,
            },
            19 => match state {
                2 => 41,
                _ => 28,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => __state_machine::SimulatedReduce::Accept,
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression8Parser {
        _priv: (),
    }

    impl Default for Expression8Parser { fn default() -> Self { Self::new() } }
    impl Expression8Parser {
        pub fn new() -> Expression8Parser {
            Expression8Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                // __Expression8 = Expression8 => ActionFn(6);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action6::<>(__sym0);
                return Some(Ok(__nt));
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression8::Expression8Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Expression9 {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 2
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 4
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 5
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 6
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 7
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 8
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 9
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 10
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 11
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 12
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 13
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 14
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 15
        58, 37, 36, 35, 2, 0, -15, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 16
        34, 37, 36, 63, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 17
        58, 37, 36, 35, 2, 0, -17, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 18
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 19
        34, 37, 36, 35, 2, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 20
        0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, -44, -44, 0, -44,
        // State 21
        0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, -22, 0, -22,
        // State 22
        0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, -24, 0, -24,
        // State 23
        0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, -27, -27, 0, 4,
        // State 24
        0, 0, 0, 0, 0, 0, -33, -33, -33, 5, 6, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, -33, -33, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, -37, -37, -37, 0, 0, 7, 8, 9, 10, -37, -37, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -37, -37, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, -39, -39, 14, 0, 0, 0, 0, 0, 0, 13, 12, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0,
        // State 28
        0, 0, 0, 0, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, -56, -56, 0, -56,
        // State 29
        0, 0, 0, 0, 16, 17, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 38, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, -20, -20, 0, -20,
        // State 30
        0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, -13, -13, 0, -13,
        // State 32
        0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, -12, -12, 0, -12,
        // State 33
        0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 34
        0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 35
        0, 0, 0, 0, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, -41, -41, 0, -41,
        // State 36
        0, 0, 0, 0, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, -40, -40, 0, -40,
        // State 37
        59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, -19, 0, 0,
        // State 40
        0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, -43, -43, 0, -43,
        // State 41
        0, 0, 0, 0, 16, 17, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 38, -21, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, -21, 0, -21,
        // State 42
        0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, -23, 0, -23,
        // State 43
        0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, -25, 0, 4,
        // State 44
        0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, -26, 0, 4,
        // State 45
        0, 0, 0, 0, 0, 0, -29, -29, -29, 5, 6, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, -29, -29, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, -30, -30, -30, 5, 6, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, -30, -30, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, -31, -31, -31, 5, 6, -31, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, -31, -31, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, -32, -32, -32, 5, 6, -32, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, -32, -32, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, -28, -28, -28, 5, 6, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, -28, -28, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, -35, -35, -35, 0, 0, 7, 8, 9, 10, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -35, -35, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, -34, -34, -34, 0, 0, 7, 8, 9, 10, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -34, -34, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, -36, -36, -36, 0, 0, 7, 8, 9, 10, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, -36, -36, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, -38, -38, 14, 0, 0, 0, 0, 0, 0, 13, 12, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, -38, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, -57, -57, -57, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57, 20, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, -57, -57, 0, -57,
        // State 58
        0, 0, 0, 0, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, -59, 0, 0, 0, 0, 0, 0, 0, 0, -59, 0, 0, 0, -59, -59, 0, -59,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 61
        0, 0, 0, 69, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, -42, -42, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, 0, 0, 70, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, -42, -42, 0, -42,
        // State 63
        0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58, 0, 0, 0, 0, 0, 0, 0, 0, -58, 0, 0, 0, -58, -58, 0, -58,
        // State 64
        0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, -61, 0, 0, 0, 0, 0, 0, 0, 0, -61, 0, 0, 0, -61, -61, 0, -61,
        // State 66
        -4, -4, -4, -4, -4, 0, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 67
        0, 0, 0, 0, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, -60, 0, 0, 0, 0, 0, 0, 0, 0, -60, 0, 0, 0, -60, -60, 0, -60,
        // State 68
        0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 74, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, -18, 0, 0,
        // State 71
        -5, -5, -5, -5, -5, 0, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 72
        0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0,
        // State 73
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
        -44,
        // State 21
        -22,
        // State 22
        -24,
        // State 23
        -27,
        // State 24
        -33,
        // State 25
        -37,
        // State 26
        -39,
        // State 27
        -69,
        // State 28
        -56,
        // State 29
        -20,
        // State 30
        0,
        // State 31
        -13,
        // State 32
        -12,
        // State 33
        -57,
        // State 34
        -42,
        // State 35
        -41,
        // State 36
        -40,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -43,
        // State 41
        -21,
        // State 42
        -23,
        // State 43
        -25,
        // State 44
        -26,
        // State 45
        -29,
        // State 46
        -30,
        // State 47
        -31,
        // State 48
        -32,
        // State 49
        -28,
        // State 50
        -35,
        // State 51
        -34,
        // State 52
        -36,
        // State 53
        -38,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -59,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        -58,
        // State 64
        0,
        // State 65
        -61,
        // State 66
        0,
        // State 67
        -60,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 17,
            3 => 59,
            5 => 20,
            6 => 54,
            7 => match state {
                1 => 38,
                16 => 60,
                19 => 72,
                _ => 55,
            },
            8 => 21,
            9 => match state {
                3 => 42,
                _ => 22,
            },
            10 => match state {
                4 => 43,
                5 => 44,
                _ => 23,
            },
            11 => match state {
                6 => 45,
                7 => 46,
                8 => 47,
                9 => 48,
                10 => 49,
                _ => 24,
            },
            12 => match state {
                11 => 50,
                12 => 51,
                13 => 52,
                _ => 25,
            },
            13 => match state {
                14 => 53,
                _ => 26,
            },
            14 => match state {
                0 => 27,
                18 => 70,
                _ => 39,
            },
            15 => 28,
            16 => match state {
                17 => 64,
                _ => 56,
            },
            19 => match state {
                2 => 41,
                _ => 29,
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
        type Location = Position;
        type Error = u32;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = Expression;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => __state_machine::SimulatedReduce::Accept,
            69 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 28,
                }
            }
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
                }
            }
            _ => panic!("invalid reduction index {__reduce_index}",)
        }
    }
    pub struct Expression9Parser {
        _priv: (),
    }

    impl Default for Expression9Parser { fn default() -> Self { Self::new() } }
    impl Expression9Parser {
        pub fn new() -> Expression9Parser {
            Expression9Parser {
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
        ) -> Result<Expression, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Expression,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                // __Expression9 = Expression9 => ActionFn(7);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action7::<>(__sym0);
                return Some(Ok(__nt));
            }
            69 => {
                __reduce69(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce69<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Statement = Statement => ActionFn(0);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action0::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 28)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Expression9::Expression9Parser;

#[rustfmt::skip]
#[allow(explicit_outlives_requirements, non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::needless_lifetimes, clippy::type_complexity, clippy::needless_return, clippy::too_many_arguments, clippy::match_single_binding)]
mod __parse__Statement {

    use super::super::parser::Statement;
    use super::super::ast::{Expression, Operator, ArraySubexpr};
    use super::super::lexer::StatementToken as Token;
    use super::super::lexer::StatementToken;
    use super::super::position::Position;
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
        -70,
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
        type Location = Position;
        type Error = u32;
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
            61 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 20,
                }
            }
            62 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 21,
                }
            }
            63 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 22,
                }
            }
            64 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 23,
                }
            }
            65 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 24,
                }
            }
            66 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 25,
                }
            }
            67 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 26,
                }
            }
            68 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 27,
                }
            }
            69 => __state_machine::SimulatedReduce::Accept,
            70 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 29,
                }
            }
            71 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 29,
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
        ) -> Result<Statement, __lalrpop_util::ParseError<Position, Token, u32>>
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
        __lookahead_start: Option<&Position>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> Option<Result<Statement,__lalrpop_util::ParseError<Position, Token, u32>>>
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
                __reduce61(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            65 => {
                __reduce65(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            66 => {
                __reduce66(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            67 => {
                __reduce67(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            68 => {
                __reduce68(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            69 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0;
                let __end = __sym0.2;
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            70 => {
                __reduce70(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
            }
            71 => {
                __reduce71(__lookahead_start, __symbols, core::marker::PhantomData::<()>)
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
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, (String, Expression), Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, ArraySubexpr, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Expression, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Option<String>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Statement, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, String, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Token, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, alloc::vec::Vec<(String, Expression)>, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>
    ) -> (Position, bool, Position)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __reduce0<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",") = Param, "," => ActionFn(63);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action63::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 0)
    }
    fn __reduce1<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* =  => ActionFn(61);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action61::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 1)
    }
    fn __reduce2<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")* = (<Param> ",")+ => ActionFn(62);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action62::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 1)
    }
    fn __reduce3<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = Param, "," => ActionFn(66);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action66::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 2)
    }
    fn __reduce4<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // (<Param> ",")+ = (<Param> ",")+, Param, "," => ActionFn(67);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action67::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 2)
    }
    fn __reduce5<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::", number => ActionFn(74);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action74::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 3)
    }
    fn __reduce6<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = number, "::" => ActionFn(75);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action75::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce7<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::", number => ActionFn(76);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action76::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 3)
    }
    fn __reduce8<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = "::" => ActionFn(77);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action77::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce9<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // ArraySubexpr = Expression => ActionFn(46);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action46::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 3)
    }
    fn __reduce10<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Bind = identifier => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action16::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant0(__nt), __end));
        (1, 4)
    }
    fn __reduce11<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "true" => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action54::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce12<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Boolean = "false" => ActionFn(55);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action55::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 5)
    }
    fn __reduce13<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = Param => ActionFn(70);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action70::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce14<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> =  => ActionFn(71);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action71::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 6)
    }
    fn __reduce15<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+, Param => ActionFn(72);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action72::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 6)
    }
    fn __reduce16<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Comma<Param> = (<Param> ",")+ => ActionFn(73);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action73::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 6)
    }
    fn __reduce17<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression, "or", Expression9 => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 7)
    }
    fn __reduce18<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression = Expression9 => ActionFn(38);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action38::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 7)
    }
    fn __reduce19<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression1 = Term => ActionFn(17);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action17::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 8)
    }
    fn __reduce20<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = "not", Term => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action18::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 9)
    }
    fn __reduce21<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression2 = Expression1 => ActionFn(19);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action19::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 9)
    }
    fn __reduce22<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression4, "is", Expression2 => ActionFn(20);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 10)
    }
    fn __reduce23<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression4 = Expression2 => ActionFn(21);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action21::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 10)
    }
    fn __reduce24<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "+", Expression4 => ActionFn(22);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action22::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce25<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression6, "-", Expression4 => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 11)
    }
    fn __reduce26<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression6 = Expression4 => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action24::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 11)
    }
    fn __reduce27<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "in", Expression6 => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce28<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<", Expression6 => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce29<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, "<=", Expression6 => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action27::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce30<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">", Expression6 => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce31<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression7, ">=", Expression6 => ActionFn(29);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 12)
    }
    fn __reduce32<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression7 = Expression6 => ActionFn(30);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action30::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    fn __reduce33<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "==", Expression7 => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce34<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "!=", Expression7 => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce35<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression8, "|", Expression7 => ActionFn(33);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 13)
    }
    fn __reduce36<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression8 = Expression7 => ActionFn(34);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action34::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 13)
    }
    fn __reduce37<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression9, "and", Expression8 => ActionFn(35);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action35::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 14)
    }
    fn __reduce38<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Expression9 = Expression8 => ActionFn(36);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action36::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 14)
    }
    fn __reduce39<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = string => ActionFn(49);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action49::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce40<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = sstring => ActionFn(50);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action50::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce41<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = number => ActionFn(51);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action51::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce42<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = "-", number => ActionFn(52);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action52::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 15)
    }
    fn __reduce43<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Literal = Boolean => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action53::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 15)
    }
    fn __reduce44<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = identifier, "=", Expression => ActionFn(47);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant7(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action47::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 16)
    }
    fn __reduce45<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param = Expression => ActionFn(48);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action48::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 16)
    }
    fn __reduce46<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? = Param => ActionFn(59);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action59::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 17)
    }
    fn __reduce47<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Param? =  => ActionFn(60);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action60::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 17)
    }
    fn __reduce48<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "if", Expression => ActionFn(9);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action9::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce49<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "for", Bind, "in", Expression => ActionFn(10);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce50<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endif" => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action11::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce51<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "endfor" => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action12::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce52<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "elif", Expression => ActionFn(13);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym1.2;
        let __nt = super::__action13::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 18)
    }
    fn __reduce53<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "else" => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action14::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 18)
    }
    fn __reduce54<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Statement = "set", Expression, "=", Expression => ActionFn(15);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant7(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (4, 18)
    }
    fn __reduce55<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(39);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action39::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce56<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = identifier => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action40::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 19)
    }
    fn __reduce57<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action41::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce58<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, ".", identifier => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym2.2;
        let __nt = super::__action42::<>(__sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 19)
    }
    fn __reduce59<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "[", ArraySubexpr, "]" => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action43::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce60<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = Term, "(", Comma<Param>, ")" => ActionFn(44);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym3.2;
        let __nt = super::__action44::<>(__sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 19)
    }
    fn __reduce61<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression = Expression => ActionFn(8);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 20)
    }
    fn __reduce62<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression1 = Expression1 => ActionFn(1);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 21)
    }
    fn __reduce63<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression2 = Expression2 => ActionFn(2);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 22)
    }
    fn __reduce64<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression4 = Expression4 => ActionFn(3);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action3::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 23)
    }
    fn __reduce65<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression6 = Expression6 => ActionFn(4);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 24)
    }
    fn __reduce66<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression7 = Expression7 => ActionFn(5);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 25)
    }
    fn __reduce67<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression8 = Expression8 => ActionFn(6);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action6::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 26)
    }
    fn __reduce68<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // __Expression9 = Expression9 => ActionFn(7);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action7::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 27)
    }
    fn __reduce70<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? = number => ActionFn(56);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0;
        let __end = __sym0.2;
        let __nt = super::__action56::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 29)
    }
    fn __reduce71<
    >(
        __lookahead_start: Option<&Position>,
        __symbols: &mut alloc::vec::Vec<(Position,__Symbol<>,Position)>,
        _: core::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // number? =  => ActionFn(57);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2)).unwrap_or_default();
        let __end = __start;
        let __nt = super::__action57::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (0, 29)
    }
}
#[allow(unused_imports)]
pub use self::__parse__Statement::StatementParser;

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action0<
>(
    (_, __0, _): (Position, Statement, Position),
) -> Statement
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action1<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action2<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action3<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action4<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action5<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action6<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action7<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action8<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action9<
>(
    (_, _, _): (Position, Token, Position),
    (_, condition, _): (Position, Expression, Position),
) -> Statement
{
    Statement::If { condition }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action10<
>(
    (_, _, _): (Position, Token, Position),
    (_, bind, _): (Position, String, Position),
    (_, _, _): (Position, Token, Position),
    (_, expression, _): (Position, Expression, Position),
) -> Statement
{
    Statement::For { bind, iter: expression }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action11<
>(
    (_, __0, _): (Position, Token, Position),
) -> Statement
{
    Statement::EndIf
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action12<
>(
    (_, __0, _): (Position, Token, Position),
) -> Statement
{
    Statement::EndFor
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action13<
>(
    (_, _, _): (Position, Token, Position),
    (_, condition, _): (Position, Expression, Position),
) -> Statement
{
    Statement::Elif { condition }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action14<
>(
    (_, __0, _): (Position, Token, Position),
) -> Statement
{
    Statement::Else
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action15<
>(
    (_, _, _): (Position, Token, Position),
    (_, expr, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, value, _): (Position, Expression, Position),
) -> Statement
{
    Statement::Set { expr, value }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action16<
>(
    (_, __0, _): (Position, String, Position),
) -> String
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action17<
>(
    (_, e, _): (Position, Expression, Position),
) -> Expression
{
    e
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action18<
>(
    (_, _, _): (Position, Token, Position),
    (_, t, _): (Position, Expression, Position),
) -> Expression
{
    Expression::LogicalNot(Box::new(t))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action19<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action20<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::Is, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action21<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action22<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::Add, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action23<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::Sub, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action24<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action25<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::In, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action26<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::LesserThan, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action27<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::LesserEqual, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action28<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::GreaterThan, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action29<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::GreaterEqual, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action30<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action31<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::Equal, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action32<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::BinaryOperation { lhs: Box::new(lhs), op: Operator::NotEqual, rhs: Box::new(rhs) }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action33<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    Expression::Filter(Box::new(lhs), Box::new(rhs))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action34<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action35<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::LogicalAnd(Box::new(lhs), Box::new(rhs))
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action36<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action37<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, rhs, _): (Position, Expression, Position),
) -> Expression
{
    {
        Expression::LogicalOr(Box::new(lhs), Box::new(rhs))
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action38<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action39<
>(
    (_, __0, _): (Position, Expression, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action40<
>(
    (_, ident, _): (Position, String, Position),
) -> Expression
{
    Expression::Identifier(ident)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action41<
>(
    (_, _, _): (Position, Token, Position),
    (_, __0, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
) -> Expression
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action42<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, ident, _): (Position, String, Position),
) -> Expression
{
    Expression::MemberAccess(Box::new(lhs), ident)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action43<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, sub, _): (Position, ArraySubexpr, Position),
    (_, _, _): (Position, Token, Position),
) -> Expression
{
    Expression::ArraySubscript(Box::new(lhs), sub)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action44<
>(
    (_, lhs, _): (Position, Expression, Position),
    (_, _, _): (Position, Token, Position),
    (_, params, _): (Position, Vec<(String, Expression)>, Position),
    (_, _, _): (Position, Token, Position),
) -> Expression
{
    Expression::Call(Box::new(lhs), params)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action45<
>(
    (_, nstart, _): (Position, Option<String>, Position),
    (_, _, _): (Position, Token, Position),
    (_, nend, _): (Position, Option<String>, Position),
) -> ArraySubexpr
{
    ArraySubexpr::Range(nstart, nend)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action46<
>(
    (_, expr, _): (Position, Expression, Position),
) -> ArraySubexpr
{
    ArraySubexpr::Value(Box::new(expr))
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action47<
>(
    (_, ident, _): (Position, String, Position),
    (_, _, _): (Position, Token, Position),
    (_, e, _): (Position, Expression, Position),
) -> (String, Expression)
{
    (ident, e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action48<
>(
    (_, e, _): (Position, Expression, Position),
) -> (String, Expression)
{
    (String::new(), e)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action49<
>(
    (_, s, _): (Position, String, Position),
) -> Expression
{
    Expression::String(s)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action50<
>(
    (_, s, _): (Position, String, Position),
) -> Expression
{
    Expression::String(s)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action51<
>(
    (_, n, _): (Position, String, Position),
) -> Expression
{
    Expression::Number(n)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action52<
>(
    (_, _, _): (Position, Token, Position),
    (_, n, _): (Position, String, Position),
) -> Expression
{
    Expression::Number(n)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action53<
>(
    (_, b, _): (Position, bool, Position),
) -> Expression
{
    Expression::Boolean(b)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action54<
>(
    (_, __0, _): (Position, Token, Position),
) -> bool
{
    true
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action55<
>(
    (_, __0, _): (Position, Token, Position),
) -> bool
{
    false
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action56<
>(
    (_, __0, _): (Position, String, Position),
) -> Option<String>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action57<
>(
    __lookbehind: &Position,
    __lookahead: &Position,
) -> Option<String>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action58<
>(
    (_, mut v, _): (Position, alloc::vec::Vec<(String, Expression)>, Position),
    (_, e, _): (Position, Option<(String, Expression)>, Position),
) -> Vec<(String, Expression)>
{
    match e { // (2)
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action59<
>(
    (_, __0, _): (Position, (String, Expression), Position),
) -> Option<(String, Expression)>
{
    Some(__0)
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action60<
>(
    __lookbehind: &Position,
    __lookahead: &Position,
) -> Option<(String, Expression)>
{
    None
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action61<
>(
    __lookbehind: &Position,
    __lookahead: &Position,
) -> alloc::vec::Vec<(String, Expression)>
{
    alloc::vec![]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action62<
>(
    (_, v, _): (Position, alloc::vec::Vec<(String, Expression)>, Position),
) -> alloc::vec::Vec<(String, Expression)>
{
    v
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action63<
>(
    (_, __0, _): (Position, (String, Expression), Position),
    (_, _, _): (Position, Token, Position),
) -> (String, Expression)
{
    __0
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action64<
>(
    (_, __0, _): (Position, (String, Expression), Position),
) -> alloc::vec::Vec<(String, Expression)>
{
    alloc::vec![__0]
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes, clippy::just_underscores_and_digits)]
fn __action65<
>(
    (_, v, _): (Position, alloc::vec::Vec<(String, Expression)>, Position),
    (_, e, _): (Position, (String, Expression), Position),
) -> alloc::vec::Vec<(String, Expression)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action66<
>(
    __0: (Position, (String, Expression), Position),
    __1: (Position, Token, Position),
) -> alloc::vec::Vec<(String, Expression)>
{
    let __start0 = __0.0;
    let __end0 = __1.2;
    let __temp0 = __action63(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action67<
>(
    __0: (Position, alloc::vec::Vec<(String, Expression)>, Position),
    __1: (Position, (String, Expression), Position),
    __2: (Position, Token, Position),
) -> alloc::vec::Vec<(String, Expression)>
{
    let __start0 = __1.0;
    let __end0 = __2.2;
    let __temp0 = __action63(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action68<
>(
    __0: (Position, Option<(String, Expression)>, Position),
) -> Vec<(String, Expression)>
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __temp0 = __action61(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __temp0,
        __0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action69<
>(
    __0: (Position, alloc::vec::Vec<(String, Expression)>, Position),
    __1: (Position, Option<(String, Expression)>, Position),
) -> Vec<(String, Expression)>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action62(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __temp0,
        __1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action70<
>(
    __0: (Position, (String, Expression), Position),
) -> Vec<(String, Expression)>
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __temp0 = __action59(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action71<
>(
    __lookbehind: &Position,
    __lookahead: &Position,
) -> Vec<(String, Expression)>
{
    let __start0 = *__lookbehind;
    let __end0 = *__lookahead;
    let __temp0 = __action60(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action68(
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action72<
>(
    __0: (Position, alloc::vec::Vec<(String, Expression)>, Position),
    __1: (Position, (String, Expression), Position),
) -> Vec<(String, Expression)>
{
    let __start0 = __1.0;
    let __end0 = __1.2;
    let __temp0 = __action59(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action73<
>(
    __0: (Position, alloc::vec::Vec<(String, Expression)>, Position),
) -> Vec<(String, Expression)>
{
    let __start0 = __0.2;
    let __end0 = __0.2;
    let __temp0 = __action60(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action69(
        __0,
        __temp0,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action74<
>(
    __0: (Position, String, Position),
    __1: (Position, Token, Position),
    __2: (Position, String, Position),
) -> ArraySubexpr
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __start1 = __2.0;
    let __end1 = __2.2;
    let __temp0 = __action56(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action56(
        __2,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action45(
        __temp0,
        __1,
        __temp1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action75<
>(
    __0: (Position, String, Position),
    __1: (Position, Token, Position),
) -> ArraySubexpr
{
    let __start0 = __0.0;
    let __end0 = __0.2;
    let __start1 = __1.2;
    let __end1 = __1.2;
    let __temp0 = __action56(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action57(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action45(
        __temp0,
        __1,
        __temp1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action76<
>(
    __0: (Position, Token, Position),
    __1: (Position, String, Position),
) -> ArraySubexpr
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __start1 = __1.0;
    let __end1 = __1.2;
    let __temp0 = __action57(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action56(
        __1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action45(
        __temp0,
        __0,
        __temp1,
    )
}

#[allow(clippy::too_many_arguments, clippy::needless_lifetimes,
    clippy::just_underscores_and_digits)]
fn __action77<
>(
    __0: (Position, Token, Position),
) -> ArraySubexpr
{
    let __start0 = __0.0;
    let __end0 = __0.0;
    let __start1 = __0.2;
    let __end1 = __0.2;
    let __temp0 = __action57(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    let __temp1 = __action57(
        &__start1,
        &__end1,
    );
    let __temp1 = (__start1, __temp1, __end1);
    __action45(
        __temp0,
        __0,
        __temp1,
    )
}

#[allow(clippy::type_complexity, dead_code)]
pub trait __ToTriple<>
{
    fn to_triple(self) -> Result<(Position,Token,Position), __lalrpop_util::ParseError<Position, Token, u32>>;
}

impl<> __ToTriple<> for (Position, Token, Position)
{
    fn to_triple(self) -> Result<(Position,Token,Position), __lalrpop_util::ParseError<Position, Token, u32>> {
        Ok(self)
    }
}
impl<> __ToTriple<> for Result<(Position, Token, Position), u32>
{
    fn to_triple(self) -> Result<(Position,Token,Position), __lalrpop_util::ParseError<Position, Token, u32>> {
        self.map_err(|error| __lalrpop_util::ParseError::User { error })
    }
}
