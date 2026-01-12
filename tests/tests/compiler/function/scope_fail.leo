inline foo(x: u32) -> field {
    return myGlobal;
}

program test.aleo {
    transition main() -> field {
        let myGlobal = 42field;
        let err = foo(3u32);

        return err;
    }
}
