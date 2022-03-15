/*
namespace: ParseExpression
expectation: Pass
*/

"string"

"another { } string"

"{ ] [ ; a"

"\u{FFA}"
"\u{afafa}"
"\u{afaf}"
"\u{afa}"
"\u{af}"
"\u{a}"

"\x0A"

"\x7F"

"aa \\ \" \' \n aa \t \r  \0"

"test 😒€"
"😭😂😘"

"✋🏿"
"🦀"
"￿"

"���"

"（>３<）三"
"b" // 0rphon "(⑅∫°ਊ°)∫"
"b" // 0rphon "🦀°1"