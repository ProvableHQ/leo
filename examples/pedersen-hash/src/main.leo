circuit PedersenHash {
    parameters: [group; 256],

    // Instantiates a Pedersen hash circuit
    static function new(parameters: [group; 256]) -> Self {
        return Self { parameters: parameters }
    }

    function hash(bits: [bool; 256]) -> group {
        let mut digest: group = 0;
        for i in 0..256 {
            if bits[i] {
                digest += self.parameters[i];
            }
        }
        return digest
    }
}

// The 'pedersen-hash' main function.
function main() -> group {
    const parameters = [1group; 256];
    const pedersen = PedersenHash::new(parameters);
    let input: [bool; 256] = [true; 256];
    return pedersen.hash(input)
}
