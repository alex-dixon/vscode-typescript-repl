// TypeScript REPL

// "Hello, World!"
// Increment a list of numbers.

// Evaluate line 9 by:
// 1. Highlighting it.
// 2. Selecting "Evaluate" from the Command Palette (Cmd + P).
import * as R from 'ramda'

// You should see the result in the REPL output panel.
// => 
// {
//   F: [Function: F],
//   T: [Function: T],
//   __: { '@@functional/placeholder': true },
//   add: [Function: f2],
//   addIndex: [Function: f1],
//   addIndexRight: [Function: f1],
//   adjust: [Function: f3],
//   all: [Function: f2],
//   allPass: [Function: f1],
//   always: [Function: f1],
//   and: [Function: f2],
//   any: [Function: f2],
//   ...
// }

// Use R.range function to create a list of numbers from 0 to 4
R.range(0, 5)
// => [0, 1, 2, 3, 4]

// Increment each number in the list by 1.
R.map(R.inc, R.range(0, 5))
// => [1, 2, 3, 4, 5]


// Note: You can refer to previous outputs with $1, $2, $3, etc.
// @ts-ignore
// $1
// => [1, 2, 3, 4, 5]
// @ts-ignore
// R.map(R.inc, $1)
// => [2, 3, 4, 5, 6]


// Variables
//
// Define `numbers` as the result of the range function.
const numbers = R.range(0, 5)

// Evaluate `numbers`.
numbers
// => [0, 1, 2, 3, 4]

// Redefine numbers as the result of mapping the inc function over the range.
// @ts-ignore
const numbers = R.map(R.inc, R.range(0, 5))
// => undefined
numbers
// => [1, 2, 3, 4, 5]


// Exports
//
// Make numbers available in other files.
// @ts-ignore
export const numbers = R.range(0, 5)
// => { numbers: [ 0, 1, 2, 3, 4 ] }

/**
 * Open @link file://./another-file.ts and test importing `numbers`.
 */
// You should see [0, 1, 2, 3, 4]

// 
// Uncomment the following line and evaluate the import from another-file again.
// export const numbers = R.map(R.inc, R.range(0, 5))


// Types
//
// You can use TypeScript types in code.
// You can evaluate them safely. They produce an empty result in the REPL.
type MyType = Record<string, any>
// => undefined


// Async
//
// Async functions are supported with top level await
const someAsyncFn = async () => fetch('http://google.com')

await someAsyncFn().then(x=>x.text())
someAsyncFn()



