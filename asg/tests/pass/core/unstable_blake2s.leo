import core.unstable.blake2s.Blake2s;

function main() {
    const seed: [u8; 32] = [0; 32];
    const message: [u8; 32] = [0; 32];

    const result = Blake2s::hash(seed, message);

    console.log("Result: {}", result);
}
