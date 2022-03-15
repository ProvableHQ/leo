/*
namespace: Token
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
"ヽ༼ ಠ益ಠ ༽ﾉ"
"(╯°□°）╯︵ ┻━┻"
"┬─┬ ノ( ゜-゜ノ)"
"( ͡° ͜ʖ ͡°)"
"b" // TODO reenabe once #1682 is closed "ᕙ(▀̿ĺ̯▀̿ ̿)ᕗ"
"♥╣[-_-]╠♥"
"b" // TODO reenabe once #1682 is closed "(⑅∫°ਊ°)∫"
"b" // TODO reenabe once #1682 is closed "🦀°1"