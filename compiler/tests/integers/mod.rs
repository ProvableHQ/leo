pub mod u32;

// use crate::compile_program;
//
// use leo_compiler::{
//     ConstrainedValue,
//     compiler::Compiler,
//     errors::{
//         CompilerError,
//         FunctionError,
//         StatementError
//     }
// };
// use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
// use snarkos_models::gadgets::{
//     r1cs::TestConstraintSystem
// };
// use snarkos_models::gadgets::utilities::boolean::Boolean;
// use leo_compiler::errors::{ExpressionError, BooleanError};
//
// const DIRECTORY_NAME: &str = "tests/integers/";
//
// fn get_output(program: Compiler<Fr, EdwardsProjective>) -> ConstrainedValue<Fr, EdwardsProjective>{
//     let mut cs = TestConstraintSystem::<Fr>::new();
//     let output = program.compile_constraints(&mut cs).unwrap();
//     assert!(cs.is_satisfied());
//     output
// }
//
// fn output_true(program: Compiler<Fr, EdwardsProjective>) {
//     let output = get_output(program);
//     assert_eq!(
//         ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Boolean(
//             Boolean::Constant(true)
//         )]),
//         output
//     );
// }
//
// fn output_false(program: Compiler<Fr, EdwardsProjective>) {
//     let output = get_output(program);
//     assert_eq!(
//         ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Boolean(
//             Boolean::Constant(false)
//         )]),
//         output
//     );
// }
//
// fn get_error(program: Compiler<Fr, EdwardsProjective>) -> CompilerError {
//     let mut cs = TestConstraintSystem::<Fr>::new();
//     program.compile_constraints(&mut cs).unwrap_err()
// }
//
// fn fail_evaluate(program: Compiler<Fr, EdwardsProjective>) {
//     match get_error(program) {
//         CompilerError::FunctionError(
//             FunctionError::StatementError(
//                 StatementError::ExpressionError(
//                     ExpressionError::BooleanError(
//                         BooleanError::CannotEvaluate(_string))))) => {},
//         error => panic!("Expected evaluate error, got {}", error),
//     }
// }
//
// fn fail_enforce(program: Compiler<Fr, EdwardsProjective>) {
//     match get_error(program) {
//         CompilerError::FunctionError(
//             FunctionError::StatementError(
//                 StatementError::ExpressionError(
//                     ExpressionError::BooleanError(
//                         BooleanError::CannotEnforce(_string))))) => {},
//         error => panic!("Expected evaluate error, got {}", error),
//     }
// }
//
// #[test]
// fn test_true() {
//     let program = compile_program(DIRECTORY_NAME, "true.leo").unwrap();
//     output_true(program);
// }
//
// #[test]
// fn test_false() {
//     let program = compile_program(DIRECTORY_NAME, "false.leo").unwrap();
//     output_false(program);
// }
//
// // Boolean not !
//
// #[test]
// fn test_not_true() {
//     let program = compile_program(DIRECTORY_NAME, "not_true.leo").unwrap();
//     output_false(program);
// }
//
// #[test]
// fn test_not_false() {
//     let program = compile_program(DIRECTORY_NAME, "not_false.leo").unwrap();
//     output_true(program);
// }
//
// #[test]
// fn test_not_u32() {
//     let program = compile_program(DIRECTORY_NAME, "not_u32.leo").unwrap();
//     fail_evaluate(program);
// }
//
// // Boolean or ||
//
// #[test]
// fn test_true_or_true() {
//     let program = compile_program(DIRECTORY_NAME, "true_||_true.leo").unwrap();
//     output_true(program);
// }
//
// #[test]
// fn test_true_or_false() {
//     let program = compile_program(DIRECTORY_NAME, "true_||_false.leo").unwrap();
//     output_true(program);
// }
//
// #[test]
// fn test_false_or_false() {
//     let program = compile_program(DIRECTORY_NAME, "false_||_false.leo").unwrap();
//     output_false(program);
// }
//
// #[test]
// fn test_true_or_u32() {
//     let program = compile_program(DIRECTORY_NAME, "true_||_u32.leo").unwrap();
//     fail_enforce(program);
// }
//
// // Boolean and &&
//
// #[test]
// fn test_true_and_true() {
//     let program = compile_program(DIRECTORY_NAME, "true_&&_true.leo").unwrap();
//     output_true(program);
// }
//
// #[test]
// fn test_true_and_false() {
//     let program = compile_program(DIRECTORY_NAME, "true_&&_false.leo").unwrap();
//     output_false(program);
// }
//
// #[test]
// fn test_false_and_false() {
//     let program = compile_program(DIRECTORY_NAME, "false_&&_false.leo").unwrap();
//     output_false(program);
// }
//
// #[test]
// fn test_true_and_u32() {
//     let program = compile_program(DIRECTORY_NAME, "true_&&_u32.leo").unwrap();
//     fail_enforce(program);
// }
//
// // All
//
// #[test]
// fn test_all() {
//     let program = compile_program(DIRECTORY_NAME, "all.leo").unwrap();
//     output_false(program);
// }
//
//
