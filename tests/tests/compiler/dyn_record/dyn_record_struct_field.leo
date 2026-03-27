struct Info {
    inner: u64,
}

program test.aleo {
    record Token {
        owner: address,
        data: Info,
    }

    fn main(t: Token) -> u64 {
        let d: dyn = t as dyn;
        let info: Info = d.data;
        return info.inner;
    }
}
