program test.aleo {
    // The maximum number of inputs to an entry point is 16.
    fn bar(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> u8 {
        return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q;
    }

    // The maximum number of inputs to an entry point is 16.
    fn baz(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> Final {
        // The maximum number of captures by a final block is 16.
        return final { baz_finalize(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q); };
    }
}

final fn baz_finalize(
    a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
    i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
) {  }

// The maximum number of arguments to a `function` is 16.
@no_inline
fn foo(
    a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
    i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
) -> u8 {
    return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q;
}

