interface Setter {
    fn set_val(key: u32, val: u32) -> (bool, Final);
}

program test.aleo : Setter {
    mapping store: u32 => u32;

    fn set_val(key: u32, val: u32) -> (bool, Final) {
        let f: Final = final {
            Mapping::set(store, key, val);
        };
        return (true, f);
    }

    fn main(target: field, k: u32, v: u32, extra: u32) -> (bool, Final) {
        let (ok, f): (bool, Final) = Setter@(target)/set_val(k, v);
        return (ok, final { finalize_main(f, extra); });
    }
}

final fn finalize_main(f: Final, extra: u32) {
    f.run();
    assert(extra > 0u32);
}
