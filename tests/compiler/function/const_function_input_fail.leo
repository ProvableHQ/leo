/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
state_file: input/dummy.state
*/

function main(y: bool) -> bool {
    return get_register();
}

const function get_register() -> bool {
    return input.record.is_dummy;
}
