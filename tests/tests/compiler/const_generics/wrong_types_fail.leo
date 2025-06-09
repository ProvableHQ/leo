program test.aleo {
    transition main() {
        let x = foo::[5u16]();
        let y = foo::[]();
        let z = foo::[5u32, 5u32]();
    }

    inline foo::[N: u32]() -> u32 {
        return N;
    }
}
