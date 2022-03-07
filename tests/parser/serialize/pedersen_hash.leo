/*
namespace: Serialize
expectation: Pass
*/

circuit PedersenHash {
    parameters: [group; 256];

    // Instantiates a Pedersen hash circuit
    function new(parameters: [group; 256]) -> Self {
        return Self { parameters: parameters };
    }

    function hash(self, bits: [bool; 256]) -> group {
        let digest: group = 0group;
        for i: group in 0..256 {
            if bits[i] {
                digest += self.parameters[i];
            }
        }
        return digest;
    }
}

// The 'pedersen-hash' main function.
function main(hash_input: [bool; 256], const parameters: [group; 256]) -> group {
    const pedersen: PerdesenHash = PedersenHash::new(parameters);
    return pedersen.hash(hash_input);
}

