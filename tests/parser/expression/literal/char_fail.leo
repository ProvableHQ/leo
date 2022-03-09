/*
namespace: Token
expectation: Fail
*/

'\'
\
'\
\n

'a

''

'\x7'
'\xz'
'\x9A'
'\x7g'
'\x80'
'\xc1'
'\xc2'
'\xDF'
'\xC0'
'\xe0'
'\x9f'

'abcdefg'

'\a'
'\z'
'\A'
'\Z'
'\1'
'\9'
'\*'
'\x'
'\u'
'\t\t'

'\uz'
'\u1'
'' // 0rphon '\u};
'🦀\n'
'\u123'
'' //0rphon '🦀1🦀'
'' //0rphon '\u6🦀}'
'\u{af🦀'
'\u{2764z'
'\u{276g}'
'\u9999999'
'\u00000000'
'\u01000000'
'\u{110000}'
'\u{bbbbb}\u{aaaa}'

'😭😂😘'
