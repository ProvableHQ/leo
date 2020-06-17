circuit PedersenHash {
    parameters: group[512]

    static function new(parameters: group[512]) -> Self {
        return Self { parameters: parameters }
    }

    function hash(bits: bool[512]) -> group {
        let mut digest: group = 0;

        for i in 0..512 {
            let base = if bits[i] ? parameters[i] : 0;
            digest += base;
        }

        return digest
    }
}

function main(parameters: group[512]) -> group {
    let pedersen = PedersenHash::new(parameters);

    let input: bool[512] = [true; 512]; // use mock private key until `.bits()` function is implemented
    let output = pedersen.hash(input);

    return output
}