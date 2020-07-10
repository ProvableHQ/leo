macro_rules! test_constant {
    ($_type: ty, $gadget: ty) => {
        for _ in 0..10 {
            let r: $_type = rand::random();

            <$gadget>::constant(r);
        }
    };
}
