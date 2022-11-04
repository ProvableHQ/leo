program fibonacci.aleo {
    // This calculates the n-th fibonacci number (up to 64th)
    transition fibonacci(public n: u128) -> u128 {
        let f0: u128 = 0u128;
        let f1: u128 = 1u128;
        let c: u16 = 0u16;

        let z: u128 = reverse_bits(n);

        for i:u16 in 0u16..8u16 {
            if n > 0u128 {
                let f2i1: u128 = f1 * f1 + f0 * f0;
                let f2i: u128 =  f0 * (2u128 * f1 - f0);
                if z & 1u128.shl(c) == 0u128 {
                    f0 = f2i;
                    f1 = f2i1;
                } else {
                    f0 = f2i1;
                    f1 = f2i + f2i1;
                }
                c = c + 1u16;
                n = n >> 1u16;
            }
        }

        return f0;
    }

    function reverse_bits(n: u128) -> u128 {
        let reverse: u128 = 0u128;
 
        for i:u128 in 0u128..128u128 {
            if n > 0u128 {
                reverse = reverse << 1u16;
        
                if n & 1u128 == 1u128 {
                    reverse ^= 1u128;
                }
        
                n = n >> 1u16;
            }
        }

        return reverse;
    }
}
