import core.unstable.blake2s.Blake2s;

function main(seed: [u8; 32], message: [u8; 32], expected: [u8; 32]) {
    const actual = Blake2s::hash(seed, message);

    console.assert(expected == actual);
}
