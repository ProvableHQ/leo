interface Base {
    fn double_val(x: u64) -> u64;
}

interface Extended: Base {
    fn triple_val(x: u64) -> u64;
}

program test.aleo: Extended {
    fn double_val(x: u64) -> u64 {
        return x * 2u64;
    }

    fn triple_val(x: u64) -> u64 {
        return x * 3u64;
    }

    fn main(target: field, x: u64) -> u64 {
        return Base@(target)::double_val(x);
    }
}
