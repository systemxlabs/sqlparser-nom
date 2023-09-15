use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub sql); // synthesized by LALRPOP

#[test]
fn test_create_table() {
}
