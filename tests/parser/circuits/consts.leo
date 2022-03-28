/*
namespace: Parse
expectation: Pass
*/

circuit X {
    static const x: u32 = 2;
    static const y: u32 = 5;
    static const G: u8 = G;
    static const FOO: Foo = Foo {};
    static const INDEXED: Foo = A[0];
    static const TINDEXED: Foo = T.0;
    static const TWO: i8 = 1i8 + 1i8;
    static const mult: i8 = x * y;
    static const mult: i8 = one();
}