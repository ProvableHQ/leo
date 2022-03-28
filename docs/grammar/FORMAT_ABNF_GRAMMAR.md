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


Background on ABNF
------------------

ABNF is an Internet standard:
for more info please check out the
[README](./README.md) in this directory.


--------


Format String
-------------------

This ABNF grammar consists of one grammar
that describes how a Leo string-literal is parsed
for formatting.  Meaning, in this context
all characters are already parsed and we don't
have to process escapes.

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

<a name="format-string"></a>
```abnf
format-string = *format-string-element
```
