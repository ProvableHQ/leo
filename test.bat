set CLEAR_LEO_TEST_EXPECTATIONS=1
cargo test --package leo-parser --lib -- test::parser_tests --exact --nocapture
set CLEAR_LEO_TEST_EXPECTATIONS=