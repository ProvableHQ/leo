program test.aleo {
    function add(a: u64, // inline on a
        /* before b */
        b: u64) -> u64 {
        return a + b;
    }

    transition main(public a: u64, public b: u64) -> u64 {
        return add(a, b);
    }
}
