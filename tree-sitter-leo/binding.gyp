{
  "targets": [
    {
      "target_name": "tree_sitter_leo_binding",
      "include_dirs": [
        "<!(node -e \"require('node-addon-api').include\")",
        "src"
      ],
      "sources": [
        "bindings/node/binding.cc",
        "src/parser.c"
      ],
      "cflags_c": [
        "-std=c11"
      ],
      "defines": [
        "NAPI_VERSION=6"
      ]
    }
  ]
}
