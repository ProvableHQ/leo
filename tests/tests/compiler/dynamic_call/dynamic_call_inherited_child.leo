interface Base {
    fn get_value() -> u64;
}

interface Extended: Base {
    fn double_val(x: u64) -> u64;
}

program test.aleo: Extended {
    fn get_value() -> u64 {
        return 0u64;
    }

    fn double_val(x: u64) -> u64 {
        return x * 2u64;
    }

    fn main(target: field, x: u64) -> u64 {
        return Extended@(target)::double_val(x);
    }
}
