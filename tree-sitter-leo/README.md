# tree-sitter-leo

Tree-sitter grammar for the [Leo programming language](https://github.com/ProvableLabs/leo).

Leo is a functional, statically-typed programming language built for writing private applications on the Aleo blockchain.

## Features

- Full parsing support for Leo language constructs
- Syntax highlighting queries for editors
- Tags queries for code navigation
- Locals queries for scope analysis

## Installation

### npm

```bash
npm install tree-sitter-leo
```

### From source

```bash
git clone https://github.com/ProvableLabs/leo
cd leo/tree-sitter-leo
npm install
npm run generate
```

## Usage

### Node.js

```javascript
const Parser = require('tree-sitter');
const Leo = require('tree-sitter-leo');

const parser = new Parser();
parser.setLanguage(Leo);

const sourceCode = `
program hello.aleo {
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
`;

const tree = parser.parse(sourceCode);
console.log(tree.rootNode.toString());
```

### Neovim

Add to your tree-sitter configuration:

```lua
require('nvim-treesitter.parsers').get_parser_configs().leo = {
  install_info = {
    url = "https://github.com/ProvableLabs/leo",
    files = { "tree-sitter-leo/src/parser.c" },
    location = "tree-sitter-leo",
  },
  filetype = "leo",
}
```

### Helix

Copy the queries to your Helix runtime directory:

```bash
mkdir -p ~/.config/helix/runtime/queries/leo
cp queries/*.scm ~/.config/helix/runtime/queries/leo/
```

Add to `languages.toml`:

```toml
[[language]]
name = "leo"
scope = "source.leo"
injection-regex = "leo"
file-types = ["leo"]
roots = ["program.json"]
comment-token = "//"

[[grammar]]
name = "leo"
source = { git = "https://github.com/ProvableLabs/leo", subpath = "tree-sitter-leo" }
```

## Language Overview

Leo supports the following constructs:

### Types

- Primitive types: `bool`, `u8`-`u128`, `i8`-`i128`, `field`, `group`, `scalar`, `address`, `signature`, `string`
- Composite types: `struct`, `record`
- Collection types: arrays `[T; N]`, tuples `(T1, T2, ...)`
- Optional types: `T?`
- Future type: `Future`

### Declarations

- Programs: `program name.aleo { ... }`
- Functions: `function`, `transition`, `inline`, `script`
- Async functions: `async function`, `async transition`
- Structs: `struct Name { field: Type, ... }`
- Records: `record Name { owner: address, ... }`
- Mappings: `mapping name: KeyType => ValueType;`
- Constants: `const NAME: Type = value;`

### Statements

- Variable definitions: `let x: Type = value;`
- Assignments: `x = value;`, `x += value;`, etc.
- Conditionals: `if condition { ... } else { ... }`
- Loops: `for i: u32 in 0u32..10u32 { ... }`
- Assertions: `assert(condition);`, `assert_eq(a, b);`, `assert_neq(a, b);`
- Return: `return value;`

### Expressions

- Literals: `42u32`, `true`, `"string"`, `aleo1...`
- Binary operations: `+`, `-`, `*`, `/`, `%`, `**`, `<<`, `>>`, `&`, `|`, `^`, `&&`, `||`, `==`, `!=`, `<`, `>`, `<=`, `>=`
- Unary operations: `!`, `-`
- Ternary: `condition ? a : b`
- Type casting: `value as Type`
- Member access: `obj.field`
- Method calls: `obj.method(args)`
- Array access: `arr[index]`
- Tuple access: `tuple.0`
- Struct initialization: `StructName { field: value }`
- Function calls: `func(args)`
- Associated function calls: `Type::method(args)`

## Development

### Generate the parser

```bash
npm run generate
```

### Run tests

```bash
npm test
```

### Parse a file

```bash
npx tree-sitter parse path/to/file.leo
```

## License

GPL-3.0 - See [LICENSE](../LICENSE.md) for details.
