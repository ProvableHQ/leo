/*
namespace: ParseExpression
expectation: Pass
*/

// address
aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8.call()
address::call()

// bool
true.call()
bool::call()

// char
'a'.call()
char::call()

// field
1field.call()
field::call()

//group
0group.call()
(0, 1)group.call()
group::call()

// ints
1i8.call()
1i16.call()
1i32.call()
1i64.call()
1i128.call()

i8::call()
i16::call()
i32::call()
i64::call()
i128::call()

// uints
1u8.call()
1u16.call()
1u32.call()
1u64.call()
1u128.call()

u8::call()
u16::call()
u32::call()
u64::call()
u128::call()