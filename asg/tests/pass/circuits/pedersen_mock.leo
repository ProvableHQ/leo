circuit PedersenHash {
    parameters: [u32; 512]

    function new(parameters: [u32; 512]) -> Self {
        return Self { parameters: parameters }
    }

    function hash(self, bits: [bool; 512]) -> u32 {
        let mut digest: u32 = 0;
        for i in 0..512 {
            let base = if bits[i] ? self.parameters[i] : 0u32;
            digest += base;
        }
        return digest
    }
}

// The 'pedersen_hash' main function.
function main() {
    let parameters = [0u32; 512];
    let pedersen = PedersenHash::new(parameters);
    let hash_input: [bool; 512] = [true; 512];

    let res = pedersen.hash(hash_input);

    console.assert(res == 0u32);
}
