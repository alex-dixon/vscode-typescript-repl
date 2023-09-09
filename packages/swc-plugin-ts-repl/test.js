const assert = require("assert")
const swc = require("@swc/core")

const input = `import {abc as xyz} from 'foo'; 
    import foo from 'bar';
    import * as myns from 'other'
    console.log('hello world', xyz, foo, myns);`
swc
  .transform(input, {
    // Some options cannot be specified in .swcrc
    filename: "input.ts",
    sourceMaps: true,
    // Input files are treated as module by default.
    isModule: true,

    module: { type: "commonjs", importInterop: "node" },

    // All options below can be configured via .swcrc
    jsc: {
      // FIXME. drops
      parser: {
        syntax: "typescript",
      },
      experimental: {
        plugins: [["@fit2/swc-plugin-ts-repl", {}]],
      },

      target: "es2022",
      // transform: { },
    },
  })
  .then((output) => {
    console.log("OUTPUT >>")
    console.log(output.code) // transformed code
    console.log(">>")
    console.log(output.map) // source map (in string)
    const expected = `"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
const { abc: xyz  } = require("foo");
const foo = require("bar");
const myns = require("other");
console.log('hello world', xyz, foo, myns);
`
    try {
      assert(output.code === expected)
    } catch (e) {
      console.error("Expected:\n\n", expected)
      console.error(">>\n\n")
      console.error("Actual:\n\n", output.code)
      throw e
    }
  })
