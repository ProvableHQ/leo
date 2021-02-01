import core.unstable.blake2s.Blake2s;

function main(seed: [u8; 32], message: [u8; 32]) -> [u8; 32] {
    return Blake2s::hash(seed, message)
}
