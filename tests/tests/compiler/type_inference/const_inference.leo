program type_inference.aleo {
    struct S {
        x: u8,
        b: bool,
    }

    const A = 3u8;
    const B = 3u64;
    const C = (1u8, 3u8);
    const D = [1u8, 3u8];
    const E = S { x: 0u8, b: false };
    const F = E.x + D[0u8] + C.0;

    transition main() {
        assert(A == 3u8); 
        assert(B == 3u64); 
        assert(C.0 == 1u8); 
        assert(C.1 == 3u8); 
        assert(D[0u8] == 1u8); 
        assert(D[1u8] == 3u8); 
        assert(E.x == 0u8); 
        assert(E.b == false); 
        assert(F == E.x + D[0u8] + C.0); 
    }
}
