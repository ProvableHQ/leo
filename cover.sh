cargo clean
cd parser
export RUSTFLAGS=-Cinstrument-coverage
cargo build
export LLVM_PROFILE_FILE=../target/out/leo_coverage-%p-%m.profraw
cargo test -- test::parser_tests --exact --nocapture
grcov ../target/out -s . --binary-path ../target/debug/ -t html --branch --ignore-not-existing -o ../target/debug/coverage/
cd ..
rm default.profraw
export RUSTFLAGS=
export LLVM_PROFILE_FILE=