// Safe: effects before interaction, no state ops after.
program dep.aleo {
    mapping shared: u32 => u32;

    fn do_thing(a: u32) -> Final {
        return final { finalize_do_thing(a); };
    }
}

final fn finalize_do_thing(a: u32) {
    Mapping::set(shared, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::do_thing(a);
        return final { finalize_do_work(f, a); };
    }
}

final fn finalize_do_work(f: Final, a: u32) {
    Mapping::set(balances, 0u32, a);
    f.run();
}
