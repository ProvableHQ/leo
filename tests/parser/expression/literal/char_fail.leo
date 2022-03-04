/*
namespace: Token
expectation: Fail
*/

'\'

'a

''

'\x9A'
'\x7'
'\x7g'
'\xz'
'\x80'
'\xc1'
'\xc2'
'\xDF'
'\xC0'
'\xe0'
'\x9f'

'abcdefg'

'\t\t'
'\a'
'\z'
'\A'
'\Z'
'\1'
'\9'
'\*'
'\x'
'\u'

'\u{bbbbb}\u{aaaa}'
'\uz'
'\u1'
'\u123'
'\u{2764z'
'\u{276g}'
'\u00000000'
'\u01000000'
'\u9999999'
'\u{110000}'
'\u{af🦀'

'😭😂😘'
