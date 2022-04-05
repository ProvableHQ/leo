Copyright (C) 2019-2022 Aleo Systems Inc.
This file is part of the Leo library.

The Leo library is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

The Leo library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with the Leo library. If not, see <https://www.gnu.org/licenses/>.


--------


Lexical Grammar
---------------

<a name="character"></a>
```abnf
character = %x0-D7FF / %xE000-10FFFF   ; Unicode code points decoded from UTF-8
```

<a name="horizontal-tab"></a>
```abnf
horizontal-tab = %x9   ; <HT>
```

<a name="line-feed"></a>
```abnf
line-feed = %xA   ; <LF>
```

<a name="carriage-return"></a>
```abnf
carriage-return = %xD   ; <CR>
```

<a name="space"></a>
```abnf
space = %x20   ; <SP>
```

<a name="double-quote"></a>
```abnf
double-quote = %x22   ; "
```

<a name="single-quote"></a>
```abnf
single-quote = %x27   ; '
```

<a name="not-star"></a>
```abnf
not-star = %x0-29 / %x2B-D7FF / %xE000-10FFFF   ; anything but *
```

<a name="not-star-or-slash"></a>
```abnf
not-star-or-slash = %x0-29 / %x2B-2E / %x30-D7FF / %xE000-10FFFF
                    ; anything but * or /
```

<a name="not-line-feed-or-carriage-return"></a>
```abnf
not-line-feed-or-carriage-return = %x0-9 / %xB-C / %xE-D7FF / %xE000-10FFFF
                                   ; anything but <LF> or <CR>
```

<a name="not-double-quote-or-backslash"></a>
```abnf
not-double-quote-or-backslash = %x0-21 / %x23-5B / %x5D-D7FF / %xE000-10FFFF
                                ; anything but " or \
```

<a name="not-single-quote-or-backslash"></a>
```abnf
not-single-quote-or-backslash = %x0-26 / %x28-5B / %x5D-D7FF / %xE000-10FFFF
                                ; anything but ' or \
```

<a name="line-terminator"></a>
```abnf
line-terminator = line-feed / carriage-return / carriage-return line-feed
```

