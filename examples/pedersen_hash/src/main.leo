circuit PedersenHash {
    parameters: group[512]
    static function new(parameters: group[512]) -> Self {
        return Self { parameters: parameters }
    }
    function hash(bits: bool[512]) -> group {
        let mut digest: group = 0;
        for i in 0..512 {
            let base = if bits[i] ? parameters[i] : 0group;
            digest += base;
        }
        return digest
    }
}

// The 'pedersen_hash' main function.
function main() -> group {
    let parameters = [0group; 512];
    const pedersen = PedersenHash::new(parameters);
    let input: bool[512] = [true; 512];
    return pedersen.hash(input)
}
