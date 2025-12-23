
program test.aleo {
    function f1(a: u8) -> u8 {
        return a * 100u8;
    }

    transition main(id_type: i8, s: u8) -> u8 {
        let f1: u8 = 1u8;
        return f1;
    }
}
