/*
namespace: ParseExpression
expectation: Pass
*/

[0u8; 1].len()

[0; 1].len()

[0; (1, 2)].len()

[0; (1, 2, 3)].len()

[[[0; 3]; 2]; 1].len()
