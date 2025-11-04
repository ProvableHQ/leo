program test.aleo {
    // The maximum number of inputs to a `transition` is 16.
    transition bar(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> u8 {
        return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q;
    }

    // The maximum number of inputs to an `async transition` is 16.
    async transition baz(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> Future {
        return baz_finalize(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q);
    }

    // The maximum number of inputs to an `async function`  is 16.
    async function baz_finalize(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) {  }

    // The maximum number of arguments to a `function` is 16.
    function foo(
        a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8,
        i: u8, j: u8, k: u8, l: u8, m: u8, n: u8, o: u8, p: u8, q: u8
    ) -> u8 {
        return a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q;
    }
}