Go to: _[carriage-return](#user-content-carriage-return), [line-feed](#user-content-line-feed)_;


<a name="whitespace"></a>
```abnf
whitespace = space / horizontal-tab / line-terminator
```

Go to: _[horizontal-tab](#user-content-horizontal-tab), [line-terminator](#user-content-line-terminator), [space](#user-content-space)_;


<a name="comment"></a>
```abnf
comment = block-comment / end-of-line-comment
```

Go to: _[block-comment](#user-content-block-comment), [end-of-line-comment](#user-content-end-of-line-comment)_;


<a name="block-comment"></a>
```abnf
block-comment = "/*" rest-of-block-comment
```

Go to: _[rest-of-block-comment](#user-content-rest-of-block-comment)_;


<a name="rest-of-block-comment"></a>
```abnf
rest-of-block-comment = "*" rest-of-block-comment-after-star
                      / not-star rest-of-block-comment
```

Go to: _[not-star](#user-content-not-star), [rest-of-block-comment-after-star](#user-content-rest-of-block-comment-after-star), [rest-of-block-comment](#user-content-rest-of-block-comment)_;


<a name="rest-of-block-comment-after-star"></a>
```abnf
rest-of-block-comment-after-star = "/"
                                 / "*" rest-of-block-comment-after-star
                                 / not-star-or-slash rest-of-block-comment
```

Go to: _[not-star-or-slash](#user-content-not-star-or-slash), [rest-of-block-comment-after-star](#user-content-rest-of-block-comment-after-star), [rest-of-block-comment](#user-content-rest-of-block-comment)_;


<a name="end-of-line-comment"></a>
```abnf
end-of-line-comment = "//" *not-line-feed-or-carriage-return
```

<a name="keyword"></a>
```abnf
keyword = %s"address"
        / %s"bool"
        / %s"char"
        / %s"console"
        / %s"const"
        / %s"else"
        / %s"field"
        / %s"for"
        / %s"function"
        / %s"group"
        / %s"i8"
        / %s"i16"
        / %s"i32"
        / %s"i64"
        / %s"i128"
        / %s"if"
        / %s"in"
        / %s"let"
        / %s"return"
        / %s"u8"
        / %s"u16"
        / %s"u32"
        / %s"u64"
        / %s"u128"
```

<a name="uppercase-letter"></a>
```abnf
uppercase-letter = %x41-5A   ; A-Z
```

<a name="lowercase-letter"></a>
```abnf
lowercase-letter = %x61-7A   ; a-z
```

<a name="letter"></a>
```abnf
letter = uppercase-letter / lowercase-letter
```

Go to: _[lowercase-letter](#user-content-lowercase-letter), [uppercase-letter](#user-content-uppercase-letter)_;


<a name="decimal-digit"></a>
```abnf
decimal-digit = %x30-39   ; 0-9
```

<a name="octal-digit"></a>
```abnf
octal-digit = %x30-37   ; 0-7
```

<a name="hexadecimal-digit"></a>
```abnf
hexadecimal-digit = decimal-digit / "a" / "b" / "c" / "d" / "e" / "f"
```

Go to: _[decimal-digit](#user-content-decimal-digit)_;


<a name="identifier"></a>
```abnf
identifier = letter *( letter / decimal-digit / "_" )
             ; but not a keyword or a boolean literal or aleo1...
```

Go to: _[letter](#user-content-letter)_;


<a name="numeral"></a>
```abnf
numeral = 1*decimal-digit
```

<a name="unsigned-literal"></a>
```abnf
unsigned-literal = numeral ( %s"u8" / %s"u16" / %s"u32" / %s"u64" / %s"u128" )
```

Go to: _[numeral](#user-content-numeral)_;


<a name="signed-literal"></a>
```abnf
signed-literal = numeral ( %s"i8" / %s"i16" / %s"i32" / %s"i64" / %s"i128" )
```

Go to: _[numeral](#user-content-numeral)_;


<a name="field-literal"></a>
```abnf
field-literal = numeral %s"field"
```

Go to: _[numeral](#user-content-numeral)_;


<a name="product-group-literal"></a>
```abnf
product-group-literal = numeral %s"group"
```

Go to: _[numeral](#user-content-numeral)_;


<a name="boolean-literal"></a>
```abnf
boolean-literal = %s"true" / %s"false"
```

<a name="address-literal"></a>
```abnf
address-literal = %s"aleo1" 58( lowercase-letter / decimal-digit )
```

<a name="character-literal"></a>
```abnf
character-literal = single-quote character-literal-element single-quote
```

Go to: _[character-literal-element](#user-content-character-literal-element), [single-quote](#user-content-single-quote)_;


<a name="character-literal-element"></a>
```abnf
character-literal-element = not-single-quote-or-backslash
                          / simple-character-escape
                          / ascii-character-escape
                          / unicode-character-escape
```

Go to: _[ascii-character-escape](#user-content-ascii-character-escape), [not-single-quote-or-backslash](#user-content-not-single-quote-or-backslash), [simple-character-escape](#user-content-simple-character-escape), [unicode-character-escape](#user-content-unicode-character-escape)_;


<a name="single-quote-escape"></a>
```abnf
single-quote-escape = "\" single-quote   ; \'
```

Go to: _[single-quote](#user-content-single-quote)_;


<a name="double-quote-escape"></a>
```abnf
double-quote-escape = "\" double-quote   ; \"
```

Go to: _[double-quote](#user-content-double-quote)_;


<a name="backslash-escape"></a>
```abnf
backslash-escape = "\\"
```

<a name="line-feed-escape"></a>
```abnf
line-feed-escape = %s"\n"
```

<a name="carriage-return-escape"></a>
```abnf
carriage-return-escape = %s"\r"
```

<a name="horizontal-tab-escape"></a>
```abnf
horizontal-tab-escape = %s"\t"
```

<a name="null-character-escape"></a>
```abnf
null-character-escape = "\0"
```

<a name="simple-character-escape"></a>
```abnf
simple-character-escape = single-quote-escape
                        / double-quote-escape
                        / backslash-escape
                        / line-feed-escape
                        / carriage-return-escape
                        / horizontal-tab-escape
                        / null-character-escape
```

Go to: _[backslash-escape](#user-content-backslash-escape), [carriage-return-escape](#user-content-carriage-return-escape), [double-quote-escape](#user-content-double-quote-escape), [horizontal-tab-escape](#user-content-horizontal-tab-escape), [line-feed-escape](#user-content-line-feed-escape), [null-character-escape](#user-content-null-character-escape), [single-quote-escape](#user-content-single-quote-escape)_;


<a name="ascii-character-escape"></a>
```abnf
ascii-character-escape = %s"\x" octal-digit hexadecimal-digit
```

Go to: _[hexadecimal-digit](#user-content-hexadecimal-digit), [octal-digit](#user-content-octal-digit)_;


<a name="unicode-character-escape"></a>
```abnf
unicode-character-escape = %s"\u{" 1*6hexadecimal-digit "}"
```

<a name="string-literal"></a>
```abnf
string-literal = double-quote *string-literal-element double-quote
```

Go to: _[double-quote](#user-content-double-quote)_;


<a name="string-literal-element"></a>
```abnf
string-literal-element = not-double-quote-or-backslash
                       / simple-character-escape
                       / ascii-character-escape
                       / unicode-character-escape
```

Go to: _[ascii-character-escape](#user-content-ascii-character-escape), [not-double-quote-or-backslash](#user-content-not-double-quote-or-backslash), [simple-character-escape](#user-content-simple-character-escape), [unicode-character-escape](#user-content-unicode-character-escape)_;


<a name="atomic-literal"></a>
```abnf
atomic-literal = unsigned-literal
               / signed-literal
               / field-literal
               / product-group-literal
               / boolean-literal
               / address-literal
               / character-literal
               / string-literal
```

Go to: _[address-literal](#user-content-address-literal), [boolean-literal](#user-content-boolean-literal), [character-literal](#user-content-character-literal), [field-literal](#user-content-field-literal), [product-group-literal](#user-content-product-group-literal), [signed-literal](#user-content-signed-literal), [string-literal](#user-content-string-literal), [unsigned-literal](#user-content-unsigned-literal)_;


<a name="symbol"></a>
```abnf
symbol = "!" / "&&" / "||"
       / "==" / "!="
       / "<" / "<=" / ">" / ">="
       / "+" / "-" / "*" / "/" / "**"
       / "="
       / "(" / ")"
       / "{" / "}"
       / "," / "." / ".." / ";" / ":" / "?"
       / "->" / "_"
       / %s")group"
```

<a name="token"></a>
```abnf
token = keyword
      / identifier
      / atomic-literal
      / numeral
      / symbol
```

Go to: _[atomic-literal](#user-content-atomic-literal), [identifier](#user-content-identifier), [keyword](#user-content-keyword), [numeral](#user-content-numeral), [symbol](#user-content-symbol)_;


<a name="lexeme"></a>
```abnf
lexeme = token / comment / whitespace
```

Go to: _[comment](#user-content-comment), [token](#user-content-token), [whitespace](#user-content-whitespace)_;



--------


Syntactic Grammar
-----------------

<a name="unsigned-type"></a>
```abnf
unsigned-type = %s"u8" / %s"u16" / %s"u32" / %s"u64" / %s"u128"
```

<a name="signed-type"></a>
```abnf
signed-type = %s"i8" / %s"i16" / %s"i32" / %s"i64" / %s"i128"
```

<a name="integer-type"></a>
```abnf
integer-type = unsigned-type / signed-type
```

Go to: _[signed-type](#user-content-signed-type), [unsigned-type](#user-content-unsigned-type)_;


<a name="field-type"></a>
```abnf
field-type = %s"field"
```

<a name="group-type"></a>
```abnf
group-type = %s"group"
```

<a name="arithmetic-type"></a>
```abnf
arithmetic-type = integer-type / field-type / group-type
```

Go to: _[field-type](#user-content-field-type), [group-type](#user-content-group-type), [integer-type](#user-content-integer-type)_;


<a name="boolean-type"></a>
```abnf
boolean-type = %s"bool"
```

<a name="address-type"></a>
```abnf
address-type = %s"address"
```

<a name="character-type"></a>
```abnf
character-type = %s"char"
```

<a name="scalar-type"></a>
```abnf
scalar-type =  boolean-type / arithmetic-type / address-type / character-type
```

Go to: _[address-type](#user-content-address-type), [arithmetic-type](#user-content-arithmetic-type), [boolean-type](#user-content-boolean-type), [character-type](#user-content-character-type)_;


<a name="type"></a>
```abnf
type = scalar-type
```

Go to: _[scalar-type](#user-content-scalar-type)_;


<a name="group-coordinate"></a>
```abnf
group-coordinate = ( [ "-" ] numeral ) / "+" / "-" / "_"
```

Go to: _[numeral](#user-content-numeral)_;


<a name="affine-group-literal"></a>
```abnf
affine-group-literal = "(" group-coordinate "," group-coordinate %s")group"
```

Go to: _[group-coordinate](#user-content-group-coordinate)_;


<a name="literal"></a>
```abnf
literal = atomic-literal / affine-group-literal
```

Go to: _[affine-group-literal](#user-content-affine-group-literal), [atomic-literal](#user-content-atomic-literal)_;


<a name="group-literal"></a>
```abnf
group-literal = product-group-literal / affine-group-literal
```

Go to: _[affine-group-literal](#user-content-affine-group-literal), [product-group-literal](#user-content-product-group-literal)_;


<a name="primary-expression"></a>
```abnf
primary-expression = identifier
                   / literal
                   / "(" expression ")"
                   / identifier function-arguments
```

Go to: _[expression](#user-content-expression), [function-arguments](#user-content-function-arguments), [identifier](#user-content-identifier), [literal](#user-content-literal)_;


<a name="function-arguments"></a>
```abnf
function-arguments = "(" [ expression *( "," expression ) [ "," ] ] ")"
```

Go to: _[expression](#user-content-expression)_;


<a name="unary-expression"></a>
```abnf
unary-expression = primary-expression
                 / "!" unary-expression
                 / "-" unary-expression
```

Go to: _[primary-expression](#user-content-primary-expression), [unary-expression](#user-content-unary-expression)_;


<a name="exponential-expression"></a>
```abnf
exponential-expression = unary-expression
                       / unary-expression "**" exponential-expression
```

Go to: _[exponential-expression](#user-content-exponential-expression), [unary-expression](#user-content-unary-expression)_;


<a name="multiplicative-expression"></a>
```abnf
multiplicative-expression = exponential-expression
                          / multiplicative-expression "*" exponential-expression
                          / multiplicative-expression "/" exponential-expression
```

Go to: _[exponential-expression](#user-content-exponential-expression), [multiplicative-expression](#user-content-multiplicative-expression)_;


<a name="additive-expression"></a>
```abnf
additive-expression = multiplicative-expression
                    / additive-expression "+" multiplicative-expression
                    / additive-expression "-" multiplicative-expression
```

Go to: _[additive-expression](#user-content-additive-expression), [multiplicative-expression](#user-content-multiplicative-expression)_;


<a name="ordering-expression"></a>
```abnf
ordering-expression = additive-expression
                    / additive-expression "<" additive-expression
                    / additive-expression ">" additive-expression
                    / additive-expression "<=" additive-expression
                    / additive-expression ">=" additive-expression
```

Go to: _[additive-expression](#user-content-additive-expression)_;


<a name="equality-expression"></a>
```abnf
equality-expression = ordering-expression
                    / ordering-expression "==" ordering-expression
                    / ordering-expression "!=" ordering-expression
```

Go to: _[ordering-expression](#user-content-ordering-expression)_;


<a name="conjunctive-expression"></a>
```abnf
conjunctive-expression = equality-expression
                       / conjunctive-expression "&&" equality-expression
```

Go to: _[conjunctive-expression](#user-content-conjunctive-expression), [equality-expression](#user-content-equality-expression)_;


<a name="disjunctive-expression"></a>
```abnf
disjunctive-expression = conjunctive-expression
                       / disjunctive-expression "||" conjunctive-expression
```

Go to: _[conjunctive-expression](#user-content-conjunctive-expression), [disjunctive-expression](#user-content-disjunctive-expression)_;


<a name="conditional-expression"></a>
```abnf
conditional-expression = disjunctive-expression
                       / disjunctive-expression "?" expression ":" expression
```

Go to: _[disjunctive-expression](#user-content-disjunctive-expression), [expression](#user-content-expression)_;


<a name="expression"></a>
```abnf
expression = conditional-expression
```

Go to: _[conditional-expression](#user-content-conditional-expression)_;


<a name="statement"></a>
```abnf
statement = return-statement
          / variable-declaration
          / constant-declaration
          / conditional-statement
          / loop-statement
          / assignment-statement
          / console-statement
          / block
```

Go to: _[assignment-statement](#user-content-assignment-statement), [block](#user-content-block), [conditional-statement](#user-content-conditional-statement), [console-statement](#user-content-console-statement), [constant-declaration](#user-content-constant-declaration), [loop-statement](#user-content-loop-statement), [return-statement](#user-content-return-statement), [variable-declaration](#user-content-variable-declaration)_;


<a name="block"></a>
```abnf
block = "{" *statement "}"
```

<a name="return-statement"></a>
```abnf
return-statement = %s"return" expression ";"
```

Go to: _[expression](#user-content-expression)_;


<a name="variable-declaration"></a>
```abnf
variable-declaration = %s"let" identifier ":" type "=" expression ";"
```

Go to: _[expression](#user-content-expression), [identifier](#user-content-identifier), [type](#user-content-type)_;


<a name="constant-declaration"></a>
```abnf
constant-declaration = %s"const" identifier ":" type "=" expression ";"
```

Go to: _[expression](#user-content-expression), [identifier](#user-content-identifier), [type](#user-content-type)_;


<a name="branch"></a>
```abnf
branch = %s"if" expression block
```

Go to: _[block](#user-content-block), [expression](#user-content-expression)_;


<a name="conditional-statement"></a>
```abnf
conditional-statement = branch
                      / branch %s"else" block
                      / branch %s"else" conditional-statement
```

Go to: _[block](#user-content-block), [branch](#user-content-branch), [conditional-statement](#user-content-conditional-statement)_;


<a name="loop-statement"></a>
```abnf
loop-statement = %s"for" identifier ":" type
                 %s"in" expression ".." expression
                 block
```

Go to: _[block](#user-content-block), [expression](#user-content-expression), [identifier](#user-content-identifier), [type](#user-content-type)_;


<a name="assignment-operator"></a>
```abnf
assignment-operator = "="
```

<a name="assignment-statement"></a>
```abnf
assignment-statement = expression assignment-operator expression ";"
```

Go to: _[assignment-operator](#user-content-assignment-operator), [expression](#user-content-expression)_;


<a name="console-statement"></a>
```abnf
console-statement = %s"console" "." console-call ";"
```

Go to: _[console-call](#user-content-console-call)_;


<a name="console-call"></a>
```abnf
console-call = assert-call
             / print-call
```

Go to: _[assert-call](#user-content-assert-call), [print-call](#user-content-print-call)_;


<a name="assert-call"></a>
```abnf
assert-call = %s"assert" "(" expression ")"
```

Go to: _[expression](#user-content-expression)_;


<a name="print-function"></a>
```abnf
print-function = %s"error" / %s"log"
```

<a name="print-arguments"></a>
```abnf
print-arguments = "(" string-literal  *( "," expression ) [ "," ] ")"
```

Go to: _[string-literal](#user-content-string-literal)_;


<a name="print-call"></a>
```abnf
print-call = print-function print-arguments
```

Go to: _[print-arguments](#user-content-print-arguments), [print-function](#user-content-print-function)_;


<a name="function-declaration"></a>
```abnf
function-declaration = %s"function" identifier
                       "(" [ function-parameters ] ")" "->" type
                       block
```

Go to: _[block](#user-content-block), [function-parameters](#user-content-function-parameters), [identifier](#user-content-identifier), [type](#user-content-type)_;


<a name="function-parameters"></a>
```abnf
function-parameters = function-parameter *( "," function-parameter ) [ "," ]
```

Go to: _[function-parameter](#user-content-function-parameter)_;


<a name="function-parameter"></a>
```abnf
function-parameter = [ %s"const" ] identifier ":" type
```

Go to: _[identifier](#user-content-identifier), [type](#user-content-type)_;


<a name="declaration"></a>
```abnf
declaration = function-declaration
```

Go to: _[function-declaration](#user-content-function-declaration)_;


<a name="file"></a>
```abnf
file = *declaration
```


--------


Format String Grammar
---------------------

<a name="not-brace"></a>
```abnf
not-brace = %x0-7A / %x7C / %x7E-10FFFF
            ; codes permitted in string after escapes processed, except { or }
```

<a name="format-string-container"></a>
```abnf
format-string-container = "{}"
```

<a name="format-string-open-brace"></a>
```abnf
format-string-open-brace = "{{"
```

<a name="format-string-close-brace"></a>
```abnf
format-string-close-brace = "}}"
```

<a name="format-string-element"></a>
```abnf
format-string-element = not-brace
                      / format-string-container
                      / format-string-open-brace
                      / format-string-close-brace
```

Go to: _[format-string-close-brace](#user-content-format-string-close-brace), [format-string-container](#user-content-format-string-container), [format-string-open-brace](#user-content-format-string-open-brace), [not-brace](#user-content-not-brace)_;


<a name="format-string"></a>
```abnf
format-string = *format-string-element

