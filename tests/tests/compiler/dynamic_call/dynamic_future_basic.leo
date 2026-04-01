interface Greeter {
    fn greet(a: u32) -> (u32, Final);
}

program test.aleo: Greeter {
    mapping counts: u32 => u32;

    fn greet(a: u32) -> (u32, Final) {
        let f: Final = final { Mapping::set(counts, a, a); };
        return (a, f);
    }

    fn main(target: field, x: u32) -> (u32, Final) {
        let (val, f): (u32, Final) = Greeter@(target)::greet(x);
        return (
            val,
            final {
                finalize_main(f);
            },
        );
    }
}

final fn finalize_main(f: Final) {
    f.run();
}
