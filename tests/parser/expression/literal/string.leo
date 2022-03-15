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
"b" // TODO reenabe once #1682 is closed "(⑅∫°ਊ°)∫"
"b" // TODO reenabe once #1682 is closed "🦀°1"