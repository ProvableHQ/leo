# tree-sitter-leo

Tree-sitter grammar for the [Leo programming language](https://github.com/ProvableLabs/leo).

Leo is a functional, statically-typed programming language built for writing private applications on the Aleo blockchain.

## Features

- Full parsing support for Leo language constructs
- Syntax highlighting queries for editors
- Tags queries for code navigation
- Locals queries for scope analysis
- Editor configurations for Neovim, Helix, and VS Code

## Installation

### Prerequisites

- Node.js 16+ or tree-sitter CLI
- A C compiler (for building the parser)

### From npm

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

## Editor Setup

### Neovim

See [editors/neovim/README.md](editors/neovim/README.md) for detailed instructions.

Quick setup with nvim-treesitter:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()
parser_config.leo = {
  install_info = {
    url = "https://github.com/ProvableLabs/leo",
    files = { "tree-sitter-leo/src/parser.c" },
    location = "tree-sitter-leo",
  },
  filetype = "leo",
}

vim.filetype.add({ extension = { leo = "leo" } })
```

Then run `:TSInstall leo`

### Helix

See [editors/helix/README.md](editors/helix/README.md) for detailed instructions.

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "leo"
scope = "source.leo"
file-types = ["leo"]
roots = ["program.json"]
comment-token = "//"

[[grammar]]
name = "leo"
source = { git = "https://github.com/ProvableLabs/leo", subpath = "tree-sitter-leo" }
```

Copy queries:

```bash
mkdir -p ~/.config/helix/runtime/queries/leo
cp queries/*.scm ~/.config/helix/runtime/queries/leo/
```

Build the grammar:

```bash
hx --grammar fetch && hx --grammar build
```

### VS Code

See [editors/vscode/README.md](editors/vscode/README.md) for detailed instructions.

A TextMate grammar is provided for VS Code since it doesn't natively support tree-sitter.

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

### CLI

```bash
# Parse a file
tree-sitter parse path/to/file.leo

# Run tests
tree-sitter test

# Highlight a file (requires queries)
tree-sitter highlight path/to/file.leo
```

## Highlighting Captures

The highlighting queries use the following capture groups:

| Capture | Description |
|---------|-------------|
| `@keyword` | Language keywords |
| `@keyword.function` | Function-related keywords |
| `@keyword.control` | Control flow keywords |
| `@keyword.modifier` | Visibility modifiers |
| `@type` | Type names |
| `@type.builtin` | Built-in types |
| `@function` | Function definitions |
| `@function.call` | Function calls |
| `@function.method` | Method calls |
| `@variable` | Variables |
| `@variable.parameter` | Function parameters |
| `@variable.builtin` | Built-in variables (self, block, network) |
| `@constant` | Constants |
| `@property` | Struct fields |
| `@string` | String literals |
| `@string.special` | Address literals |
| `@number` | Numeric literals |
| `@boolean` | Boolean literals |
| `@operator` | Operators |
| `@punctuation.bracket` | Brackets |
| `@punctuation.delimiter` | Delimiters |
| `@comment` | Comments |
| `@namespace` | Program IDs and locators |
| `@attribute` | Annotations |

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
# or
tree-sitter test
```

### Test highlighting

```bash
tree-sitter highlight --scope source.leo path/to/file.leo
```

### Parse a file

```bash
tree-sitter parse path/to/file.leo
```

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`npm test`)
2. New language features have corresponding test cases
3. Highlighting queries cover new syntax

## License

GPL-3.0 - See [LICENSE](../LICENSE.md) for details.
