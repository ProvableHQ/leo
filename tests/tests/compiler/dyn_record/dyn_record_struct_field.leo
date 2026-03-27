struct Info {
    inner: u64,
}

program test.aleo {
    record Token {
        owner: address,
        data: Info,
    }

    fn main(t: Token) -> u64 {
        let d: dyn record = t as dyn record;
        let info: Info = d.data;
        return info.inner;
    }
}
