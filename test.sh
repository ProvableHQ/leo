export CLEAR_LEO_TEST_EXPECTATIONS=1
cargo test --package leo-parser --lib -- test::parser_tests --exact --nocapture
# cargo test --package leo-parser --lib -- test::parser_tests --exact --nocapture 2>&1 | grep -A 2 dbg
export CLEAR_LEO_TEST_EXPECTATIONS=