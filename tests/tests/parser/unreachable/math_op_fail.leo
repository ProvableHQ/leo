/*
namespace: ParseStatement
expectation: Fail
*/

let x = a ; b;

let x = a import b;

let x = a , b;

let x = a [ b;

let x = a ] b;

let x = a { b;

let x = a } b;

let x = a ( b;

let x = a ) b;

let x = a : b;

let x = a ? b;

let x = a _ b;

let x = a = b;

let x = a ! b;

let x = a .. b;

let x = a console b;

let x = a const b;

let x = a let b;

let x = a for b;

let x = a if b;

let x = a else b;

let x = a i8 b;

let x = a i16 b;

let x = a i32 b;

let x = a i64 b;

let x = a i128 b;

let x = a u8 b;

let x = a u16 b;

let x = a u32 b;

let x = a u64 b;

let x = a u128 b;

let x = a & b;

let x = a return b;

let x = a self b;

let x = a Self b;

let x = a true b;

let x = a false b;

let x = a 0 b;


x;=b;

x.=b;

x,=b; // 43

x[=b;

x]=b;

x{=b;

x}=b;

x=(;

x=);

x=:;

x=::;

x?=b;

x!==b;

x>==b;

x<==b;

x..=b;
