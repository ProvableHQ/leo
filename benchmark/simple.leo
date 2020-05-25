circuit PedersenHash {
    parameters: u32[512]
    static function new(parameters: u32[512]) -> Self {
        return Self { parameters: parameters }
    }
    function hash(bits: bool[512]) -> u32 {
        let mut digest: u32 = 0;
        for i in 0..512 {
            let base = if bits[i] ? parameters[i] : 0u32;
            digest += base;
        }
        return digest
    }
}
function main() -> u32 {
    let parameters = [0u32; 512];
    let pedersen = PedersenHash::new(parameters);
    let input: bool[512] = [true; 512];
    return pedersen.hash(input)
}