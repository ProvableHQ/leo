Copyright (C) 2019-2021 Aleo Systems Inc.
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


Introduction
------------

This file contains an ABNF (Augmented Backus-Naur Form) grammar of Leo.
Background on ABNF is provided later in this file.

This grammar provides an official definition of the syntax of Leo
that is both human-readable and machine-readable.
It will be part of an upcoming Leo language reference.
It may also be used to generate parser tests at some point.

We are also using this grammar
as part of a mathematical formalization of the Leo language,
which we are developing in the ACL2 theorem prover
and which we plan to publish at some point.
In particular, we have used a formally verified parser of ABNF grammars
(at https://github.com/acl2/acl2/tree/master/books/kestrel/abnf;
also see the paper at https://www.kestrel.edu/people/coglio/vstte18.pdf)
to parse this grammar into a formal representation of the Leo concrete syntax
and to validate that the grammar satisfies certain consistency properties.


--------


Background on ABNF
------------------

ABNF is an Internet standard:
see RFC 5234 at https://www.rfc-editor.org/info/rfc5234
and RFC 7405 at https://www.rfc-editor.org/info/rfc7405.
It is used to specify the syntax of JSON, HTTP, and other standards.

ABNF adds conveniences and makes slight modifications
to Backus-Naur Form (BNF),
without going beyond context-free grammars.

Instead of BNF's angle-bracket notation for nonterminals,
ABNF uses case-insensitive names consisting of letters, digits, and dashes,
e.g. `HTTP-message` and `IPv6address`.
ABNF includes an angle-bracket notation for prose descriptions,
e.g. `<host, see [RFC3986], Section 3.2.2>`,
usable as last resort in the definiens of a nonterminal.

While BNF allows arbitrary terminals,
ABNF uses only natural numbers as terminals,
and denotes them via:
(i) binary, decimal, or hexadecimal sequences,
e.g. `%b1.11.1010`, `%d1.3.10`, and `%x.1.3.A`
all denote the sequence of terminals [1, 3, 10];
(ii) binary, decimal, or hexadecimal ranges,
e.g. `%x30-39` denotes any singleton sequence of terminals
[_n_] with 48 <= _n_ <= 57 (an ASCII digit);
(iii) case-sensitive ASCII strings,
e.g. `%s"Ab"` denotes the sequence of terminals [65, 98];
and (iv) case-insensitive ASCII strings,
e.g. `%i"ab"`, or just `"ab"`, denotes
any sequence of terminals among
[65, 66],
[65, 98],
[97, 66], and
[97, 98].
ABNF terminals in suitable sets represent ASCII or Unicode characters.

ABNF allows repetition prefixes `n*m`,
where `n` and `m` are natural numbers in decimal notation;
if absent,
`n` defaults to 0, and
`m` defaults to infinity.
For example,
`1*4HEXDIG` denotes one to four `HEXDIG`s,
`*3DIGIT` denotes up to three `DIGIT`s, and
`1*OCTET` denotes one or more `OCTET`s.
A single `n` prefix
abbreviates `n*n`,
e.g. `3DIGIT` denotes three `DIGIT`s.

Instead of BNF's `|`, ABNF uses `/` to separate alternatives.
Repetition prefixes have precedence over juxtapositions,
which have precedence over `/`.
Round brackets group things and override the aforementioned precedence rules,
e.g. `*(WSP / CRLF WSP)` denotes sequences of terminals
obtained by repeating, zero or more times,
either (i) a `WSP` or (ii) a `CRLF` followed by a `WSP`.
Square brackets also group things but make them optional,
e.g. `[":" port]` is equivalent to `0*1(":" port)`.

Instead of BNF's `::=`, ABNF uses `=` to define nonterminals,
and `=/` to incrementally add alternatives
to previously defined nonterminals.
For example, the rule `BIT = "0" / "1"`
is equivalent to `BIT = "0"` followed by `BIT =/ "1"`.

The syntax of ABNF itself is formally specified in ABNF
(in Section 4 of the aforementioned RFC 5234,
after the syntax and semantics of ABNF
are informally specified in natural language
(in Sections 1, 2, and 3 of the aforementioned RFC 5234).
The syntax rules of ABNF prescribe the ASCII codes allowed for
white space (spaces and horizontal tabs),
line endings (carriage returns followed by line feeds),
and comments (semicolons to line endings).


--------


Structure
---------

This ABNF grammar consists of two (sub-)grammars:
(i) a lexical grammar that describes how
sequence of characters are parsed into tokens, and
(ii) a syntactic grammar that describes how
tokens are parsed into expressions, statements, etc.
The adjectives 'lexical' and 'syntactic' are
the same ones used in the Java language reference,
for instance;
alternative terms may be used in other languages,
but the separation into these two components is quite common
(the situation is sometimes a bit more complex, with multiple passes,
e.g. Unicode escape processing in Java).

This separation enables
concerns of white space, line endings, etc.
to be handled by the lexical grammar,
with the syntactic grammar focused on the more important structure.
Handling both aspects in a single grammar may be unwieldy,
so having two grammars provides more clarity and readability.

ABNF is a context-free grammar notation, with no procedural interpretation.
The two grammars conceptually define two subsequent processing phases,
as detailed below.
However, a parser implementation does not need to perform
two strictly separate phases (in fact, it typically does not),
so long as it produces the same final result.

The grammar is accompanied by some extra-grammatical requirements,
which are not conveniently expressible in a context-free grammar like ABNF.
These requirements are needed to make the grammar unambiguous,
i.e. to ensure that, for each sequence of terminals,
there is exactly one parse tree for that sequence terminals
that satisfies not only the grammar rules
but also the extra-grammatical requirements.
These requirements are expressed as comments in this file.


--------


Operator Precedence
-------------------

We formulate the grammar rules for expressions
in a way that describes the relative precedence of operators,
as often done in language syntax specifications.

For instance, consider the rules



```
multiplicative-expression =
        exponential-expression
      / multiplicative-expression "*" exponential-expression
      / multiplicative-expression "/" exponential-expression
```



```
additive-expression =
        multiplicative-expression
      / additive-expression "+" multiplicative-expression
      / additive-expression "-" multiplicative-expression
```



These rules tell us
that the additive operators `+` and `-` have lower precedence
than the multiplicative operators `*` and `/`,
and that both the additive and multiplicative operators associate to the left.
This may be best understood via the examples given below.

According to the rules, the expression



```
x + y * z
```



can only be parsed as



```
  +
 / \
x   *
   / \
  y   z
```



and not as



```
    *
   / \
  +   z
 / \
x   y
```



because a multiplicative expression cannot have an additive expression
as first sub-expression, as it would in the second tree above.

Also according to the rules, the expression



```
x + y + z
```



can only be parsed as



```
    +
   / \
  +   z
 / \
x   y
```



and not as



```
  +
 / \
x   +
   / \
  y   z
```



because an additive expression cannot have an additive expression
as second sub-expression, as it would in the second tree above.


--------


Naming Convention
-----------------

This ABNF grammar uses nonterminal names
that consist of complete English words, separated by dashes,
and that describe the construct the way it is in English.
For instance, we use the name `conditional-statement`
to describe conditional statements.

At the same time, this grammar establishes
a precise and official nomenclature for the Leo constructs,
by way of the nonterminal names that define their syntax.
For instance, the rule



```
group-literal = product-group-literal
              / affine-group-literal
```



tells us that there are two kinds of group literals,
namely product group literals and affine group literals.
This is more precise than describing them as
integers (which are not really group elements per se),
or points (they are all points, just differently specified),
or being singletons vs. pairs (which is a bit generic).

The only exception to the nomenclature-establishing role of the grammar
is the fact that, as discussed above,
we write the grammar rules in a way that determines
the relative precedence and the associativity of expression operators,
and therefore we have rules like



```
unary-expression = primary-expression
                 / "!" unary-expression
                 / "-" unary-expression
```



In order to allow the recursion of the rule to stop,
we need to regard, in the grammar, a primary expression as a unary expression
(i.e. a primary expression is also a unary expression in the grammar;
but note that the opposite is not true).
However, this is just a grammatical artifact:
ontologically, a primary expression is not really a unary expression,
because a unary expression is one that consists of
a unary operator and an operand sub-expression.
These terminological exceptions should be easy to identify in the rules.


--------


Lexical Grammar
---------------

A Leo file is a finite sequence of Unicode characters,
represented as Unicode code points,
which are numbers in the range from 0 to 10FFFFh.
These are captured by the ABNF rule `character` below.

The lexical grammar defines how, at least conceptually,
the sequence of characters is turned into
a sequence of tokens, comments, and whitespaces:
these entities are all defined by the grammar rules below.

As stated, the lexical grammar alone is ambiguous.
For example, the sequence of characters `**` (i.e. two stars)
could be equally parsed as two `*` symbol tokens or one `**` symbol token
(see rule for `symbol` below).
As another example, the sequence or characters `<CR><LF>`
(i.e. carriage return followed by line feed)
could be equally parsed as two line terminators or one
(see rule for `newline`).

Thus, as often done in language syntax definitions,
the lexical grammar is disambiguated by
the extra-grammatical requirement that
the longest possible sequence of characters is always parsed.
This way, `**` must be parsed as one `**` symbol token,
and `<CR><LF>` must be parsed as one line terminator.

As mentioned above, a character is any Unicode code point.
This grammar does not say how those are encoded in files (e.g. UTF-8):
it starts with a decoded sequence of Unicode code points.
Note that we allow any value,
even though some values may not be used according to the Unicode standard.

<a name="character"></a>
```abnf
character = %x0-10FFFF   ; any Unicode code point
```

We give names to certain ASCII characters.

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

We give names to complements of certain ASCII characters.
These consist of all the Unicode characters except for one or two.

<a name="not-double-quote"></a>
```abnf
not-double-quote = %x0-22 / %x24-10FFFF   ; anything but "
```

<a name="not-star"></a>
```abnf
not-star = %x0-29 / %x2B-10FFFF   ; anything but *
```

<a name="not-line-feed-or-carriage-return"></a>
```abnf
not-line-feed-or-carriage-return = %x0-9 / %xB-C / %xE-10FFFF
                                   ; anything but <LF> or <CR>
```

<a name="not-double-quote-or-open-brace"></a>
```abnf
not-double-quote-or-open-brace = %x0-22 / %x24-7A / %x7C-10FFFF
                                 ; anything but " or {
```

<a name="not-double-quote-or-close-brace"></a>
```abnf
not-double-quote-or-close-brace = %x0-22 / %x24-7C / %x7E-10FFFF
                                  ; anything but " or }
```

<a name="not-star-or-slash"></a>
```abnf
not-star-or-slash = %x0-29 / %x2B-2E / %x30-10FFFF
                    ; anything but * or /
```

Lines in Leo may be terminated via
a single carriage return,
a line feed,
or a carriage return immediately followed by a line feed.
Note that the latter combination constitutes a single line terminator,
according to the extra-grammatical requirement of the longest sequence,
described above.

<a name="newline"></a>
```abnf
newline = line-feed / carriage-return / carriage-return line-feed
```

Go to: _[line-feed](#user-content-line-feed), [carriage-return](#user-content-carriage-return)_;


Line terminators form whitespace, along with spaces and horizontal tabs.

<a name="whitespace"></a>
```abnf
whitespace = space / horizontal-tab / newline
```

Go to: _[space](#user-content-space), [newline](#user-content-newline), [horizontal-tab](#user-content-horizontal-tab)_;


There are two kinds of comments in Leo, as in other languages.
One is block comments of the form `/* ... */`,
and the other is end-of-line comments of the form `// ...`.
The first kind start at `/*` and end at the first `*/`,
possibly spanning multiple (partial) lines;
these do no nest.
The second kind start at `//` and extend till the end of the line.
The rules about comments given below are similar to
the ones used in the Java language reference.

<a name="comment"></a>
```abnf
comment = block-comment / end-of-line-comment
```

Go to: _[end-of-line-comment](#user-content-end-of-line-comment), [block-comment](#user-content-block-comment)_;


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

Go to: _[rest-of-block-comment-after-star](#user-content-rest-of-block-comment-after-star), [not-star](#user-content-not-star), [rest-of-block-comment](#user-content-rest-of-block-comment)_;


<a name="rest-of-block-comment-after-star"></a>
```abnf
rest-of-block-comment-after-star = "/"
                                 / "*" rest-of-block-comment-after-star
                                 / not-star-or-slash rest-of-block-comment
```

Go to: _[not-star-or-slash](#user-content-not-star-or-slash), [rest-of-block-comment-after-star](#user-content-rest-of-block-comment-after-star), [rest-of-block-comment](#user-content-rest-of-block-comment)_;


<a name="end-of-line-comment"></a>
```abnf
end-of-line-comment = "//" *not-line-feed-or-carriage-return newline
```

Go to: _[newline](#user-content-newline)_;


Below are the keywords in the Leo language.
They cannot be used as identifiers.

<a name="keyword"></a>
```abnf
keyword = %s"address"
        / %s"as"
        / %s"bool"
        / %s"circuit"
        / %s"console"
        / %s"const"
        / %s"else"
        / %s"false"
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
        / %s"import"
        / %s"in"
        / %s"input"
        / %s"let"
        / %s"mut"
        / %s"return"
        / %s"Self"
        / %s"self"
        / %s"static"
        / %s"string"
        / %s"true"
        / %s"u8"
        / %s"u16"
        / %s"u32"
        / %s"u64"
        / %s"u128"
```

The following rules define (ASCII) digits
and (uppercase and lowercase) letters.

<a name="digit"></a>
```abnf
digit = %x30-39   ; 0-9
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

Go to: _[uppercase-letter](#user-content-uppercase-letter), [lowercase-letter](#user-content-lowercase-letter)_;


An identifier is a non-empty sequence of letters, digits, and underscores,
starting with a letter.
It must not be a keyword: this is an extra-grammatical requirement.
It must also not be or start with `aleo1`,
because that is used for address literals:
this is another extra-grammatical requirement.

<a name="identifier"></a>
```abnf
identifier = letter *( letter / digit / "_" )   ; but not a keyword or aleo1...
```

Go to: _[letter](#user-content-letter)_;


A package name consists of one or more segments separated by single dashes,
where each segment is a non-empty sequence of lowercase letters and digits.

<a name="package-name"></a>
```abnf
package-name = 1*( lowercase-letter / digit )
               *( "-" 1*( lowercase-letter / digit ) )
```

A format string is a sequence of characters, other than double quote,
surrounded by double quotes.
Within a format string, sub-strings `{}` are distinguished as containers
(these are the ones that may be matched with values
whose textual representation replaces the containers
in the printed string).

<a name="format-string-container"></a>
```abnf
format-string-container = "{}"
```

<a name="format-string-element"></a>
```abnf
format-string-element = not-double-quote-or-open-brace
                      / "{" not-double-quote-or-close-brace
                      / format-string-container
```

Go to: _[not-double-quote-or-close-brace](#user-content-not-double-quote-or-close-brace), [format-string-container](#user-content-format-string-container), [not-double-quote-or-open-brace](#user-content-not-double-quote-or-open-brace)_;


<a name="format-string"></a>
```abnf
format-string = double-quote *format-string-element double-quote
```

Go to: _[double-quote](#user-content-double-quote)_;


Annotations have names, which are identifiers immediately preceded by `@`.

<a name="annotation-name"></a>
```abnf
annotation-name = "@" identifier
```

Go to: _[identifier](#user-content-identifier)_;


A natural (number) is a sequence of one or more digits.
We allow leading zeros, e.g. `007`.

<a name="natural"></a>
```abnf
natural = 1*digit
```

An integer (number) is either a natural or its negation.
We allow leading zeros also in negative numbers, e.g. `-007`.

<a name="integer"></a>
```abnf
integer = [ "-" ] natural
```

Go to: _[natural](#user-content-natural)_;


An untyped literal is just an integer.

<a name="untyped-literal"></a>
```abnf
untyped-literal = integer
```

Go to: _[integer](#user-content-integer)_;


Unsigned literals are naturals followed by unsigned types.

<a name="unsigned-literal"></a>
```abnf
unsigned-literal = natural ( %s"u8" / %s"u16" / %s"u32" / %s"u64" / %s"u128" )
```

Go to: _[natural](#user-content-natural)_;


Signed literals are integers followed by signed types.

<a name="signed-literal"></a>
```abnf
signed-literal = integer ( %s"i8" / %s"i16" / %s"i32" / %s"i64" / %s"i128" )
```

Go to: _[integer](#user-content-integer)_;


Field literals are integers followed by the type of field elements.

<a name="field-literal"></a>
```abnf
field-literal = integer %s"field"
```

Go to: _[integer](#user-content-integer)_;


There are two kinds of group literals.
One is a single integer followed by the type of group elements,
which denotes the scalar product of the generator point by the integer.
The other kind is not a token because it allows some whitespace inside;
therefore, it is defined in the syntactic grammar.

<a name="product-group-literal"></a>
```abnf
product-group-literal = integer %s"group"
```

Go to: _[integer](#user-content-integer)_;


Boolean literals are the usual two.

<a name="boolean-literal"></a>
```abnf
boolean-literal = %s"true" / %s"false"
```

An address literal starts with `aleo1`
and continues with exactly 58 lowercase letters and digits.
Thus an address always consists of 63 characters.

<a name="address-literal"></a>
```abnf
address-literal = %s"aleo1" 58( lowercase-letter / digit )
```

The ones above are all the atomic literals
(in the sense that they are tokens, without whitespace allowed in them),
as defined by the following rule.

<a name="atomic-literal"></a>
```abnf
atomic-literal = untyped-literal
               / unsigned-literal
               / signed-literal
               / field-literal
               / product-group-literal
               / boolean-literal
               / address-literal
```

Go to: _[product-group-literal](#user-content-product-group-literal), [untyped-literal](#user-content-untyped-literal), [address-literal](#user-content-address-literal), [unsigned-literal](#user-content-unsigned-literal), [boolean-literal](#user-content-boolean-literal), [signed-literal](#user-content-signed-literal), [field-literal](#user-content-field-literal)_;


After defining the (mostly) alphanumeric tokens above,
it remains to define tokens for non-alphanumeric symbols such as `+` and `(`.
Different programming languages used different terminologies for these,
e.g. operators, separators, punctuators, etc.
Here we use `symbol`, for all of them.
We also include a token consisting of
a closing parenthesis `)` immediately followed by `group`:
as defined in the syntactic grammar,
this is the final part of an affine group literal;
even though it includes letters,
it seems appropriate to still consider it a symbol,
particularly since it starts with a proper symbol.

<a name="symbol"></a>
```abnf
symbol = "!" / "&&" / "||"
       / "==" / "!="
       / "<" / "<=" / ">" / ">="
       / "+" / "-" / "*" / "/" / "**"
       / "=" / "+=" / "-=" / "*=" / "/=" / "**="
       / "(" / ")"
       / "[" / "]"
       / "{" / "}"
       / "," / "." / ".." / "..." / ";" / ":" / "::" / "?"
       / "->" / "_"
       / %s")group"
```

Everything defined above, other than comments and whitespace,
is a token, as defined by the following rule.

<a name="token"></a>
```abnf
token = keyword
      / identifier
      / atomic-literal
      / package-name
      / format-string
      / annotation-name
      / symbol
```

Go to: _[package-name](#user-content-package-name), [format-string](#user-content-format-string), [annotation-name](#user-content-annotation-name), [identifier](#user-content-identifier), [keyword](#user-content-keyword), [symbol](#user-content-symbol), [atomic-literal](#user-content-atomic-literal)_;



--------


Syntactic Grammar
-----------------

The processing defined by the lexical grammar above
turns the initial sequence of characters
into a sequence of tokens, comments, and whitespaces.
The purpose of comments and whitespaces, from a syntactic point of view,
is just to separate tokens:
they are discarded, leaving a sequence of tokens.
The syntactic grammar describes how to turn
a sequence of tokens into concrete syntax trees.

There are unsigned and signed integer types, for five sizes.

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


The integer types, along with the field and group types,
for the arithmetic types, i.e. the ones that support arithmetic operations.

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

Go to: _[integer-type](#user-content-integer-type), [group-type](#user-content-group-type), [field-type](#user-content-field-type)_;


The arithmetic types, along with the boolean and address types,
form the scalar types, i.e. the ones whose values do not contain (sub-)values.

<a name="boolean-type"></a>
```abnf
boolean-type = %s"bool"
```

<a name="address-type"></a>
```abnf
address-type = %s"address"
```

<a name="scalar-type"></a>
```abnf
scalar-type =  boolean-type / arithmetic-type / address-type
```

Go to: _[address-type](#user-content-address-type), [arithmetic-type](#user-content-arithmetic-type), [boolean-type](#user-content-boolean-type)_;


Circuit types are denoted by identifiers and the keyword `Self`.
The latter is only allowed inside a circuit definition,
to denote the circuit being defined.

<a name="self-type"></a>
```abnf
self-type = %s"Self"
```

<a name="circuit-type"></a>
```abnf
circuit-type = identifier / self-type
```

Go to: _[identifier](#user-content-identifier), [self-type](#user-content-self-type)_;


A tuple type consists of zero, two, or more component types.

<a name="tuple-type"></a>
```abnf
tuple-type = "(" [ type 1*( "," type ) ] ")"
```

Go to: _[type](#user-content-type)_;


An array type consists of an element type
and an indication of dimensions.
There is either a single dimension,
or a tuple of one or more dimensions.

<a name="array-type"></a>
```abnf
array-type = "[" type ";" array-dimensions "]"
```

Go to: _[type](#user-content-type), [array-dimensions](#user-content-array-dimensions)_;


<a name="array-dimensions"></a>
```abnf
array-dimensions = natural
                 / "(" natural *( "," natural ) ")"
```

Go to: _[natural](#user-content-natural)_;


Circuit, tuple, and array types form the aggregate types,
i.e. types whose values contain (sub-)values
(with the corner-case exception of the empty tuple value).

<a name="aggregate-type"></a>
```abnf
aggregate-type = tuple-type / array-type / circuit-type
```

Go to: _[circuit-type](#user-content-circuit-type), [array-type](#user-content-array-type), [tuple-type](#user-content-tuple-type)_;


Scalar and aggregate types form all the types.

<a name="type"></a>
```abnf
type = scalar-type / aggregate-type
```

Go to: _[scalar-type](#user-content-scalar-type), [aggregate-type](#user-content-aggregate-type)_;


The lexical grammar given earlier defines product group literals.
The other kind of group literal is a pair of integer coordinates,
which are reduced modulo the prime to identify a point,
which must be on the elliptic curve.
It is also allowed to omit one coordinate (not both),
with an indication of how to fill in the missing coordinate
(i.e. sign high, sign low, or inferred).
This is an affine group literal,
because it consists of affine point coordinates.

<a name="group-coordinate"></a>
```abnf
group-coordinate = integer / "+" / "-" / "_"
```

Go to: _[integer](#user-content-integer)_;


<a name="affine-group-literal"></a>
```abnf
affine-group-literal = "(" group-coordinate "," group-coordinate %s")group"
```

Go to: _[group-coordinate](#user-content-group-coordinate)_;


A literal is either an atomic one or an affine group literal.

<a name="literal"></a>
```abnf
literal = atomic-literal / affine-group-literal
```

Go to: _[affine-group-literal](#user-content-affine-group-literal), [atomic-literal](#user-content-atomic-literal)_;


The following rule is not directly referenced in the rules for expressions
(which reference `literal` instead),
but it is useful to establish terminology:
a group literal is either a product group literal or an affine group literal.

<a name="group-literal"></a>
```abnf
group-literal = product-group-literal / affine-group-literal
```

Go to: _[product-group-literal](#user-content-product-group-literal), [affine-group-literal](#user-content-affine-group-literal)_;


As often done in grammatical language syntax specifications,
we define rules for different kinds of expressions,
which also defines the relative precedence
of operators and other expression constructs,
and the (left or right) associativity of binary operators.

The primary expressions are self-contained in a way,
i.e. they have clear delimitations:
Some consist of single tokens,
while others have explicit endings.
Primary expressions also include parenthesized expressions,
i.e. any expression may be turned into a primary one
by putting parentheses around it.

<a name="primary-expression"></a>
```abnf
primary-expression = identifier
                   / %s"self"
                   / %s"input"
                   / literal
                   / "(" expression ")"
                   / tuple-expression
                   / array-expression
                   / circuit-expression
```

Go to: _[circuit-expression](#user-content-circuit-expression), [expression](#user-content-expression), [literal](#user-content-literal), [tuple-expression](#user-content-tuple-expression), [identifier](#user-content-identifier), [array-expression](#user-content-array-expression)_;


Tuple expressions construct tuples.
Each consists of zero, two, or more component expressions.

<a name="tuple-construction"></a>
```abnf
tuple-construction = "(" [ expression 1*( "," expression ) ] ")"
```

Go to: _[expression](#user-content-expression)_;


<a name="tuple-expression"></a>
```abnf
tuple-expression = tuple-construction
```

Go to: _[tuple-construction](#user-content-tuple-construction)_;


Array expressions construct arrays.
There are two kinds:
one lists the element expressions (at least one),
including spreads (via `...`) which are arrays being spliced in;
the other repeats (the value of) a single expression
across one or more dimensions.

<a name="array-inline-construction"></a>
```abnf
array-inline-construction = "["
                            array-inline-element
                            *( "," array-inline-element )
                            "]"
```

Go to: _[array-inline-element](#user-content-array-inline-element)_;


<a name="array-inline-element"></a>
```abnf
array-inline-element = expression / "..." expression
```

Go to: _[expression](#user-content-expression)_;


<a name="array-repeat-construction"></a>
```abnf
array-repeat-construction = "[" expression ";" array-dimensions "]"
```

Go to: _[expression](#user-content-expression), [array-dimensions](#user-content-array-dimensions)_;


<a name="array-construction"></a>
```abnf
array-construction = array-inline-construction / array-repeat-construction
```

Go to: _[array-inline-construction](#user-content-array-inline-construction), [array-repeat-construction](#user-content-array-repeat-construction)_;


<a name="array-expression"></a>
```abnf
array-expression = array-construction
```

Go to: _[array-construction](#user-content-array-construction)_;


Circuit expressions construct circuit values.
Each lists values for all the member variables (in any order);
there must be at least one member variable.
A single identifier abbreviates
a pair consisting of the same identifier separated by colon;
note that, in the expansion, the left one denotes a member name,
while the right one denotes an expression (a variable),
so they are syntactically identical but semantically different.

<a name="circuit-construction"></a>
```abnf
circuit-construction = circuit-type "{"
                       circuit-inline-element
                       *( "," circuit-inline-element ) [ "," ]
                       "}"
```

Go to: _[circuit-type](#user-content-circuit-type), [circuit-inline-element](#user-content-circuit-inline-element)_;


<a name="circuit-inline-element"></a>
```abnf
circuit-inline-element = identifier ":" expression / identifier
```

Go to: _[identifier](#user-content-identifier), [expression](#user-content-expression)_;


<a name="circuit-expression"></a>
```abnf
circuit-expression = circuit-construction
```

Go to: _[circuit-construction](#user-content-circuit-construction)_;


After primary expressions, postfix expressions have highest precedence.
They apply to primary expressions, and recursively to postfix expressions.

There are postfix expressions to access parts of aggregate values.
A tuple access selects a component by index (zero-based).
There are two kinds of array accesses:
one selects a single element by index (zero-based);
the other selects a range via two indices,
the first inclusive and the second exclusive --
both are optional,
the first defaulting to 0 and the second to the array length.
A circuit access selects a member variable by name.

Function calls are also postfix expressions.
There are three kinds of function calls:
top-level function calls,
instance (i.e. non-static) member function calls, and
static member function calls.
What changes is the start, but they all end in an argument list.

<a name="function-arguments"></a>
```abnf
function-arguments = "(" [ expression *( "," expression ) ] ")"
```

Go to: _[expression](#user-content-expression)_;


<a name="postfix-expression"></a>
```abnf
postfix-expression = primary-expression
                   / postfix-expression "." natural
                   / postfix-expression "." identifier
                   / identifier function-arguments
                   / postfix-expression "." identifier function-arguments
                   / circuit-type "::" identifier function-arguments
                   / postfix-expression "[" expression "]"
                   / postfix-expression "[" [expression] ".." [expression] "]"
```

Go to: _[natural](#user-content-natural), [circuit-type](#user-content-circuit-type), [function-arguments](#user-content-function-arguments), [primary-expression](#user-content-primary-expression), [postfix-expression](#user-content-postfix-expression), [identifier](#user-content-identifier), [expression](#user-content-expression)_;


Unary operators have the highest operator precedence.
They apply to postfix expressions,
and recursively to unary expressions.

<a name="unary-expression"></a>
```abnf
unary-expression = postfix-expression
                 / "!" unary-expression
                 / "-" unary-expression
```

Go to: _[postfix-expression](#user-content-postfix-expression), [unary-expression](#user-content-unary-expression)_;


Next in the operator precedence is exponentiation,
following mathematical practice.
The current rule below makes exponentiation right-associative,
i.e. `a ** b ** c` must be parsed as `a ** (b ** c)`.

<a name="exponential-expression"></a>
```abnf
exponential-expression = unary-expression
                       / unary-expression "**" exponential-expression
```

Go to: _[unary-expression](#user-content-unary-expression), [exponential-expression](#user-content-exponential-expression)_;


Next in precedence come multiplication and division, both left-associative.

<a name="multiplicative-expression"></a>
```abnf
multiplicative-expression = exponential-expression
                          / multiplicative-expression "*" exponential-expression
                          / multiplicative-expression "/" exponential-expression
```

Go to: _[exponential-expression](#user-content-exponential-expression), [multiplicative-expression](#user-content-multiplicative-expression)_;


Then there are addition and subtraction, both left-assocative.

<a name="additive-expression"></a>
```abnf
additive-expression = multiplicative-expression
                    / additive-expression "+" multiplicative-expression
                    / additive-expression "-" multiplicative-expression
```

Go to: _[additive-expression](#user-content-additive-expression), [multiplicative-expression](#user-content-multiplicative-expression)_;


Next in the precedence order are ordering relations.
These are not associative, because they return boolean values.

<a name="ordering-expression"></a>
```abnf
ordering-expression = additive-expression
                    / additive-expression "<" additive-expression
                    / additive-expression ">" additive-expression
                    / additive-expression "<=" additive-expression
                    / additive-expression ">=" additive-expression
```

Go to: _[additive-expression](#user-content-additive-expression)_;


Equalities return booleans but may also operate on booleans;
the rule below makes them left-associative.

<a name="equality-expression"></a>
```abnf
equality-expression = ordering-expression
                    / equality-expression "==" ordering-expression
                    / equality-expression "!=" ordering-expression
```

Go to: _[equality-expression](#user-content-equality-expression), [ordering-expression](#user-content-ordering-expression)_;


Next come conjunctive expressions, left-associative.

<a name="conjunctive-expression"></a>
```abnf
conjunctive-expression = equality-expression
                       / conjunctive-expression "&&" equality-expression
```

Go to: _[equality-expression](#user-content-equality-expression), [conjunctive-expression](#user-content-conjunctive-expression)_;


Next come disjunctive expressions, left-associative.

<a name="disjunctive-expression"></a>
```abnf
disjunctive-expression = conjunctive-expression
                       / disjunctive-expression "||" conjunctive-expression
```

Go to: _[disjunctive-expression](#user-content-disjunctive-expression), [conjunctive-expression](#user-content-conjunctive-expression)_;


Finally we have conditional expressions.

<a name="conditional-expression"></a>
```abnf
conditional-expression = disjunctive-expression
                       / conditional-expression
                         "?" expression
                         ":" conditional-expression
```

Go to: _[expression](#user-content-expression), [disjunctive-expression](#user-content-disjunctive-expression), [conditional-expression](#user-content-conditional-expression)_;


Those above are all the expressions.
Recall that conditional expressions
may be disjunctive expressions,
which may be conjunctive expressions,
and so on all the way to primary expressions.

<a name="expression"></a>
```abnf
expression = conditional-expression
```

Go to: _[conditional-expression](#user-content-conditional-expression)_;


There are various kinds of statements, including blocks.
Blocks are possibly empty sequences of statements surrounded by curly braces.

<a name="statement"></a>
```abnf
statement = expression-statement
          / return-statement
          / variable-declaration
          / constant-declaration
          / conditional-statement
          / loop-statement
          / assignment-statement
          / console-statement
          / block
```

Go to: _[return-statement](#user-content-return-statement), [block](#user-content-block), [expression-statement](#user-content-expression-statement), [assignment-statement](#user-content-assignment-statement), [constant-declaration](#user-content-constant-declaration), [conditional-statement](#user-content-conditional-statement), [console-statement](#user-content-console-statement), [variable-declaration](#user-content-variable-declaration), [loop-statement](#user-content-loop-statement)_;


<a name="block"></a>
```abnf
block = "{" *statement "}"
```

An expression (that must return the empty tuple, as semantically required)
can be turned into a statement by appending a semicolon.

<a name="expression-statement"></a>
```abnf
expression-statement = expression ";"
```

Go to: _[expression](#user-content-expression)_;


A return statement always takes an expression, and ends with a semicolon.

<a name="return-statement"></a>
```abnf
return-statement = %s"return" expression ";"
```

Go to: _[expression](#user-content-expression)_;


There are variable declarations and constant declarations,
which only differ in the starting keyword.
These declarations are also statements.
The names of the variables or constants are
either a single one or a tuple of two or more;
in all cases, there is just one optional type
and just one initializing expression.

<a name="variable-declaration"></a>
```abnf
variable-declaration = %s"let" identifier-or-identifiers [ ":" type ]
                       "=" expression ";"
```

Go to: _[identifier-or-identifiers](#user-content-identifier-or-identifiers), [type](#user-content-type), [expression](#user-content-expression)_;


<a name="constant-declaration"></a>
```abnf
constant-declaration = %s"const" identifier-or-identifiers [ ":" type ]
                       "=" expression ";"
```

Go to: _[type](#user-content-type), [expression](#user-content-expression), [identifier-or-identifiers](#user-content-identifier-or-identifiers)_;


<a name="identifier-or-identifiers"></a>
```abnf
identifier-or-identifiers = identifier
                          / "(" identifier 1*( "," identifier ) ")"
```

Go to: _[identifier](#user-content-identifier)_;


A conditional statement always starts with a condition and a block
(which together form a branch).
It may stop there, or it may continue with an alternative block,
or possibly with another conditional statement, forming a chain.
Note that blocks are required in all branches, not merely statements.

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

Go to: _[conditional-statement](#user-content-conditional-statement), [block](#user-content-block), [branch](#user-content-branch)_;


A loop statement implicitly defines a loop variable
that goes from a starting value (inclusive) to an ending value (exclusive).
The body is a block.

<a name="loop-statement"></a>
```abnf
loop-statement = %s"for" identifier %s"in" expression ".." expression block
```

Go to: _[identifier](#user-content-identifier), [expression](#user-content-expression), [block](#user-content-block)_;


An assignment statement is straightforward.
Based on the operator, the assignment may be simple (i.e. `=`)
or compound (i.e. combining assignment with an arithmetic operation).

<a name="assignment-operator"></a>
```abnf
assignment-operator = "=" / "+=" / "-=" / "*=" / "/=" / "**="
```

<a name="assignment-statement"></a>
```abnf
assignment-statement = expression assignment-operator expression ";"
```

Go to: _[expression](#user-content-expression), [assignment-operator](#user-content-assignment-operator)_;


Console statements start with the `console` keyword,
followed by a console function call.
The call may be an assertion or a print command.
The former takes an expression (which must be boolean) as argument.
The latter takes either no argument,
or a format string followed by expressions,
whose number must match the number of containers `{}` in the format string.
Note that the console function names are identifiers, not keywords.
There are three kinds of print commands.

<a name="console-statement"></a>
```abnf
console-statement = %s"console" "." console-call
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
print-function = %s"debug" / %s"error" / %s"log"
```

<a name="print-arguments"></a>
```abnf
print-arguments = "(" [ format-string *( "," expression ) ] ")"
```

Go to: _[format-string](#user-content-format-string)_;


<a name="print-call"></a>
```abnf
print-call = print-function print-arguments
```

Go to: _[print-arguments](#user-content-print-arguments), [print-function](#user-content-print-function)_;


An annotation consists of an annotation name (which starts with `@`)
with optional annotation arguments, which are identifiers.
Note that no parentheses are used if there are no arguments.

<a name="annotation"></a>
```abnf
annotation = annotation-name
             [ "(" identifier *( "," identifier ) ")" ]
```

Go to: _[identifier](#user-content-identifier), [annotation-name](#user-content-annotation-name)_;


A function declaration defines a function.
The output type is optional, defaulting to the empty tuple type.
In general, a function input consists of an identifier and a type,
with an optional 'const' modifier.
Additionally, functions inside circuits
may start with a `mut self` or `const self` or `self` parameter.

<a name="function-declaration"></a>
```abnf
function-declaration = *annotation %s"function" identifier
                       "(" [ function-parameters ] ")" [ "->" type ]
                       block
```

Go to: _[block](#user-content-block), [identifier](#user-content-identifier), [function-parameters](#user-content-function-parameters), [type](#user-content-type)_;


<a name="function-parameters"></a>
```abnf
function-parameters = self-parameter
                    / self-parameter "," function-inputs
                    / function-inputs
```

Go to: _[self-parameter](#user-content-self-parameter), [function-inputs](#user-content-function-inputs)_;


<a name="self-parameter"></a>
```abnf
self-parameter = [ %s"mut" / %s"const" ] %s"self"
```

<a name="function-inputs"></a>
```abnf
function-inputs = function-input *( "," function-input )
```

Go to: _[function-input](#user-content-function-input)_;


<a name="function-input"></a>
```abnf
function-input = [ %s"const" ] identifier ":" type
```

Go to: _[identifier](#user-content-identifier), [type](#user-content-type)_;


A circuit member variable declaration consists of an identifier and a type.
A circuit member function declaration consists of a function declaration.

<a name="member-declaration"></a>
```abnf
member-declaration = member-variable-declaration
                   / member-function-declaration
```

Go to: _[member-function-declaration](#user-content-member-function-declaration), [member-variable-declaration](#user-content-member-variable-declaration)_;


<a name="member-variable-declaration"></a>
```abnf
member-variable-declaration = identifier ":" type
```

Go to: _[type](#user-content-type), [identifier](#user-content-identifier)_;


<a name="member-function-declaration"></a>
```abnf
member-function-declaration = function-declaration
```

Go to: _[function-declaration](#user-content-function-declaration)_;


A circuit declaration defines a circuit type,
as consisting of member variables and functions.

<a name="circuit-declaration"></a>
```abnf
circuit-declaration = *annotation %s"circuit" identifier
                      "{" member-declaration *( "," member-declaration ) "}"
```

Go to: _[member-declaration](#user-content-member-declaration), [identifier](#user-content-identifier)_;


An import declaration consists of the `import` keyword
followed by a package path, which may be one of the following:
a single wildcard;
an identifier, optionally followed by a local renamer;
a package name followed by a path, recursively;
or a parenthesized list of package paths,
which are "fan out" of the initial path.
Note that we allow the last element of the parenthesized list
to be followed by a comma, for convenience.

<a name="import-declaration"></a>
```abnf
import-declaration = %s"import" package-name "." package-path ";"
```

Go to: _[package-name](#user-content-package-name), [package-path](#user-content-package-path)_;


<a name="package-path"></a>
```abnf
package-path = "*"
             / identifier [ %s"as" identifier ]
             / package-name "." package-path
             / "(" package-path *( "," package-path ) [","] ")"
```

Go to: _[identifier](#user-content-identifier), [package-name](#user-content-package-name), [package-path](#user-content-package-path)_;


Finally, we define a file as a sequence of zero or more declarations.
We allow constant declarations at the top level, for global constants.
Currently variable declarations are disallowed at the top level.

<a name="declaration"></a>
```abnf
declaration = import-declaration
            / function-declaration
            / circuit-declaration
            / constant-declaration
```

Go to: _[circuit-declaration](#user-content-circuit-declaration), [constant-declaration](#user-content-constant-declaration), [import-declaration](#user-content-import-declaration), [function-declaration](#user-content-function-declaration)_;


<a name="file"></a>
```abnf
file = *declaration
```


--------


Format Note
-----------

The ABNF standard requires grammars
to consist of lines terminated by `<CR><LF>`
(i.e. carriage return followed by line feed, DOS/Windows-style),
as explained in the background on ABNF earlier in this file.
This file's lines are therefore terminated by `<CR><LF>`.
To avoid losing this requirement across systems,
this file is marked as `text eol=crlf` in `.gitattributes`:
this means that the file is textual, enabling visual diffs,
but its lines will always be terminated by `<CR><LF>` on any system.

Note that this `<CR><LF>` requirement only applies
to the grammar files themselves.
It does not apply to the lines of the languages described by the grammar.
ABNF grammars may describe any kind of languages,
with any kind of line terminators,
or even without line terminators at all (e.g. for "binary" languages).

