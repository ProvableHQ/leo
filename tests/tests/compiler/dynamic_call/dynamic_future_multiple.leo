interface Worker {
    fn work(a: u32) -> (u32, Final);
}

program test.aleo: Worker {
    mapping results: u32 => u32;

    fn work(a: u32) -> (u32, Final) {
        let f: Final = final { Mapping::set(results, a, a); };
        return (a, f);
    }

    fn main(target: field, x: u32, y: u32) -> (u32, Final) {
        let (val1, f1): (u32, Final) = Worker@(target)/work(x);
        let (val2, f2): (u32, Final) = Worker@(target)/work(y);
        return (
            val1 + val2,
            final {
                f1.run();
                f2.run();
            },
        );
    }
}

final fn finalize_main(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
