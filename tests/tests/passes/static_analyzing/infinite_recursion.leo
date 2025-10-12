program test.aleo {
    function loop_forever(x: u32) -> u32 {
        return loop_forever(x);
    }

    transition main() -> u32 {
        return loop_forever(5u32);
    }
}

