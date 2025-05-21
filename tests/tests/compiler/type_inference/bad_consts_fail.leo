program type_inference.aleo {
    struct S {
        x: u8,
        b: bool,
    }
    
    // These are bad
    const A: bool = 3u8;
    const B: [u8; 3] = 3u64;
    const C: (S, S) = (1u8, 3u8);
    const D: S = [1u8, 3u8];
    const E: (u8, u8) = S { x: 0u8, b: false };

    // These are okay
    const C_: (u8, u8) = (1u8, 3u8);
    const D_: [u8; 2] = [1u8, 3u8];
    const E_: S = S { x: 0u8, b: false };

    // These are bad
    const F: bool = E_.x + D_[0u8] + C_.0;
    const G = (0u8, (0u8, 0u8));

    transition main() {}
}
