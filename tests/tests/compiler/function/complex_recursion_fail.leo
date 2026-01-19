
fn one(n: u8) -> u8 {
    return two(n);
}

fn two(n: u8) -> u8 {
    return three(n) + four(n);
}

fn three(n: u8) -> u8 {
    return one(n);
}

fn four(n: u8) -> u8 {
    return one(n);
}

program test.aleo {
    fn five(n: u8) -> u8 {
        return six(n);
    }

    fn six(n: u8) -> u8 {
        return seven(n) + eight(n);
    }

    fn seven(n: u8) -> u8 {
        return five(n);
    }

    fn eight(n: u8) -> u8 {
        return five(n);
    }
}
