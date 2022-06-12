/*
namespace: Compile
expectation: Pass
inputs:
 - pedersen.in: |
    [main]
    hash_input: [bool; 512] = [true; 512];

    [registers]
    r0: bool = false;
*/

circuit PedersenHash {
    parameters: [u32; 512];

    function new(parameters: [u32; 512]) -> Self {
        return Self { parameters: parameters };
    }

    function hash(const self, bits: [bool; 512]) -> u32 {
        let digest: u32 = 0;
        for i in 0..512 {
            let base = bits[i] ? self.parameters[i] : 0u32;
            digest += base;
        }
        return digest;
    }
}

// The 'pedersen_hash' main function.
function main(hash_input: [bool; 512]) -> bool {
    const parameters = [0u32; 512];
    const pedersen = PedersenHash::new(parameters);
    let res = pedersen.hash(hash_input);

    return res == 0u32;
}
