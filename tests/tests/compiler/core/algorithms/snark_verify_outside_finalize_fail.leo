/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition main(vk: [u8; 128], v: u8, inputs: [field; 4], proof: [u8; 256]) -> bool {
        return Snark::verify(vk, v, inputs, proof);
    }
}
