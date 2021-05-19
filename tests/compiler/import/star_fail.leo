// namespace: Compile
// expectation: Fail
// input_file: input/dummy.in
// cwd: imports

// importing `*` from a directory is illegal
import bar.bat.*;

function main() {}
