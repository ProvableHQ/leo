/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
cwd: statement
*/
import foo.leo;

function main(a: u32) -> Foo {
    return Foo { a: a };
}