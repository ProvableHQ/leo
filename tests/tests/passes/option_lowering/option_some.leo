program test.aleo {
    transition main(x: u32, flag: bool) -> u32 {
        let result: u32? = 5u32;
        return result.unwrap();
    }
}

