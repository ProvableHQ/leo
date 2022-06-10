// namespace: Compile
// expectation: Pass
// input_file: input/dummy.in
// cwd: imports

import lib.ello;

function main(y: bool) -> bool {
    let ello_str = ello();
    console.log("{}", ello_str);
    return ello_str == "ello" && y;
}