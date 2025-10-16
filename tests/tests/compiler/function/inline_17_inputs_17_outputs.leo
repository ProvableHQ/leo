// Inlines can have any number of inputs/outputs.
program test.aleo {
    inline baz(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) {
        return (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
    }

    transition baz_caller() -> u8 {
        let inputs = baz(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q) = inputs;
        return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q;
    }
}

