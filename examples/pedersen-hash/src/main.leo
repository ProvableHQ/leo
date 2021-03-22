circuit PedersenHash {
    parameters: [group; 256],

    // Instantiates a Pedersen hash circuit
    function new(parameters: [group; 256]) -> Self {
        return Self { parameters: parameters }
    }

    function hash(self, bits: [bool; 256]) -> group {
        let mut digest: group = 0group;
        for i in 0..256 {
            if bits[i] {
                digest += self.parameters[i];
            }
        }
        return digest
    }
}

// The 'pedersen-hash' main function.
function main(hash_input: [bool; 256], const parameters: [group; 256]) -> group {
    let pedersen = PedersenHash::new(parameters);
    let output = pedersen.hash(hash_input);
    console.log("{}", output);
    return output
}

