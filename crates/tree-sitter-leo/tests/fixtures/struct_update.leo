struct Point {
    x: u32,
    y: u32,
    z: u32,
}

program test.aleo {
    record Token {
        owner: address,
        amount: u64,
    }

    fn updates(base: Point, tok: Token, new_x: u32) -> Point {
        // Override one field, copy the rest from the base.
        let a: Point = Point { x: new_x, ..base };
        // Copy every field from the base.
        let b: Point = Point { ..base };
        // A base after several explicit fields.
        let c: Point = Point { x: 1u32, y: 2u32, ..base };
        // The base may be an arbitrary expression.
        let d: Point = Point { z: 9u32, ..(base) };
        // Records support the same syntax.
        let e: Token = Token { amount: 5u64, ..tok };
        return a;
    }

    @noupgrade
    constructor() {}
}
