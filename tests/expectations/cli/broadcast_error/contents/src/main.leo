program broadcast_error.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @admin(address = "aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px")
    constructor() {}
}
