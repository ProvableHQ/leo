
program my.aleo {
    transition main() -> u32 {
        let a = 4u32;
        let b = 4u32;
        let c = 4u32;
        c = 5u32;

        if true {
            a = 3u32;
        } else {
            a = 6u32;
        }

        return a;
    }
}
