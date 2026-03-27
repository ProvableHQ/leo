interface Producer {
    fn produce(a: u32) -> (u32, Final);
}

program test.aleo: Producer {
    mapping store: u32 => u32;

    fn produce(a: u32) -> (u32, Final) {
        let f: Final = final { Mapping::set(store, a, a); };
        return (a, f);
    }

    fn main(target: field, x: u32) -> u32 {
        // Error: trying to use the result as a plain u32 when it includes a Final
        let val: u32 = Producer@(target)/produce(x);
        return val;
    }
}
