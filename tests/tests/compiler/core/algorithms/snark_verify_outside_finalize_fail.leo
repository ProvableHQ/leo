/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    fn main(vk: [u8; 128], v: u8, inputs: [field; 4], proof: [u8; 256]) -> bool {
        return Snark::verify(vk, v, inputs, proof);
    }

    fn verify_batch_outside_finalize(
        vks: [[u8; 128]; 2],
        v: u8,
        batch_inputs: [[[field; 4]; 3]; 2],
        proof: [u8; 256],
    ) -> bool {
        return Snark::verify_batch(vks, v, batch_inputs, proof);
    }
}
