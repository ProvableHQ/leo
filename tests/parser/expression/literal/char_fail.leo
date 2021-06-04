/*
namespace: Token
expectation: Fail
*/

'\'

'a

''

'\x9'

'abcdefg'

'\t\t'

'\u{bbbbb}\u{aaaa}'

'\u123'

'\u1'        // should error
'\u00000000' // should error
'\u01000000' // should error
'\u9999999'  // should error

'😭😂😘'
