
program test.aleo {
    transition tester(hi: u8) -> u8 {
        let hi = 2u8;
        return hi;
    }

    transition main(y: bool) -> bool {
        return y;
    }}
