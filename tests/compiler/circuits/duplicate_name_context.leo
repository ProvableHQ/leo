/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

circuit Bar {
    b2: u32

    function add_five(z: u32) -> u32 {
        return z + 5u32;
    }
}

function main(y: bool) -> bool {
    const Bar = 66u32;
    const k1 = Bar { b2: 30u32 };
    const k2 = Bar::add_five(55u32);

    return y == true;
}
